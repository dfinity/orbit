import { NavigationGuard, RouterView, createRouter, createWebHistory } from 'vue-router';
import { supportedLocales } from '~/configs/i18n.config';
import { appInitConfig } from '~/configs/init.config';
import {
  RouteStatusCode,
  Routes,
  defaultHomeRoute,
  defaultLoginRoute,
} from '~/configs/routes.config';
import logger from '~/core/logger.core';
import ErrorPage from '~/pages/ErrorPage.vue';
import LoginPage from '~/pages/LoginPage.vue';
import { useAppStore } from '~/stores/app.store';
import { useSessionStore } from '~/stores/session.store';
import { Privilege, RequiredSessionState } from '~/types/auth.types';
import { RequestDomains } from '~/types/station.types';
import { hasRequiredPrivilege, hasRequiredSession } from '~/utils/auth.utils';
import { i18n, i18nRouteGuard } from './i18n.plugin';
import { initStateGuard } from './pinia.plugin';
import { services } from './services.plugin';

export const redirectToKey = 'redirectTo';

const router = createRouter({
  history: createWebHistory(appInitConfig.versionedBaseUrl),
  routes: [
    {
      path: `/:locale(${supportedLocales.join('|')})?`,
      component: RouterView,
      children: [
        {
          path: '',
          redirect: {
            name: defaultHomeRoute,
          },
          meta: {
            auth: {
              check: {
                session: RequiredSessionState.Any,
              },
            },
          },
        },
        {
          path: 'login',
          name: Routes.Login,
          component: LoginPage,
          meta: {
            auth: {
              check: {
                session: RequiredSessionState.Guest,
              },
            },
          },
        },
        {
          path: 'accounts',
          component: RouterView,
          meta: {
            auth: {
              check: {
                session: RequiredSessionState.ConnectedToStation,
              },
            },
          },
          children: [
            {
              path: '',
              name: Routes.Accounts,
              component: () => import('~/pages/AccountsPage.vue'),
              props: () => {
                return {
                  breadcrumbs: [
                    { title: i18n.global.t('navigation.home'), to: { name: defaultHomeRoute } },
                    { title: i18n.global.t('navigation.accounts') },
                  ],
                };
              },
              meta: {
                auth: {
                  check: {
                    session: RequiredSessionState.ConnectedToStation,
                    privileges: [Privilege.ListAccounts],
                  },
                },
              },
            },
            {
              path: ':id',
              component: RouterView,
              meta: {
                auth: {
                  check: {
                    session: RequiredSessionState.ConnectedToStation,
                    privileges: [Privilege.ListAccounts],
                  },
                },
              },
              children: [
                {
                  path: '',
                  name: Routes.Account,
                  component: () => import('~/pages/AccountPage.vue'),
                  props: () => {
                    return {
                      breadcrumbs: [
                        { title: i18n.global.t('navigation.home'), to: { name: defaultHomeRoute } },
                        {
                          title: i18n.global.t('navigation.accounts'),
                          to: { name: Routes.Accounts },
                        },
                      ],
                    };
                  },
                },
                {
                  path: ':assetId',
                  name: Routes.AccountAsset,
                  component: () => import('~/pages/AccountAssetPage.vue'),
                  props: params => {
                    return {
                      breadcrumbs: [
                        { title: i18n.global.t('navigation.home'), to: { name: defaultHomeRoute } },
                        {
                          title: i18n.global.t('navigation.accounts'),
                          to: { name: Routes.Accounts },
                        },
                        {
                          title: i18n.global.t('navigation.account'),
                          to: { name: Routes.Account, params: { id: params.params.id } },
                        },
                      ],
                    };
                  },
                },
              ],
            },
          ],
        },
        {
          path: 'initialization',
          name: Routes.Initialization,
          component: () => import('~/pages/InitializationPage.vue'),
          meta: {
            auth: {
              check: {
                session: RequiredSessionState.AuthenticatedNoStation,
              },
            },
          },
        },
        {
          path: 'add-station',
          name: Routes.AddStation,
          component: () => import('~/pages/AddStationPage.vue'),
          meta: {
            auth: {
              check: {
                session: RequiredSessionState.Authenticated,
              },
            },
          },
        },
        {
          path: 'transfer-requests',
          name: Routes.TransferRequests,
          component: () => import('~/pages/RequestsPage.vue'),
          props: () => {
            return {
              title: i18n.global.t('pages.requests.transfer_title'),
              domains: [RequestDomains.Transfers],
              breadcrumbs: [
                { title: i18n.global.t('navigation.home'), to: { name: defaultHomeRoute } },
                { title: i18n.global.t('navigation.transfer_requests') },
              ],
            };
          },
          meta: {
            auth: {
              check: {
                session: RequiredSessionState.ConnectedToStation,
                privileges: [Privilege.ListAccounts],
              },
            },
          },
        },
        {
          path: 'canisters',
          component: RouterView,
          meta: {
            auth: {
              check: {
                session: RequiredSessionState.ConnectedToStation,
              },
            },
          },
          children: [
            {
              path: '',
              name: Routes.ExternalCanisters,
              component: () => import('~/pages/ExternalCanisterListPage.vue'),
              props: () => {
                return {
                  breadcrumbs: [
                    { title: i18n.global.t('navigation.home'), to: { name: defaultHomeRoute } },
                    { title: i18n.global.t('navigation.external_canisters') },
                  ],
                };
              },
              meta: {
                auth: {
                  check: {
                    session: RequiredSessionState.ConnectedToStation,
                    privileges: [Privilege.ListExternalCanisters],
                  },
                },
              },
            },
            {
              path: ':cid',
              name: Routes.ExternalCanister,
              component: () => import('~/pages/ExternalCanisterDetailPage.vue'),
              props: () => {
                return {
                  breadcrumbs: [
                    { title: i18n.global.t('navigation.home'), to: { name: defaultHomeRoute } },
                    {
                      title: i18n.global.t('navigation.external_canisters'),
                      to: { name: Routes.ExternalCanisters },
                    },
                  ],
                };
              },
              meta: {
                auth: {
                  check: {
                    session: RequiredSessionState.ConnectedToStation,
                    privileges: [Privilege.ListExternalCanisters],
                  },
                },
              },
            },
          ],
        },
        {
          path: 'my-settings',
          name: Routes.MySettings,
          component: () => import('~/pages/MySettingsPage.vue'),
          props: () => {
            return {
              breadcrumbs: [
                { title: i18n.global.t('navigation.home'), to: { name: defaultHomeRoute } },
                { title: i18n.global.t('navigation.account_info_settings') },
              ],
            };
          },
          meta: {
            auth: {
              check: {
                session: RequiredSessionState.Authenticated,
              },
            },
          },
        },
        {
          path: 'settings',
          component: RouterView,
          meta: {
            auth: {
              check: {
                session: RequiredSessionState.ConnectedToStation,
              },
            },
          },
          children: [
            {
              path: 'system',
              name: Routes.SystemSettings,
              component: () => import('~/pages/AdministrationPage.vue'),
              props: () => {
                return {
                  breadcrumbs: [
                    { title: i18n.global.t('navigation.home'), to: { name: defaultHomeRoute } },
                    { title: i18n.global.t('navigation.settings') },
                    { title: i18n.global.t('navigation.administration') },
                  ],
                };
              },
              meta: {
                auth: {
                  check: {
                    session: RequiredSessionState.Authenticated,
                  },
                },
              },
            },
            {
              path: 'user-groups',
              component: RouterView,
              meta: {
                auth: {
                  check: {
                    session: RequiredSessionState.ConnectedToStation,
                    privileges: [Privilege.ListUserGroups, Privilege.ListPermissions],
                  },
                },
              },
              children: [
                {
                  path: '',
                  name: Routes.UserGroups,
                  component: () => import('~/pages/UserGroupsPage.vue'),
                  props: () => {
                    return {
                      breadcrumbs: [
                        { title: i18n.global.t('navigation.home'), to: { name: defaultHomeRoute } },
                        { title: i18n.global.t('navigation.settings') },
                        { title: i18n.global.t('navigation.user_groups_permissions') },
                      ],
                    };
                  },
                  meta: {
                    auth: {
                      check: {
                        session: RequiredSessionState.ConnectedToStation,
                        privileges: [Privilege.ListUserGroups],
                      },
                    },
                  },
                },
                {
                  path: 'permissions',
                  name: Routes.Permissions,
                  component: () => import('~/pages/PermissionsPage.vue'),
                  props: () => {
                    return {
                      breadcrumbs: [
                        { title: i18n.global.t('navigation.home'), to: { name: defaultHomeRoute } },
                        { title: i18n.global.t('navigation.settings') },
                        {
                          title: i18n.global.t('terms.user_groups'),
                          to: { name: Routes.UserGroups },
                        },
                        { title: i18n.global.t('navigation.permissions') },
                      ],
                    };
                  },
                  meta: {
                    auth: {
                      check: {
                        session: RequiredSessionState.ConnectedToStation,
                        privileges: [Privilege.ListPermissions],
                      },
                    },
                  },
                },
              ],
            },
            {
              path: 'users',
              name: Routes.Users,
              component: () => import('~/pages/UsersPage.vue'),
              props: () => {
                return {
                  breadcrumbs: [
                    { title: i18n.global.t('navigation.home'), to: { name: defaultHomeRoute } },
                    { title: i18n.global.t('navigation.settings') },
                    { title: i18n.global.t('navigation.users') },
                  ],
                };
              },
              meta: {
                auth: {
                  check: {
                    session: RequiredSessionState.ConnectedToStation,
                    privileges: [Privilege.ListUsers],
                  },
                },
              },
            },
            {
              path: 'requests',
              name: Routes.Requests,
              component: () => import('~/pages/RequestsPage.vue'),
              props: () => {
                return {
                  title: i18n.global.t('pages.requests.title'),
                  breadcrumbs: [
                    { title: i18n.global.t('navigation.home'), to: { name: defaultHomeRoute } },
                    { title: i18n.global.t('navigation.settings') },
                    { title: i18n.global.t('navigation.requests') },
                  ],
                };
              },
              meta: {
                auth: {
                  check: {
                    session: RequiredSessionState.ConnectedToStation,
                    privileges: [Privilege.ListRequests],
                  },
                },
              },
            },
            {
              path: 'policies',
              name: Routes.RequestPolicies,
              component: () => import('~/pages/RequestPoliciesPage.vue'),
              props: () => {
                return {
                  title: i18n.global.t('pages.request_policies.title'),
                  breadcrumbs: [
                    { title: i18n.global.t('navigation.home'), to: { name: defaultHomeRoute } },
                    { title: i18n.global.t('navigation.settings') },
                    { title: i18n.global.t('navigation.request_policies') },
                  ],
                };
              },
              meta: {
                auth: {
                  check: {
                    session: RequiredSessionState.ConnectedToStation,
                    privileges: [Privilege.ListRequestPolicies],
                  },
                },
              },
            },
            {
              path: 'assets',
              name: Routes.Assets,
              component: () => import('~/pages/AssetsPage.vue'),
              props: () => {
                return {
                  title: i18n.global.t('pages.assets.title'),
                  breadcrumbs: [
                    { title: i18n.global.t('navigation.home'), to: { name: defaultHomeRoute } },
                    { title: i18n.global.t('navigation.settings') },
                    { title: i18n.global.t('navigation.assets') },
                  ],
                };
              },
              meta: {
                auth: {
                  check: {
                    session: RequiredSessionState.ConnectedToStation,
                    privileges: [Privilege.ListAssets],
                  },
                },
              },
            },
          ],
        },
        {
          path: 'address-book',
          name: Routes.AddressBook,
          component: () => import('~/pages/AddressBookPage.vue'),
          props: () => {
            return {
              breadcrumbs: [
                {
                  title: i18n.global.t('navigation.home'),
                  to: { name: defaultHomeRoute },
                },
                {
                  title: i18n.global.t('navigation.address_book'),
                },
              ],
            };
          },
          meta: {
            auth: {
              check: {
                session: RequiredSessionState.ConnectedToStation,
                privileges: [Privilege.ListAddressBookEntries],
              },
            },
          },
        },
        {
          path: ':pathMatch(.*)*',
          name: Routes.Error,
          component: ErrorPage,
          meta: {
            auth: {
              check: {
                session: RequiredSessionState.Any,
              },
            },
          },
        },
      ],
    },
  ],
});

