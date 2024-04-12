import { createApp } from 'vue';
import { icAgent } from '~/core/ic-agent.core';
import { logger } from '~/core/logger.core';
import { i18n } from '~/plugins/i18n.plugin';
import { navigation } from '~/plugins/navigation.plugin';
import { pinia } from '~/plugins/pinia.plugin';
import { router } from '~/plugins/router.plugin';
import { serviceManager } from '~/plugins/services.plugin';
import { vuetify } from '~/plugins/vuetify.plugin';
import './styles/main.scss';
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
  app.use(serviceManager.withAgent(icAgent.get()));
  app.use(vuetify(serviceManager.services.theme.resolveTheme()));
  app.use(i18n);
  app.use(router);
  app.use(navigation.withRoutes(router.getRoutes()));

  app.mount('#app');
};

initializeApp().catch(err => {
  logger.error(`Failed to initialize app`, { err });
});
