<template>
  <slot v-if="failed" name="error">
    <VAlert type="error" variant="tonal" density="compact">
      {{ props.errorMsg }}
    </VAlert>
  </slot>
  <slot v-else name="default" :loading="loading" :data="data"></slot>
</template>
<script lang="ts" setup generic="T">
import { onUnmounted } from 'vue';
import { onMounted, ref } from 'vue';
import { logger } from '~/core/logger';
import { i18n } from '~/ui/modules/i18n';

const loading = ref<boolean>(false);
const failed = ref<boolean>(false);
const data = ref<T | undefined>();

const props = withDefaults(
  defineProps<{
    load: () => Promise<T>;
    retries?: number;
    refreshIntervalMs?: number;
    errorMsg?: string;
  }>(),
  {
    errorMsg: i18n.global.t('app.data_load_error'),
    refreshIntervalMs: undefined,
    retries: 0,
  },
);

const emit = defineEmits<{
  (event: 'failed', payload: unknown): void;
  (event: 'loaded', payload: T): void;
}>();

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

const fetchData = async ({ cleanupOnFail }: { cleanupOnFail?: boolean } = {}): Promise<void> => {
  try {
    if (loading.value) {
      // prevents multiple calls to fetchData at the same time
      return;
    }

    failed.value = false;
    loading.value = true;

    data.value = await fetchWithRetries(props.retries);

    emit('loaded', data.value);
  } catch (err) {
    logger.error(`Failed to load data: ${err}`);

    if (cleanupOnFail) {
      if (refreshTimer) {
        clearInterval(refreshTimer);
      }

      failed.value = true;
    }

    emit('failed', err);
  } finally {
    loading.value = false;
  }
};

onMounted(async () => fetchData({ cleanupOnFail: true }));

onUnmounted(() => {
  if (refreshTimer) {
    clearInterval(refreshTimer);
  }
});
</script>
