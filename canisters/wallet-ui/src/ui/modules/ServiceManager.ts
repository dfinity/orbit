import { App } from 'vue';
import {
  AuthService,
  ControlPanelService,
  LocalesService,
  RoutesService,
  ThemeService,
} from '~/services';

class ServiceManager {
  constructor(
    public readonly services: {
      locales: LocalesService;
      routes: RoutesService;
      auth: AuthService;
      theme: ThemeService;
      controlPanel: ControlPanelService;
    },
  ) {}

  install(app: App): void {
    app.config.globalProperties.$services = this.services;
  }
}

const serviceManager = new ServiceManager({
  locales: new LocalesService(),
  routes: new RoutesService(),
  auth: new AuthService(),
  theme: new ThemeService(),
  controlPanel: new ControlPanelService(),
});

const services = (): ServiceManager['services'] => serviceManager.services;

export { serviceManager, services };
