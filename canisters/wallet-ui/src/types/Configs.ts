import en from '~/locales/en.json';

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
    walletUI: string;
    controlPanel: string;
    internetIdentity: string;
  };
}

export enum SupportedTheme {
  Dark = 'dark',
  Light = 'light',
}

export type AppTranslations = typeof en;
