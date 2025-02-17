import {
  mdiBookOpenVariant,
  mdiCogs,
  mdiDatabase,
  mdiFormatListText,
  mdiPlusBox,
  mdiViewDashboard,
  mdiWalletBifold,
} from '@mdi/js';
import { App, Ref, computed, ref, watch } from 'vue';
import { RouteRecordRaw } from 'vue-router';
import { Routes } from '~/configs/routes.config';
import { logger } from '~/core/logger.core';
import { useSessionStore } from '~/stores/session.store';
import { useStationStore } from '~/stores/station.store';
import { RequiredSessionState } from '~/types/auth.types';
import {
  NavigastionAuthType,
  NavigationActionType,
  NavigationItem,
} from '~/types/navigation.types';
import { hasRequiredPrivilege, hasRequiredSession } from '~/utils/auth.utils';

const sections = (): NavigationSections => ({
  main: [
    {
      name: 'initialization',
      localeKey: 'stations.add_station_list_item',
      action: {
        type: NavigationActionType.To,
        handle: route =>
          route.params.locale ? `/${route.params.locale}/initialization` : '/initialization',
      },
      auth: {
        type: NavigastionAuthType.Custom,
        required: {
          session: RequiredSessionState.AuthenticatedNoStation,
        },
      },
      icon: mdiPlusBox,
    },
    {
      name: 'dashboard',
      localeKey: 'navigation.dashboard',
      action: {
        type: NavigationActionType.To,
        handle: route => (route.params.locale ? `/${route.params.locale}/dashboard` : '/dashboard'),
      },
      auth: {
        type: NavigastionAuthType.Route,
        route: Routes.Dashboard,
      },
      icon: mdiViewDashboard,
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
      name: 'transfer_requests',
      localeKey: 'navigation.transfer_requests',
      action: {
        type: NavigationActionType.To,
        handle: route =>
          route.params.locale ? `/${route.params.locale}/transfer-requests` : '/transfer-requests',
      },
      auth: {
        type: NavigastionAuthType.Route,
        route: Routes.TransferRequests,
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
    {
      name: 'external_canisters',
      localeKey: 'navigation.external_canisters',
      action: {
        type: NavigationActionType.To,
        handle: route => (route.params.locale ? `/${route.params.locale}/canisters` : '/canisters'),
      },
      auth: {
        type: NavigastionAuthType.Route,
        route: Routes.ExternalCanisters,
      },
      icon: mdiDatabase,
    },
    {
      name: 'settings',
      localeKey: 'navigation.settings',
      action: {
        type: NavigationActionType.None,
      },
      icon: mdiCogs,
      auth: {
        type: NavigastionAuthType.Custom,
        required: {
          session: RequiredSessionState.AuthenticatedHasStations,
        },
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
          name: 'policies',
          localeKey: 'navigation.policies',
          action: {
            type: NavigationActionType.To,
            handle: route =>
              route.params.locale
                ? `/${route.params.locale}/settings/policies`
                : '/settings/policies',
          },
          auth: {
            type: NavigastionAuthType.Route,
            route: Routes.Policies,
          },
        },

        {
          name: 'requests',
          localeKey: 'navigation.requests',
          action: {
            type: NavigationActionType.To,
            handle: route =>
              route.params.locale
                ? `/${route.params.locale}/settings/requests`
                : '/settings/requests',
          },
          auth: {
            type: NavigastionAuthType.Route,
            route: Routes.Requests,
          },
        },
        {
          name: 'assets',
          localeKey: 'navigation.assets',
          action: {
            type: NavigationActionType.To,
            handle: route =>
              route.params.locale ? `/${route.params.locale}/settings/assets` : '/settings/assets',
          },
          auth: {
            type: NavigastionAuthType.Route,
            route: Routes.Assets,
          },
        },
      ],
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
    const store = useStationStore();
    const userState = computed(() => ({
      user: store.$state.user,
      privileges: store.$state.privileges,
      selectedStation: session.data.selected,
    }));

    watch(
      () => userState.value,
      () => {
        const full = sections();
        navigation.value = {
          main: this.retainAuthorizedNavigation(full.main),
        };
      },
      { deep: true, immediate: true },
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
        const matchesRequiredSession = hasRequiredSession(item.auth.required.session);
        const matchesRequiredPrivilege = hasRequiredPrivilege({
          anyOf: item.auth.required.privileges,
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
