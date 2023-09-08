import { defaultLocale, supportedLocales } from '~/configs/i18n';
import { AppInitConfig } from '~/types/configs';

const appInitConfig: AppInitConfig = {
  baseUrl: import.meta.env.APP_BASE_URL || '/',
  locale: {
    default: defaultLocale,
    supportedLocales,
  },
};

export { appInitConfig };
