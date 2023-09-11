import { defaultLocale, supportedLocales } from '~/configs/I18n';
import { AppInitConfig } from '~/types';

const appInitConfig: AppInitConfig = {
  baseUrl: import.meta.env.APP_BASE_URL || '/',
  locale: {
    default: defaultLocale,
    supportedLocales,
  },
};

export { appInitConfig };
