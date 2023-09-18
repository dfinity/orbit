/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly APP_ALCHEMY_API_KEY: string;
  readonly APP_ALCHEMY_JSON_RPC_URL: string;
  readonly APP_ENV: 'development' | 'production';
  readonly APP_BASE_URL: string;
  readonly APP_TITLE: string;
  readonly APP_VERSION: string;
  readonly APP_SUPPORTED_LOCALES: string;
  readonly APP_LOG_LEVEL: 'trace' | 'debug' | 'info' | 'warn' | 'error' | 'silent';
  readonly APP_WALLET_UI_CANISTER_ID: string;
  readonly APP_INTERNET_IDENTITY_CANISTER_ID: string;
  readonly APP_INTERNET_IDENTITY_PROVIDER_URL: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
