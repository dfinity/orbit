import { NavigationGuard, RouterView, createRouter, createWebHistory } from 'vue-router';
import { appInitConfig } from '~/configs/init.config';
import { Routes, defaultHomeRoute, defaultLoginRoute } from '~/configs/routes.config';
import { useAppStore } from '~/stores/app.store';
import { useSessionStore } from '~/stores/session.store';
import { Privilege, RequiredSessionState } from '~/types/auth.types';
import { hasRequiredPrivilege, hasRequiredSession } from '~/utils/auth.utils';
import { i18n, i18nRouteGuard } from './i18n.plugin';
import { initStateGuard } from './pinia.plugin';
import { services } from './services.plugin';
import NotFoundPage from '~/pages/NotFoundPage.vue';
import { supportedLocales } from '~/configs/i18n.config';
import LoginPage from '~/pages/LoginPage.vue';
import { ProposalDomains } from '~/types/wallet.types';

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
              component: () => import('~/pages/AccountListPage.vue'),
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
                    {
                      title: i18n.global.t('navigation.accounts'),
                      to: { name: Routes.Accounts },
                    },
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
          path: 'disconnected',
          name: Routes.Disconnected,
          component: () => import('~/pages/DisconnectedPage.vue'),
          meta: {
            auth: {
              check: {
                session: RequiredSessionState.Authenticated,
              },
            },
          },
        },
        {
          path: 'unauthorized',
          name: Routes.Unauthorized,
          component: () => import('~/pages/UnauthorizedPage.vue'),
          meta: {
            auth: {
              check: {
                session: RequiredSessionState.Authenticated,
              },
            },
          },
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
                {
                  title: i18n.global.t('navigation.proposals'),
                  to: { name: Routes.Proposals },
                },
                {
                  title: i18n.global.t('navigation.transfer_proposals'),
                },
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
                        {
                          title: i18n.global.t('navigation.settings'),
                        },
                        {
                          title: i18n.global.t('terms.user_groups'),
                          to: { name: Routes.UserGroups },
                        },
                        {
                          title: i18n.global.t('navigation.access_policies'),
                        },
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
                    {
                      title: i18n.global.t('navigation.settings'),
                    },
                    {
                      title: i18n.global.t('navigation.proposals'),
                    },
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
                    {
                      title: i18n.global.t('navigation.settings'),
                    },
                    {
                      title: i18n.global.t('navigation.proposal_policies'),
                    },
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
          name: Routes.NotFound,
          component: NotFoundPage,
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
    let redirectToRoute = defaultHomeRoute;
    switch (to.meta.auth.check.session) {
      case RequiredSessionState.Authenticated:
        redirectToRoute = defaultLoginRoute;
        break;
      case RequiredSessionState.ConnectedToWallet: {
        redirectToRoute = Routes.Disconnected;
        break;
      }
    }

    return next({ name: redirectToRoute });
  }

  const matchesRequiredPrivilege = hasRequiredPrivilege({ anyOf: to.meta.auth.check.privileges });
  if (!matchesRequiredPrivilege) {
    return next({ name: Routes.Unauthorized });
  }

  return next();
};

router.beforeEach(initStateGuard);
router.beforeEach(i18nRouteGuard(services(), () => useAppStore()));
router.beforeEach(routeAccessGuard);

export { router };
