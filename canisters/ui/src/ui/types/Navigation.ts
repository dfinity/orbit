import { RouteLocationNormalizedLoaded } from 'vue-router';

export enum NavigationActionType {
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

export type NavigationAction = NavigationTo | NavigationHref | NavigationCallback;

export interface NavigationItem {
  name: string;
  localeKey: string;
  icon: string;
  action: NavigationAction;
}

export interface NavigationSection {
  name: string;
  localeKey: string;
  items: NavigationItem[];
}
