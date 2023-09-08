import { Plugin } from 'vue';
import { LocalesService, RoutesService } from '~/services';

const services = {
  locales: new LocalesService(),
  routes: new RoutesService(),
};

const appServicesPlugin: Plugin = {
  install(app, _options: unknown) {
    app.config.globalProperties.$services = services;
  },
};

export { appServicesPlugin, services };
