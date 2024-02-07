import { onMounted, onUnmounted } from 'vue';
import { throttle } from '~/core/utils.core';

export function useUserActivity({
  onActive,
  throttleMs = 1000,
}: {
  onActive: () => void;
  throttleMs?: number;
}) {
  const registerActivity = throttle(() => {
    onActive();
  }, throttleMs);

  onMounted(() => {
    window.addEventListener('mousemove', registerActivity);
    window.addEventListener('mousedown', registerActivity);
    window.addEventListener('keypress', registerActivity);
    window.addEventListener('DOMMouseScroll', registerActivity);
    window.addEventListener('mousewheel', registerActivity);
    window.addEventListener('touchmove', registerActivity);
    window.addEventListener('MSPointerMove', registerActivity);
  });

  onUnmounted(() => {
    window.removeEventListener('mousemove', registerActivity);
    window.removeEventListener('mousedown', registerActivity);
    window.removeEventListener('keypress', registerActivity);
    window.removeEventListener('DOMMouseScroll', registerActivity);
    window.removeEventListener('mousewheel', registerActivity);
    window.removeEventListener('touchmove', registerActivity);
    window.removeEventListener('MSPointerMove', registerActivity);
  });
}
