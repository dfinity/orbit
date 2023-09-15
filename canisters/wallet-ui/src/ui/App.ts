import { createApp } from 'vue';
import App from '~/ui/App.vue';
import { i18n, navigation, pinia, router, serviceManager, vuetify } from '~/ui/modules';
import './App.scss';

export const initializeApp = () => {
  const app = createApp(App);

  app.use(pinia);
  app.use(vuetify);
  app.use(i18n);
  app.use(router);
  app.use(navigation);
  app.use(serviceManager);

  app.mount('#app');
};
