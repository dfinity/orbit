import { RouteRecordRaw, RouterView } from 'vue-router';
import { Privilege, RequiredSessionState } from '~/types/auth.types';
import { ProposalDomains } from '~/types/wallet.types';
import { i18n } from '~/ui/modules';
import { services } from '~/ui/modules/services';
import DisconnectedPage from '~/ui/pages/DisconnectedPage.vue';
import InitializationPage from '~/ui/pages/InitializationPage.vue';
import LoginPageVue from '~/ui/pages/LoginPage.vue';
import NotFoundPageVue from '~/ui/pages/NotFoundPage.vue';
import UnauthorizedPageVue from '~/ui/pages/UnauthorizedPage.vue';

export enum Routes {
  Login = 'Login',
  NotFound = 'NotFound',
  Accounts = 'Accounts',
  Account = 'Account',
  MySettings = 'MySettings',
  UserGroups = 'UserGroups',
  SystemSettings = 'SystemSettings',
  Disconnected = 'Disconnected',
  Unauthorized = 'Unauthorized',
  Users = 'Users',
  AddressBook = 'AddressBook',
  Initialization = 'Initialization',
  AccessPolicies = 'AccessPolicies',
  // Proposal Pages
  Proposals = 'Proposals',
  TransferProposals = 'TransferProposals',
}

export const defaultLoginRoute = Routes.Login;
export const defaultHomeRoute = Routes.Accounts;

export const routes: RouteRecordRaw[] = [
  {
    path: `/:locale(${services().locales.supportedLocales.join('|')})?`,
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
        component: LoginPageVue,
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
            component: () => import('~/ui/pages/AccountListPage.vue'),
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
            component: () => import('~/ui/pages/AccountPage.vue'),
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
        component: DisconnectedPage,
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
        component: UnauthorizedPageVue,
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
        component: InitializationPage,
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
        component: () => import('~/ui/pages/ProposalsPage.vue'),
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
        component: () => import('~/ui/pages/MySettingsPage.vue'),
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
            component: () => import('~/ui/pages/AdministrationPage.vue'),
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
                component: () => import('~/ui/pages/UserGroupsPage.vue'),
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
                component: () => import('~/ui/pages/AccessPoliciesPage.vue'),
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
            component: () => import('~/ui/pages/UsersPage.vue'),
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
            component: () => import('~/ui/pages/ProposalsPage.vue'),
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
        ],
      },
      {
        path: 'address-book',
        name: Routes.AddressBook,
        component: () => import('~/ui/pages/AddressBookPage.vue'),
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
        component: NotFoundPageVue,
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
];
