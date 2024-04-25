import type { AppInitConfig } from '~/types/app.types';
import { defaultLocale, supportedLocales } from './i18n.config';

const appInitConfig: AppInitConfig = {
  name: import.meta.env.APP_TITLE || 'Orbit',
  version: import.meta.env.APP_VERSION || '0.0.0',
  logLevel: import.meta.env.APP_LOG_LEVEL || 'info',
  baseUrl: import.meta.env.APP_BASE_URL || '/',
  buildMode: import.meta.env.APP_BUILD_MODE || 'production',
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
    app_wallet: import.meta.env.APP_CANISTER_ID_APP_WALLET,
    controlPanel: import.meta.env.APP_CANISTER_ID_CONTROL_PANEL,
    internetIdentity: import.meta.env.APP_CANISTER_ID_INTERNET_IDENTITY,
    icpIndex: import.meta.env.APP_CANISTER_ID_ICP_INDEX,
  },
};

export { appInitConfig };
