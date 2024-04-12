/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly APP_ALCHEMY_API_KEY: string;
  readonly APP_ALCHEMY_JSON_RPC_URL: string;
  readonly APP_ENV: 'development' | 'production';
  readonly APP_URL: string;
  readonly APP_BASE_URL: string;
  readonly APP_TITLE: string;
  readonly APP_VERSION: string;
  readonly APP_SUPPORTED_LOCALES: string;
  readonly APP_LOG_LEVEL: 'trace' | 'debug' | 'info' | 'warn' | 'error' | 'silent';
  readonly APP_CANISTER_ID_UI: string;
  readonly APP_CANISTER_ID_CONTROL_PANEL: string;
  readonly APP_CANISTER_ID_INTERNET_IDENTITY: string;
  readonly APP_PROVIDER_URL_INTERNET_IDENTITY: string;
  readonly APP_CANISTER_ID_ICP_INDEX: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
