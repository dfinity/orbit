import { NavigationGuard, RouterView, createRouter, createWebHistory } from 'vue-router';
import { supportedLocales } from '~/configs/i18n.config';
import { appInitConfig } from '~/configs/init.config';
import {
  RouteStatusCode,
  Routes,
  defaultHomeRoute,
  defaultLoginRoute,
} from '~/configs/routes.config';
import ErrorPage from '~/pages/ErrorPage.vue';
import LoginPage from '~/pages/LoginPage.vue';
import { useAppStore } from '~/stores/app.store';
import { useSessionStore } from '~/stores/session.store';
import { Privilege, RequiredSessionState } from '~/types/auth.types';
import { ProposalDomains } from '~/types/wallet.types';
import { hasRequiredPrivilege, hasRequiredSession } from '~/utils/auth.utils';
import { i18n, i18nRouteGuard } from './i18n.plugin';
import { initStateGuard } from './pinia.plugin';
import { services } from './services.plugin';
import logger from '~/core/logger.core';

export const redirectToKey = 'redirectTo';

const router = createRouter({
  history: createWebHistory(appInitConfig.baseUrl),
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
                session: RequiredSessionState.ConnectedToWallet,
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
                session: RequiredSessionState.ConnectedToWallet,
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
                    session: RequiredSessionState.ConnectedToWallet,
                    privileges: [Privilege.ListAccounts],
                  },
                },
              },
            },
            {
              path: ':id',
              name: Routes.Account,
              component: () => import('~/pages/AccountPage.vue'),
              props: () => {
                return {
                  breadcrumbs: [
                    { title: i18n.global.t('navigation.home'), to: { name: defaultHomeRoute } },
                    { title: i18n.global.t('navigation.accounts'), to: { name: Routes.Accounts } },
                  ],
                };
              },
              meta: {
                auth: {
                  check: {
                    session: RequiredSessionState.ConnectedToWallet,
                    privileges: [Privilege.ListAccounts],
                  },
                },
              },
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
                session: RequiredSessionState.Authenticated,
              },
            },
          },
        },
        {
          path: 'transfer-requests',
          name: Routes.TransferProposals,
          component: () => import('~/pages/ProposalsPage.vue'),
          props: () => {
            return {
              title: i18n.global.t('pages.proposals.transfer_title'),
              domains: [ProposalDomains.Transfers],
              breadcrumbs: [
                { title: i18n.global.t('navigation.home'), to: { name: defaultHomeRoute } },
                { title: i18n.global.t('navigation.transfer_proposals') },
              ],
            };
          },
          meta: {
            auth: {
              check: {
                session: RequiredSessionState.ConnectedToWallet,
                privileges: [Privilege.ListAccounts],
              },
            },
          },
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
                session: RequiredSessionState.ConnectedToWallet,
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
                    session: RequiredSessionState.ConnectedToWallet,
                    privileges: [Privilege.ListUserGroups, Privilege.ListAccessPolicies],
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
                        session: RequiredSessionState.ConnectedToWallet,
                        privileges: [Privilege.ListUserGroups],
                      },
                    },
                  },
                },
                {
                  path: 'permissions',
                  name: Routes.AccessPolicies,
                  component: () => import('~/pages/AccessPoliciesPage.vue'),
                  props: () => {
                    return {
                      breadcrumbs: [
                        { title: i18n.global.t('navigation.home'), to: { name: defaultHomeRoute } },
                        { title: i18n.global.t('navigation.settings') },
                        {
                          title: i18n.global.t('terms.user_groups'),
                          to: { name: Routes.UserGroups },
                        },
                        { title: i18n.global.t('navigation.access_policies') },
                      ],
                    };
                  },
                  meta: {
                    auth: {
                      check: {
                        session: RequiredSessionState.ConnectedToWallet,
                        privileges: [Privilege.ListAccessPolicies],
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
                    session: RequiredSessionState.ConnectedToWallet,
                    privileges: [Privilege.ListUsers],
                  },
                },
              },
            },
            {
              path: 'requests',
              name: Routes.Proposals,
              component: () => import('~/pages/ProposalsPage.vue'),
              props: () => {
                return {
                  title: i18n.global.t('pages.proposals.title'),
                  breadcrumbs: [
                    { title: i18n.global.t('navigation.home'), to: { name: defaultHomeRoute } },
                    { title: i18n.global.t('navigation.settings') },
                    { title: i18n.global.t('navigation.proposals') },
                  ],
                };
              },
              meta: {
                auth: {
                  check: {
                    session: RequiredSessionState.ConnectedToWallet,
                  },
                },
              },
            },
            {
              path: 'policies',
              name: Routes.ProposalPolicies,
              component: () => import('~/pages/ProposalPoliciesPage.vue'),
              props: () => {
                return {
                  title: i18n.global.t('pages.proposal_policies.title'),
                  breadcrumbs: [
                    { title: i18n.global.t('navigation.home'), to: { name: defaultHomeRoute } },
                    { title: i18n.global.t('navigation.settings') },
                    { title: i18n.global.t('navigation.proposal_policies') },
                  ],
                };
              },
              meta: {
                auth: {
                  check: {
                    session: RequiredSessionState.ConnectedToWallet,
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
                session: RequiredSessionState.ConnectedToWallet,
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

  if (to.name === Routes.Disconnected && session.data.selectedWallet.hasAccess) {
    return next({ name: defaultHomeRoute });
  }

  if (to.name === Routes.Initialization && (!session.isAuthenticated || session.hasWallets)) {
    return next({ name: defaultHomeRoute });
  }

  if (to.name !== Routes.Initialization && session.isAuthenticated && !session.hasWallets) {
    return next({ name: Routes.Initialization });
  }

  const matchesRequiredSession = hasRequiredSession(to.meta.auth.check.session);
  if (!matchesRequiredSession) {
    switch (to.meta.auth.check.session) {
      case RequiredSessionState.Authenticated:
        return next({ name: defaultLoginRoute });
      case RequiredSessionState.ConnectedToWallet: {
        if (!session.isAuthenticated) {
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
