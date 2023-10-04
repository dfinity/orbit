import { App } from 'vue';
import {
  AuthService,
  BankService,
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
      bank: BankService;
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
  bank: new BankService(),
});

const services = (): ServiceManager['services'] => serviceManager.services;

export { serviceManager, services };
