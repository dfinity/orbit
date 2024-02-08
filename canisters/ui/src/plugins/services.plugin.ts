import { HttpAgent } from '@dfinity/agent';
import { App } from 'vue';
import { appInitConfig } from '~/configs/init.config';
import { AuthService } from '~/services/auth.service';
import { ControlPanelService } from '~/services/control-panel.service';
import { LocalesService } from '~/services/locales.service';
import { SessionExpirationService } from '~/services/session-expiration.service';
import { ThemeService } from '~/services/theme.service';
import { WalletService } from '~/services/wallet.service';

export interface Services {
  locales: LocalesService;
  auth: AuthService;
  theme: ThemeService;
  controlPanel: ControlPanelService;
  wallet: WalletService;
  sessionExpiration: SessionExpirationService;
}

const getDefauultServices = (): Services => ({
  locales: new LocalesService(),
  auth: new AuthService(),
  theme: new ThemeService(),
  controlPanel: new ControlPanelService(
    new HttpAgent({ host: appInitConfig.apiGatewayUrl.toString() }),
  ),
  wallet: new WalletService(new HttpAgent({ host: appInitConfig.apiGatewayUrl.toString() })),
  sessionExpiration: new SessionExpirationService(),
});

class ServiceManager {
  constructor(public services: Services = getDefauultServices()) {}

  withAgent(icAgent: HttpAgent): ServiceManager {
    this.services.controlPanel = new ControlPanelService(icAgent);
    this.services.wallet = new WalletService(icAgent);

    return this;
  }

  install(app: App): void {
    app.config.globalProperties.$services = this.services;
  }
}

const serviceManager = new ServiceManager();

const services = (): ServiceManager['services'] => serviceManager.services;

export { serviceManager, services };
