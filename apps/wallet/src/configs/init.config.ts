import type { AppInitConfig } from '~/types/app.types';
import { defaultLocale, supportedLocales } from './i18n.config';
import { isSemanticVersion } from '~/utils/helper.utils';

let baseUrl = import.meta.env.BASE_URL || '/';
const currentPath = location.pathname;
const parsedPath = currentPath.startsWith(baseUrl)
  ? currentPath.slice(baseUrl.length)
  : currentPath;
const parts = parsedPath.split('/').filter(Boolean);

if (parts.length && isSemanticVersion(parts[0], 'v')) {
  baseUrl = `${baseUrl}${parts[0]}/`;
}

const appInitConfig: AppInitConfig = {
  name: import.meta.env.APP_TITLE || 'Orbit',
  version: import.meta.env.APP_VERSION || '0.0.0',
  logLevel: import.meta.env.APP_LOG_LEVEL || 'info',
  baseUrl,
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
