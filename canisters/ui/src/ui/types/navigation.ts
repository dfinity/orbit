import { RouteLocationNormalizedLoaded } from 'vue-router';

export enum NavigationActionType {
  None = 'none',
  Callback = 'callback',
  Href = 'href',
  To = 'to',
}

export type NavigationCallback = {
  type: NavigationActionType.Callback;
  handle: () => void;
};

export type NavigationHref = {
  type: NavigationActionType.Href;
  handle: () => string;
};

export type NavigationTo = {
  type: NavigationActionType.To;
  handle: (route: RouteLocationNormalizedLoaded) => string;
};

export type NagivationNone = {
  type: NavigationActionType.None;
};

export type NavigationAction = NavigationTo | NavigationHref | NavigationCallback | NagivationNone;

export interface NavigationItem {
  name: string;
  localeKey: string;
  icon?: string;
  action: NavigationAction;
  items?: NavigationItem[];
}
