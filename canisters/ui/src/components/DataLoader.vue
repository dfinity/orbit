<template>
  <slot v-if="failed" name="error" :error-msg="props.errorMsg" :error-details="errorDetails">
    <VAlert type="error" variant="tonal" density="compact">
      {{ props.errorMsg }}
    </VAlert>
  </slot>
  <slot v-else name="default" :loading="loading" :reloading="reloading" :data="data"></slot>
</template>
<script lang="ts" setup generic="T">
import { Ref } from 'vue';
import { computed, onMounted, onUnmounted, ref, watch } from 'vue';
import { VAlert } from 'vuetify/components';
import { DisabledBackgroundPollingError } from '~/core/errors.core';
import { logger } from '~/core/logger.core';
import { i18n } from '~/plugins/i18n.plugin';
import { useAppStore } from '~/stores/app.store';
import { useSessionStore } from '~/stores/session.store';

const loading = ref<boolean>(false);
const failed = ref<boolean>(false);
const data: Ref<T | null> = ref(null);
const reloading = ref<boolean>(false);
const errorDetails = ref<string>();

const props = withDefaults(
  defineProps<{
    load: () => Promise<T>;
    retries?: number;
    refreshIntervalMs?: number;
    errorMsg?: string;
    forceReload?: boolean;
    disableRefresh?: boolean;
  }>(),
  {
    errorMsg: i18n.global.t('app.data_load_error'),
    refreshIntervalMs: undefined,
    retries: 0,
    forceReload: false,
    disableRefresh: false,
  },
);

const emit = defineEmits<{
  (event: 'failed', payload: unknown): void;
  (event: 'loaded', payload: T): void;
  (event: 'loading', payload: boolean): void;
  (event: 'update:forceReload', payload: boolean): void;
}>();

watch(
  () => props.forceReload,
  forceReload => {
    if (forceReload) {
      emit('update:forceReload', false);

      data.value = null;

      fetchData({ cleanupOnFail: false }, true);
    }
  },
);

const app = useAppStore();
const session = useSessionStore();
const initialized = ref(false);
const fetchJobId = ref<number | null>(null);

let refreshTimer: ReturnType<typeof setInterval> | undefined;

if (props.refreshIntervalMs) {
  refreshTimer = setInterval(async () => fetchData(), props.refreshIntervalMs);
}

const canFetchData = (): boolean => {
  // the data is first loaded if the component has just been initialized
  if (!initialized.value) {
    // prevents calls to fetchData while the user is locked out
    return !session.reauthenticationNeeded;
  }

  return (
    // disables the refresh functionality if set by the parent component
    !props.disableRefresh &&
    // prevents calls to fetchData while the user is locked out
    !session.reauthenticationNeeded &&
    // prevents calls to fetchData, this can happen when the user is switching between wallets
    !app.disableBackgroundPolling
  );
};

const fetchWithRetries = async (retries: number): Promise<T> => {
  try {
    if (!canFetchData()) {
      throw new DisabledBackgroundPollingError();
    }

    return await props.load();
  } catch (err) {
    if (retries > 0) {
      return fetchWithRetries(retries - 1);
    }

    throw err;
  }
};

const working = computed({
  get: () => loading.value || reloading.value,
  set: (value: boolean) => {
    if (data.value == null) {
      loading.value = value;
      emit('loading', value);
    } else {
      reloading.value = value;
    }
  },
});

const fetchData = async (
  { cleanupOnFail }: { cleanupOnFail?: boolean } = {},
  force = false,
): Promise<void> => {
  try {
    // prevents multiple calls to fetchData at the same time
    if (working.value && !force) {
      return;
    }

    if (!canFetchData()) {
      return;
    }

    const jobId = fetchJobId.value === null ? 1 : fetchJobId.value + 1;

    failed.value = false;
    working.value = true;
    fetchJobId.value = jobId;

    const newData = await fetchWithRetries(props.retries);

    if (fetchJobId.value !== jobId) {
      return;
    }

    fetchJobId.value = null;
    working.value = false;

    if (
      // the data is first loaded if the component has just been initialized
      !initialized.value ||
      // the data is reloaded if the component has been initialized and the refresh interval is set
      !props.disableRefresh
    ) {
      data.value = newData;
      emit('loaded', data.value);
    }
  } catch (err) {
    if (err instanceof DisabledBackgroundPollingError) {
      // do nothing, this is expected
      return;
    }

    logger.error(`Failed to load data`, err);

    if (cleanupOnFail) {
      if (refreshTimer) {
        clearInterval(refreshTimer);
      }

      failed.value = true;
    }

    errorDetails.value = `${err}`;
    emit('failed', err);

    working.value = false;
  }
};

onMounted(async () => fetchData({ cleanupOnFail: true }).then(() => (initialized.value = true)));

onUnmounted(() => {
  if (refreshTimer) {
    clearInterval(refreshTimer);
  }
});
</script>
