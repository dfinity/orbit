import { createApp } from 'vue';
import { icAgent } from '~/core/ic-agent.core';
import { logger } from '~/core/logger.core';
import { i18n } from '~/modules/i18n.module';
import { navigation } from '~/modules/navigation.module';
import { pinia } from '~/modules/pinia.module';
import { router } from '~/modules/router.module';
import { serviceManager } from '~/modules/services.module';
import { vuetify } from '~/modules/vuetify.module';
import './App.scss';
import App from './App.vue';

const initializeApp = async (): Promise<void> => {
  const app = createApp(App);

  app.config.errorHandler = (err, instance, info) => {
    logger.error(`Global VueError`, {
      err,
      instance,
      info,
    });
  };

  await icAgent.init();

  app.use(pinia);
  app.use(vuetify);
  app.use(i18n);
  app.use(serviceManager);
  app.use(router);
  app.use(navigation);

  app.mount('#app');
};

initializeApp().catch(err => {
  logger.error(`Failed to initialize app`, { err });
});
