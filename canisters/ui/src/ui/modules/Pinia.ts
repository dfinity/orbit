import { createPinia } from 'pinia';
import { NavigationGuard } from 'vue-router';
import { logger } from '~/core';
import { useAuthStore } from '~/ui/stores';

const pinia = createPinia();

const initStateGuard: NavigationGuard = async (_to, _from, next) => {
  const auth = useAuthStore();

  await auth
    .initialize()
    .catch(e => logger.error(`Application failed to initialize the state`, { error: e }))
    .finally(() => next());
};

export { initStateGuard, pinia };
