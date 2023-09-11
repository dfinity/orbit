import { createApp } from 'vue';
import 'vuetify/styles';
import App from '~/ui/App.vue';
import { i18n, pinia, router, serviceManager, vuetify } from '~/ui/modules';
import '~/ui/style.scss';

export const initializeApp = () => {
  const app = createApp(App);

  app.use(pinia);
  app.use(vuetify);
  app.use(i18n);
  app.use(router);
  app.use(serviceManager);

  app.mount('#app');
};
