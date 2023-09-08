import { createApp } from 'vue';
import 'vuetify/styles';
import App from '~/ui/App.vue';
import { i18n } from '~/ui/i18n';
import { router } from '~/ui/router';
import { appServicesPlugin as services } from '~/ui/services';
import { pinia } from '~/ui/stores';
import '~/ui/style.scss';
import { vuetify } from '~/ui/vuetify';

export const initializeApp = () => {
  const app = createApp(App);

  app.use(pinia);
  app.use(vuetify);
  app.use(i18n);
  app.use(router);
  app.use(services);

  app.mount('#app');
};
