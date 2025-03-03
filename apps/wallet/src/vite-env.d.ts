/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly APP_MODE: 'development' | 'production';
  readonly APP_STATION_API_VERSION: string;
  readonly APP_URL: string;
  readonly APP_BASE_URL: string;
  readonly APP_TITLE: string;
  readonly APP_BUILD_MODE: string;
  readonly APP_BUILD_VERSION: string;
  readonly APP_BUILD_HASH: string;
  readonly APP_SUPPORTED_LOCALES: string;
  readonly APP_LOG_LEVEL: 'trace' | 'debug' | 'info' | 'warn' | 'error' | 'silent';
  readonly APP_CANISTER_ID_APP_WALLET: string;
  readonly APP_CANISTER_ID_CONTROL_PANEL: string;
  readonly APP_CANISTER_ID_INTERNET_IDENTITY: string;
  readonly APP_PROVIDER_URL_INTERNET_IDENTITY: string;
  readonly APP_CANISTER_ID_ICP_INDEX: string;
  readonly APP_MARKETING_SITE_URL?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
