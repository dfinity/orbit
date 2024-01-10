import { App } from 'vue';
import {
  AuthService,
  WalletService,
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
      wallet: WalletService;
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
  wallet: new WalletService(),
});

const services = (): ServiceManager['services'] => serviceManager.services;

export { serviceManager, services };
