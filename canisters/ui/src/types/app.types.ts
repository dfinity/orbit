import en from '~/locales/en.locale';

export interface AppInitConfig {
  name: string;
  version: string;
  logLevel: 'trace' | 'debug' | 'info' | 'warn' | 'error' | 'silent';
  baseUrl: string;
  isProduction: boolean;
  apiGatewayUrl: URL;
  locale: {
    default: string;
    supportedLocales: string[];
  };
  providers: {
    internetIdentity: string;
  };
  canisters: {
    ui: string;
    controlPanel: string;
    internetIdentity: string;
    icpIndex: string;
  };
}

export enum SupportedTheme {
  Dark = 'dark',
  Light = 'light',
}

export type AppTranslations = typeof en;

export interface GlobalNotification {
  show: boolean;
  message: string;
  type: 'error' | 'success' | 'info' | 'warning';
}
