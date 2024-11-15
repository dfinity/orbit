import en from '~/locales/en.locale';
import { BreadCrumbItem } from '~/types/navigation.types';

export interface AppInitConfig {
  name: string;
  version: string;
  logLevel: 'trace' | 'debug' | 'info' | 'warn' | 'error' | 'silent';
  baseUrl: string;
  versionedBaseUrl: string;
  isProduction: boolean;
  buildMode: string;
  apiGatewayUrl: URL;
  locale: {
    default: string;
    supportedLocales: string[];
  };
  providers: {
    internetIdentity: string;
  };
  canisters: {
    app_wallet: string;
    controlPanel: string;
    internetIdentity: string;
    icpIndex: string;
  };
}

export enum SupportedTheme {
  Dark = 'dark',
  Light = 'light',
}

export type AppTranslations = typeof en;

export interface GlobalNotification {
  show: boolean;
  message: string;
  type: 'error' | 'success' | 'info' | 'warning';
}

export interface Pagination {
  limit: number;
  totalPages: number;
  selectedPage: number;
}

export interface TableHeaderProps {
  class?: string;
}

export interface TableHeader {
  title: string;
  key: string;
  sortable?: boolean;
  headerProps?: TableHeaderProps;
}

export interface PageProps {
  title?: string;
  subtitle?: string;
  breadcrumbs?: BreadCrumbItem[];
}

export type CsvRow = Record<string, string>;

export interface CsvTable {
  headers: CsvRow;
  rows: CsvRow[];
}

export enum CyclesUnit {
  Trillion = 'TC',
  Billion = 'BC',
  Million = 'MC',
  // Can be used to display cycles in the smallest unit.
  Smallest = 'e8s',
}

export enum TimeUnit {
  Years = 'Y',
  Months = 'M',
  Days = 'D',
  Hours = 'H',
  Minutes = 'Min',
  Seconds = 'Sec',
}
