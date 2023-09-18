import { createPinia } from 'pinia';
import { NavigationGuard } from 'vue-router';
import { useAuthStore } from '~/ui/stores';

const pinia = createPinia();

const initStateGuard: NavigationGuard = async (_to, _from, next) => {
  const auth = useAuthStore();

  await auth.initialize().finally(() => next());
};

export { initStateGuard, pinia };
