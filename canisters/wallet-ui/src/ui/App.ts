import { createApp } from 'vue';
import { logger } from '~/core';
import App from '~/ui/App.vue';
import { i18n, navigation, pinia, router, serviceManager, vuetify } from '~/ui/modules';
import './App.scss';

export const initializeApp = () => {
  const app = createApp(App);

  app.config.errorHandler = (err, instance, info) => {
    logger.error(`Global VueError`, {
      err,
      instance,
      info,
    });
  };

  app.use(pinia);
  app.use(vuetify);
  app.use(i18n);
  app.use(router);
  app.use(navigation);
  app.use(serviceManager);

  app.mount('#app');
};
