import { appInitConfig } from '~/configs';

export class RoutesService {
  get baseUrl(): string {
    return `${appInitConfig.baseUrl}`;
  }
}
