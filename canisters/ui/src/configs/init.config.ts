import { defaultLocale, supportedLocales } from './i18n.config';
import { AppInitConfig } from '~/types/app.types';

const appInitConfig: AppInitConfig = {
  name: import.meta.env.APP_TITLE || 'App',
  version: import.meta.env.APP_VERSION || '0.0.0',
  logLevel: import.meta.env.APP_LOG_LEVEL || 'info',
  baseUrl: import.meta.env.APP_BASE_URL || '/',
  isProduction: !!import.meta.env.PROD,
  apiGatewayUrl: new URL(import.meta.env.PROD ? 'https://icp-api.io' : 'http://localhost:4943'),
  locale: {
    default: defaultLocale,
    supportedLocales,
  },
  providers: {
    internetIdentity: import.meta.env.APP_PROVIDER_URL_INTERNET_IDENTITY,
  },
  canisters: {
    controlPanel: import.meta.env.APP_CANISTER_ID_CONTROL_PANEL,
    ui: import.meta.env.APP_CANISTER_ID_UI,
    internetIdentity: import.meta.env.APP_CANISTER_ID_INTERNET_IDENTITY,
    icpIndex: import.meta.env.APP_CANISTER_ID_ICP_INDEX,
  },
};

export { appInitConfig };
