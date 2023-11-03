import { mdiCogs, mdiHome, mdiLogoutVariant, mdiWalletBifold } from '@mdi/js';
import { App } from 'vue';
import { NavigationGuard } from 'vue-router';
import { defaultHomeRoute, defaultLoginRoute, redirectToKey } from '~/ui/modules';
import { useAuthStore } from '~/ui/stores';
import { AuthState, NavigationActionType, NavigationSection } from '~/ui/types';

const mainNavigation: NavigationSection[] = [
  {
    name: 'main',
    localeKey: 'navigation.main.name',
    items: [
      {
        name: 'home',
        localeKey: 'navigation.main.items.home',
        action: {
          type: NavigationActionType.To,
          handle: route => (route.params.locale ? `/${route.params.locale}/home` : '/home'),
        },
        icon: mdiHome,
      },
      {
        name: 'accounts',
        localeKey: 'navigation.main.items.accounts',
        action: {
          type: NavigationActionType.To,
          handle: route => (route.params.locale ? `/${route.params.locale}/accounts` : '/accounts'),
        },
        icon: mdiWalletBifold,
      },
    ],
  },
  {
    name: 'configuration',
    localeKey: 'navigation.configuration.name',
    items: [
      {
        name: 'settings',
        localeKey: 'navigation.configuration.items.settings',
        action: {
          type: NavigationActionType.To,
          handle: route => (route.params.locale ? `/${route.params.locale}/settings` : '/settings'),
        },
        icon: mdiCogs,
      },
      {
        name: 'logout',
        localeKey: 'navigation.configuration.items.logout',
        action: {
          type: NavigationActionType.Callback,
          handle: async () => {
            const auth = useAuthStore();

            await auth.signOut();
          },
        },
        icon: mdiLogoutVariant,
      },
    ],
  },
];

class Navigation {
  constructor(
    private readonly navigation: {
      main: NavigationSection[];
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
      main: NavigationSection[];
    };
  }
}

export const navigation = new Navigation({
  main: mainNavigation,
});
