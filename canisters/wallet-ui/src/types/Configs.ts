import en from '~/locales/en.json';

export interface AppInitConfig {
  baseUrl: string;
  locale: {
    default: string;
    supportedLocales: string[];
  };
}

export type AppTranslations = typeof en;