export const routeAccessGuard: NavigationGuard = async (to, _from, next) => {
  const session = useSessionStore();
  const app = useAppStore();

  if (to.name === Routes.Disconnected && session.data.selected.hasAccess) {
    return next({ name: defaultHomeRoute });
  }

  if (to.name === Routes.Initialization && (!session.isAuthenticated || session.hasStations)) {
    return next({ name: defaultHomeRoute });
  }

  if (
    to.name &&
    ![Routes.Initialization.toString(), Routes.MySettings.toString()].includes(
      to.name.toString(),
    ) &&
    session.isAuthenticated &&
    !session.hasStations
  ) {
    return next({ name: Routes.Initialization });
  }

  const matchesRequiredSession = hasRequiredSession(to.meta.auth.check.session);
  if (!matchesRequiredSession) {
    switch (to.meta.auth.check.session) {
      case RequiredSessionState.Authenticated:
        // save the current route to redirect back after login
        sessionStorage.setItem(redirectToKey, to.fullPath);
        return next({ name: defaultLoginRoute });
      case RequiredSessionState.ConnectedToStation: {
        if (!session.isAuthenticated) {
          // save the current route to redirect back after login
          sessionStorage.setItem(redirectToKey, to.fullPath);
          return next({ name: defaultLoginRoute });
        }

        app.routeStatusCode = RouteStatusCode.Disconnected;
        return next();
      }
      default: {
        return next({ name: defaultHomeRoute });
      }
    }
  }

  const matchesRequiredPrivilege = hasRequiredPrivilege({ anyOf: to.meta.auth.check.privileges });
  if (!matchesRequiredPrivilege) {
    app.routeStatusCode = RouteStatusCode.Unauthorized;
    return next();
  }

  return next();
};

// needs to be the first guard to begin the loading state of the app routing
router.beforeEach((_, _from, next) => {
  const app = useAppStore();
  app.loading = true;
  app.routeStatusCode = RouteStatusCode.Success;

  return next();
});

router.beforeEach(initStateGuard);
router.beforeEach(i18nRouteGuard(services(), () => useAppStore()));
router.beforeEach(routeAccessGuard);

// needs to be the last guard to end the loading state of the app routing
router.afterEach((_to, _from) => {
  const app = useAppStore();
  app.loading = false;
});

router.onError(error => {
  logger.error(`Router error`, { error });

  const app = useAppStore();
  app.loading = false;
});

export { router };
