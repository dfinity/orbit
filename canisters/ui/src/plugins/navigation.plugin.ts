import { mdiBookOpenVariant, mdiCogs, mdiFormatListText, mdiWalletBifold } from '@mdi/js';
import { App, Ref, computed, ref, watch } from 'vue';
import { RouteRecordRaw } from 'vue-router';
import { Routes } from '~/configs/routes.config';
import { logger } from '~/core/logger.core';
import { useSessionStore } from '~/stores/session.store';
import { useWalletStore } from '~/stores/wallet.store';
import {
  NavigastionAuthType,
  NavigationActionType,
  NavigationItem,
} from '~/types/navigation.types';
import { hasRequiredPrivilege, hasRequiredSession } from '~/utils/auth.utils';

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
          name: 'proposals',
          localeKey: 'navigation.proposals',
          action: {
            type: NavigationActionType.To,
            handle: route =>
              route.params.locale
                ? `/${route.params.locale}/settings/requests`
                : '/settings/requests',
          },
          auth: {
            type: NavigastionAuthType.Route,
            route: Routes.Proposals,
          },
        },
      ],
    },
    {
      name: 'accounts',
      localeKey: 'navigation.accounts',
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
      name: 'transfer_proposals',
      localeKey: 'navigation.transfer_proposals',
      action: {
        type: NavigationActionType.To,
        handle: route =>
          route.params.locale ? `/${route.params.locale}/transfer-requests` : '/transfer-requests',
      },
      auth: {
        type: NavigastionAuthType.Route,
        route: Routes.TransferProposals,
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
  constructor(
    public sections: () => NavigationSections,
    public routes: RouteRecordRaw[] = [],
  ) {}

  withSections(sections: NavigationSections): Navigation {
    this.sections = () => sections;
    return this;
  }

  withRoutes(routes: RouteRecordRaw[]): Navigation {
    this.routes = routes;
    return this;
  }

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
          main: this.retainAuthorizedNavigation(full.main),
        };
      },
      { deep: true },
    );

    app.config.globalProperties.$navigation = navigation;
  }

  private retainAuthorizedNavigation = (items: NavigationItem[]): NavigationItem[] => {
    return items.filter(item => {
      item.items = this.retainAuthorizedNavigation(item.items || []);

      if (item.auth.type === NavigastionAuthType.Route) {
        const route = this.findRouteByName(item.auth.route, this.routes);

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

  private findRouteByName = (
    name: Routes,
    routes: RouteRecordRaw[],
  ): RouteRecordRaw | undefined => {
    for (const route of routes) {
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
