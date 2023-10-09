import { defaultLocale, supportedLocales } from '~/configs/I18n';
import { AppInitConfig } from '~/types';

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
    internetIdentity: import.meta.env.APP_INTERNET_IDENTITY_PROVIDER_URL,
  },
  canisters: {
    controlPanel: import.meta.env.APP_CONTROL_PANEL_CANISTER_ID,
    internetIdentity: import.meta.env.APP_INTERNET_IDENTITY_CANISTER_ID,
    walletUI: import.meta.env.APP_WALLET_UI_CANISTER_ID,
    icpIndex: import.meta.env.APP_CANISTER_ID_ICP_INDEX,
  },
};

export { appInitConfig };
