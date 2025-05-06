import { onBeforeUnmount, onMounted, ref } from 'vue';

export function useInterval(callback: () => void, periodMs: number) {
  const interval = ref<ReturnType<typeof setInterval> | null>(null);

  onMounted(() => {
    interval.value = setInterval(callback, periodMs);
  });

  onBeforeUnmount(() => {
    if (interval.value) {
      clearInterval(interval.value);
    }
  });
}
