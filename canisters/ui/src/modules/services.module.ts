import { App } from 'vue';
import { AuthService } from '~/services/auth.service';
import { ControlPanelService } from '~/services/control-panel.service';
import { LocalesService } from '~/services/locales.service';
import { RoutesService } from '~/services/routes.service';
import { SessionExpirationService } from '~/services/session-expiration.service';
import { ThemeService } from '~/services/theme.service';
import { WalletService } from '~/services/wallet.service';

class ServiceManager {
  constructor(
    public readonly services: {
      locales: LocalesService;
      routes: RoutesService;
      auth: AuthService;
      theme: ThemeService;
      controlPanel: ControlPanelService;
      wallet: WalletService;
      sessionExpiration: SessionExpirationService;
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
  sessionExpiration: new SessionExpirationService(),
});

const services = (): ServiceManager['services'] => serviceManager.services;

export { serviceManager, services };
