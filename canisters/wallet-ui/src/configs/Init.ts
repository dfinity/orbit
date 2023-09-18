import { defaultLocale, supportedLocales } from '~/configs/I18n';
import { AppInitConfig } from '~/types';

const appInitConfig: AppInitConfig = {
  name: import.meta.env.APP_TITLE || 'App',
  version: import.meta.env.APP_VERSION || '0.0.0',
  logLevel: import.meta.env.APP_LOG_LEVEL || 'info',
  baseUrl: import.meta.env.APP_BASE_URL || '/',
  locale: {
    default: defaultLocale,
    supportedLocales,
  },
  providers: {
    internetIdentity: import.meta.env.APP_INTERNET_IDENTITY_PROVIDER_URL,
  },
};

export { appInitConfig };
