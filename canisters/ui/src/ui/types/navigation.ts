import { RouteLocationNormalizedLoaded } from 'vue-router';
import { Routes } from '~/ui/config/routes';
import { AccessCriteria } from '~/ui/types/auth';

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

export enum NavigastionAuthType {
  Route = 'route',
  Custom = 'custom',
}

export interface NavigationAuthRouteCheck {
  type: NavigastionAuthType.Route;
  route: Routes;
}

export interface NavigationAuthCustomCheck {
  type: NavigastionAuthType.Custom;
  criteria: AccessCriteria;
}

export type NavigastionAuth = NavigationAuthRouteCheck | NavigationAuthCustomCheck;

export interface NavigationItem {
  name: string;
  localeKey: string;
  icon?: string;
  exact?: boolean;
  action: NavigationAction;
  items?: NavigationItem[];
  auth: NavigastionAuth;
}
