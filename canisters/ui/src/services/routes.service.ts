import { appInitConfig } from '~/configs/init.config';

export class RoutesService {
  get baseUrl(): string {
    return `${appInitConfig.baseUrl}`;
  }
}
