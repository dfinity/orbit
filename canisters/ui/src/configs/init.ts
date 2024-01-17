import { defaultLocale, supportedLocales } from '~/configs/i18n';
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
    internetIdentity: import.meta.env.APP_PROVIDER_URL_INTERNET_IDENTITY,
  },
  canisters: {
    controlPanel: import.meta.env.APP_CANISTER_ID_CONTROL_PANEL || '65evf-oqaaa-aaaal-add6q-cai',
    ui: import.meta.env.APP_CANISTER_ID_UI || '6uh6z-yyaaa-aaaal-add7a-cai',
    internetIdentity:
      import.meta.env.APP_CANISTER_ID_INTERNET_IDENTITY || 'rdmx6-jaaaa-aaaaa-aaadq-cai',
    icpIndex: import.meta.env.APP_CANISTER_ID_ICP_INDEX || 'qhbym-qaaaa-aaaaa-aaafq-cai',
  },
};

export { appInitConfig };
