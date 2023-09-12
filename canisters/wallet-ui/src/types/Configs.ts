import en from '~/locales/en.json';

export interface AppInitConfig {
  version: string;
  logLevel: 'trace' | 'debug' | 'info' | 'warn' | 'error' | 'silent';
  baseUrl: string;
  locale: {
    default: string;
    supportedLocales: string[];
  };
}

export type AppTranslations = typeof en;
