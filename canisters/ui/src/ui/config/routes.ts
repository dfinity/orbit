import { RouteRecordRaw, RouterView } from 'vue-router';
import { Privilege } from '~/types';
import { services } from '~/ui/modules/services';
import DisconnectedPage from '~/ui/pages/DisconnectedPage.vue';
import InitializationPage from '~/ui/pages/InitializationPage.vue';
import LoginPageVue from '~/ui/pages/LoginPage.vue';
import NotFoundPageVue from '~/ui/pages/NotFoundPage.vue';
import UnauthorizedPageVue from '~/ui/pages/UnauthorizedPage.vue';
import { RequiredSessionState } from '~/ui/types';

export enum Routes {
  Login = 'Login',
  NotFound = 'NotFound',
  Accounts = 'Accounts',
  Account = 'Account',
  MySettings = 'MySettings',
  Proposals = 'Proposals',
  UserGroups = 'UserGroups',
  SystemSettings = 'SystemSettings',
  Disconnected = 'Disconnected',
  Unauthorized = 'Unauthorized',
  Users = 'Users',
  AddressBook = 'AddressBook',
  Initialization = 'Initialization',
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
        path: 'requests',
        name: Routes.Proposals,
        component: () => import('~/ui/pages/ProposalsPage.vue'),
        meta: {
          auth: {
            check: {
              session: RequiredSessionState.ConnectedToWallet,
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
