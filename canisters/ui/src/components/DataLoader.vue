<template>
  <slot v-if="failed" name="error" :error-msg="props.errorMsg" :error-details="errorDetails">
    <VAlert type="error" variant="tonal" density="compact">
      {{ props.errorMsg }}
    </VAlert>
  </slot>
  <slot v-else name="default" :loading="loading" :reloading="reloading" :data="data"></slot>
</template>
<script lang="ts" setup generic="T">
import { computed, onMounted, onUnmounted, ref, watch } from 'vue';
import { VAlert } from 'vuetify/components';
import { logger } from '~/core/logger.core';
import { i18n } from '~/plugins/i18n.plugin';
import { useSessionStore } from '~/stores/session.store';

const loading = ref<boolean>(false);
const failed = ref<boolean>(false);
const data = ref<T | undefined>();
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

      data.value = undefined;

      fetchData({ cleanupOnFail: false });
    }
  },
);
const session = useSessionStore();

let refreshTimer: ReturnType<typeof setInterval> | undefined;

if (props.refreshIntervalMs) {
  refreshTimer = setInterval(async () => fetchData(), props.refreshIntervalMs);
}

const fetchWithRetries = async (retries: number): Promise<T> => {
  try {
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
    if (data.value === undefined) {
      loading.value = value;
      emit('loading', value);
    } else {
      reloading.value = value;
    }
  },
});

const fetchData = async ({ cleanupOnFail }: { cleanupOnFail?: boolean } = {}): Promise<void> => {
  try {
    if (
      // prevents multiple calls to fetchData at the same time
      working.value ||
      // disables the refresh functionality if set by the parent component
      props.disableRefresh ||
      // prevents calls to fetchData while the user is locked out
      session.reauthenticationNeeded
    ) {
      return;
    }

    failed.value = false;
    working.value = true;

    const newData = await fetchWithRetries(props.retries);

    working.value = false;

    if (!props.disableRefresh) {
      data.value = newData;
      emit('loaded', data.value);
    }
  } catch (err) {
    logger.error(`Failed to load data: ${err}`);

    if (cleanupOnFail) {
      if (refreshTimer) {
        clearInterval(refreshTimer);
      }

      failed.value = true;
    }

    errorDetails.value = `${err}`;
    emit('failed', err);
  } finally {
    working.value = false;
  }
};

onMounted(async () => fetchData({ cleanupOnFail: true }));

onUnmounted(() => {
  if (refreshTimer) {
    clearInterval(refreshTimer);
  }
});
</script>
