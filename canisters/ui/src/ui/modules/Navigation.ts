import { mdiCogs, mdiHome, mdiWalletBifold, mdiBookOpenVariant } from '@mdi/js';
import { App } from 'vue';
import { NavigationGuard } from 'vue-router';
import { defaultHomeRoute, defaultLoginRoute, redirectToKey } from '~/ui/modules';
import { useAuthStore } from '~/ui/stores';
import { AuthState, NavigationActionType, NavigationItem } from '~/ui/types';

const mainNavigation: NavigationItem[] = [
  {
    name: 'settings',
    localeKey: 'navigation.settings',
    action: {
      type: NavigationActionType.None,
    },
    icon: mdiCogs,
    items: [
      {
        name: 'administration',
        localeKey: 'navigation.administration',
        action: {
          type: NavigationActionType.To,
          handle: route =>
            route.params.locale
              ? `/${route.params.locale}/settings/administration`
              : '/settings/administration',
        },
      },
      {
        name: 'user_groups_permissions',
        localeKey: 'navigation.user_groups_permissions',
        action: {
          type: NavigationActionType.To,
          handle: route =>
            route.params.locale
              ? `/${route.params.locale}/settings/permissions`
              : '/settings/permissions',
        },
      },
      {
        name: 'users',
        localeKey: 'navigation.users',
        action: {
          type: NavigationActionType.To,
          handle: route =>
            route.params.locale ? `/${route.params.locale}/settings/users` : '/settings/users',
        },
      },
      {
        name: 'address_book',
        localeKey: 'navigation.address_book',
        action: {
          type: NavigationActionType.To,
          handle: route =>
            route.params.locale
              ? `/${route.params.locale}/settings/address-book`
              : '/settings/address-book',
        },
      },
    ],
  },
  {
    name: 'home',
    localeKey: 'navigation.overview',
    action: {
      type: NavigationActionType.To,
      handle: route => (route.params.locale ? `/${route.params.locale}/overview` : '/overview'),
    },
    icon: mdiHome,
  },
  {
    name: 'accounts',
    localeKey: 'navigation.accounts_transfers',
    action: {
      type: NavigationActionType.To,
      handle: route => (route.params.locale ? `/${route.params.locale}/accounts` : '/accounts'),
    },
    icon: mdiWalletBifold,
  },
  {
    name: 'address_book',
    localeKey: 'navigation.address_book',
    action: {
      type: NavigationActionType.To,
      handle: route =>
        route.params.locale ? `/${route.params.locale}/address-book` : '/address-book',
    },
    icon: mdiBookOpenVariant,
  },
];

class Navigation {
  constructor(
    private readonly navigation: {
      main: NavigationItem[];
    },
  ) {}

  install(app: App): void {
    app.config.globalProperties.$navigation = this.navigation;
  }
}

export const navigationGuard: NavigationGuard = async (to, _from, next) => {
  const auth = useAuthStore();

  if (to.meta.auth.requireState === AuthState.Authenticated && !auth.isAuthenticated) {
    window?.sessionStorage.setItem(redirectToKey, to.fullPath);
    return next({ name: defaultLoginRoute });
  }

  if (to.meta.auth.requireState === AuthState.Guest && auth.isAuthenticated) {
    return next({ name: defaultHomeRoute });
  }

  return next();
};

declare module 'vue' {
  interface ComponentCustomProperties {
    $navigation: {
      main: NavigationItem[];
    };
  }
}

export const navigation = new Navigation({
  main: mainNavigation,
});
