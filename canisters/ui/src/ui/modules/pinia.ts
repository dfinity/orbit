import { createPinia } from 'pinia';
import { NavigationGuard } from 'vue-router';
import { logger } from '~/core';
import { useAppStore } from '~/ui/stores';

const pinia = createPinia();

const initStateGuard: NavigationGuard = async (_to, _from, next) => {
  const app = useAppStore();

  await app
    .initialize()
    .catch(e => logger.error(`Application failed to initialize the state`, { error: e }))
    .finally(() => next());
};

export { initStateGuard, pinia };
