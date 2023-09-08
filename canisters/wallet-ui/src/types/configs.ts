export interface AppInitConfig {
  baseUrl: string;
  locale: {
    default: string;
    supportedLocales: string[];
  };
}
