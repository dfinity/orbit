import { App } from 'vue';
import { LocalesService, RoutesService } from '~/services';

class ServiceManager {
  constructor(
    public readonly services: {
      locales: LocalesService;
      routes: RoutesService;
    },
  ) {}

  install(app: App): void {
    app.config.globalProperties.$services = this.services;
  }
}

const serviceManager = new ServiceManager({
  locales: new LocalesService(),
  routes: new RoutesService(),
});

const services = (): ServiceManager['services'] => serviceManager.services;

export { serviceManager, services };
