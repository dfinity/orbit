import { HttpAgent } from '@icp-sdk/core/agent';
import { App } from 'vue';
import { appInitConfig } from '~/configs/init.config';
import { AuthService } from '~/services/auth.service';
import { ControlPanelService } from '~/services/control-panel.service';
import { LocalesService } from '~/services/locales.service';
import { SessionExpirationService } from '~/services/session-expiration.service';
import { ThemeService } from '~/services/theme.service';
import { StationService } from '~/services/station.service';

export interface Services {
  locales: LocalesService;
  auth: AuthService;
  theme: ThemeService;
  controlPanel: ControlPanelService;
  station: StationService;
  sessionExpiration: SessionExpirationService;
}

const getDefaultServices = (): Services => ({
  locales: new LocalesService(),
  auth: new AuthService(),
  theme: new ThemeService(),
  controlPanel: new ControlPanelService(
    new HttpAgent({ host: appInitConfig.apiGatewayUrl.toString() }),
  ),
  station: new StationService(new HttpAgent({ host: appInitConfig.apiGatewayUrl.toString() })),
  sessionExpiration: new SessionExpirationService(),
});

class ServiceManager {
  constructor(public services: Services = getDefaultServices()) {}

  withAgent(icAgent: HttpAgent): ServiceManager {
    this.services.controlPanel = new ControlPanelService(icAgent);
    this.services.station = new StationService(icAgent);

    return this;
  }

  install(app: App): void {
    app.config.globalProperties.$services = this.services;
  }
}

const serviceManager = new ServiceManager();

const services = (): ServiceManager['services'] => serviceManager.services;

export { serviceManager, services };
