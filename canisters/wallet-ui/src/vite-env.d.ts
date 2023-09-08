/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly APP_ALCHEMY_API_KEY: string;
  readonly APP_ALCHEMY_JSON_RPC_URL: string;
  readonly APP_ENV: 'development' | 'production';
  readonly APP_BASE_URL: string;
  readonly APP_TITLE: string;
  readonly APP_VERSION: string;
  readonly APP_DEFAULT_LOCALE: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
