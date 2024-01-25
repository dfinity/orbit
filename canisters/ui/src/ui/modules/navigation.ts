import { mdiBookOpenVariant, mdiCogs, mdiHome, mdiWalletBifold, mdiFormatListText } from '@mdi/js';
import { App, Ref, computed, ref, watch } from 'vue';
import { RouteRecordRaw } from 'vue-router';
import { logger } from '~/core';
import { Routes, routes } from '~/ui/config/routes';
import { useSessionStore } from '~/ui/stores/session';
import { useWalletStore } from '~/ui/stores/wallet';
import { NavigastionAuthType, NavigationActionType, NavigationItem } from '~/ui/types';
import { hasRequiredPrivilege, hasRequiredSession } from '~/ui/utils/auth';

const sections = (): NavigationSections => ({
  main: [
    {
      name: 'settings',
      localeKey: 'navigation.settings',
      action: {
        type: NavigationActionType.None,
      },
      icon: mdiCogs,
      auth: {
        type: NavigastionAuthType.Route,
        route: Routes.MySettings,
      },
      items: [
        {
          name: 'system',
          localeKey: 'navigation.administration',
          action: {
            type: NavigationActionType.To,
            handle: route =>
              route.params.locale ? `/${route.params.locale}/settings/system` : '/settings/system',
          },
          auth: {
            type: NavigastionAuthType.Route,
            route: Routes.SystemSettings,
          },
        },
        {
          name: 'user_groups_permissions',
          localeKey: 'navigation.user_groups_permissions',
          action: {
            type: NavigationActionType.To,
            handle: route =>
              route.params.locale
                ? `/${route.params.locale}/settings/user-groups`
                : '/settings/user-groups',
          },
          auth: {
            type: NavigastionAuthType.Route,
            route: Routes.UserGroups,
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
          auth: {
            type: NavigastionAuthType.Route,
            route: Routes.Users,
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
          auth: {
            type: NavigastionAuthType.Route,
            route: Routes.AddressBookSettings,
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
      auth: {
        type: NavigastionAuthType.Route,
        route: Routes.Overview,
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
      auth: {
        type: NavigastionAuthType.Route,
        route: Routes.Accounts,
      },
      icon: mdiWalletBifold,
    },
    {
      name: 'proposals',
      localeKey: 'navigation.proposals',
      action: {
        type: NavigationActionType.To,
        handle: route => (route.params.locale ? `/${route.params.locale}/requests` : '/requests'),
      },
      auth: {
        type: NavigastionAuthType.Route,
        route: Routes.Proposals,
      },
      icon: mdiFormatListText,
    },
    {
      name: 'address_book',
      localeKey: 'navigation.address_book',
      action: {
        type: NavigationActionType.To,
        handle: route =>
          route.params.locale ? `/${route.params.locale}/address-book` : '/address-book',
      },
      auth: {
        type: NavigastionAuthType.Route,
        route: Routes.AddressBook,
      },
      icon: mdiBookOpenVariant,
    },
  ],
});

export interface NavigationSections {
  main: NavigationItem[];
}

class Navigation {
  constructor(public readonly sections: () => NavigationSections) {}

  install(app: App): void {
    const { sections } = this;

    const navigation = ref<NavigationSections>(sections());
    const session = useSessionStore();
    const store = useWalletStore();
    const userState = computed(() => ({
      user: store.$state.user,
      privileges: store.$state.privileges,
      selectedWallet: session.data.selectedWallet,
    }));

    watch(
      () => userState.value,
      () => {
        const full = sections();
        navigation.value = {
          main: Navigation.retainAuthorizedNavigation(full.main),
        };
      },
      { deep: true },
    );

    app.config.globalProperties.$navigation = navigation;
  }

  private static retainAuthorizedNavigation = (items: NavigationItem[]): NavigationItem[] => {
    return items.filter(item => {
      item.items = this.retainAuthorizedNavigation(item.items || []);

      if (item.auth.type === NavigastionAuthType.Route) {
        const route = this.findRouteByName(item.auth.route);

        if (!route) {
          logger.warn(`Route '${item.auth.route}' not found for navigation item`);
          return false;
        }

        if (!route.meta) {
          logger.warn(`Route '${item.auth.route}' has no meta with the auth configuration`);
          return false;
        }

        const matchesRequiredSession = hasRequiredSession(route.meta.auth.check.session);
        const matchesRequiredPrivilege = hasRequiredPrivilege({
          anyOf: route.meta.auth.check.privileges,
        });

        return matchesRequiredSession && matchesRequiredPrivilege;
      }

      if (item.auth.type === NavigastionAuthType.Custom) {
        const matchesRequiredSession = hasRequiredSession(item.auth.criteria.session);
        const matchesRequiredPrivilege = hasRequiredPrivilege({
          anyOf: item.auth.criteria.privileges,
        });

        return matchesRequiredSession && matchesRequiredPrivilege;
      }

      return false;
    });
  };

  private static findRouteByName = (
    name: Routes,
    routeItems: RouteRecordRaw[] = routes,
  ): RouteRecordRaw | undefined => {
    for (const route of routeItems) {
      if (route.name === name) {
        return route;
      }

      if (route.children) {
        const childRoute = this.findRouteByName(name, route.children);
        if (childRoute) {
          return childRoute;
        }
      }
    }

    return undefined;
  };
}

declare module 'vue' {
  interface ComponentCustomProperties {
    $navigation: Ref<NavigationSections>;
  }
}

export const navigation = new Navigation(sections);
