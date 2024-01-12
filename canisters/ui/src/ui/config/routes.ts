import { RouteRecordRaw, RouterView } from 'vue-router';
import { Privilege } from '~/types';
import { services } from '~/ui/modules';
import DisconnectedPageVue from '~/ui/pages/DisconnectedPage.vue';
import LoginPageVue from '~/ui/pages/LoginPage.vue';
import NotFoundPageVue from '~/ui/pages/NotFoundPage.vue';
import OverviewPageVue from '~/ui/pages/OverviewPage.vue';
import UnauthorizedPageVue from '~/ui/pages/UnauthorizedPage.vue';
import { RequiredSessionState } from '~/ui/types';

export enum Routes {
  Login = 'Login',
  Overview = 'Overview',
  NotFound = 'NotFound',
  Accounts = 'Accounts',
  Account = 'Account',
  MySettings = 'MySettings',
  UserGroups = 'UserGroups',
  SystemSettings = 'SystemSettings',
  Connect = 'Connect',
  Disconnected = 'Disconnected',
  Unauthorized = 'Unauthorized',
  Users = 'Users',
  AddressBook = 'AddressBook',
  AddressBookSettings = 'AddressBookSettings',
}

export const defaultLoginRoute = Routes.Login;
export const defaultHomeRoute = Routes.Overview;

export const routes: RouteRecordRaw[] = [
  {
    path: `/:locale(${services().locales.supportedLocales.join('|')})?`,
    component: RouterView,
    children: [
      {
        path: '',
        name: Routes.Overview,
        alias: ['overview'],
        component: OverviewPageVue,
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
        path: 'disconnected',
        name: Routes.Disconnected,
        component: DisconnectedPageVue,
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
                  session: RequiredSessionState.ConnectedToWallet,
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
          {
            path: 'address-book-settings',
            name: Routes.AddressBookSettings,
            component: () => import('~/ui/pages/AddressBookSettingsPage.vue'),
            meta: {
              auth: {
                check: {
                  session: RequiredSessionState.ConnectedToWallet,
                  // todo: add privilege to manage address book when available
                  privileges: [],
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
              // todo: add privilege to access address book when available
              privileges: [],
            },
          },
        },
      },
      {
        path: 'connect',
        name: Routes.Connect,
        component: () => import('~/ui/pages/ConnectPage.vue'),
        meta: {
          auth: {
            check: {
              session: RequiredSessionState.Authenticated,
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
