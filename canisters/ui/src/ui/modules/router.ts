import { RouterView, createRouter, createWebHistory } from 'vue-router';
import { initStateGuard, navigationGuard } from '~/ui/modules';
import OverviewPage from '~/ui/pages/OverviewPage.vue';
import LoginPage from '~/ui/pages/LoginPage.vue';
import NotFoundPage from '~/ui/pages/NotFoundPage.vue';
import { AuthState } from '~/ui/types';
import { i18nRouteGuard } from './i18n';
import { services } from './services';

export const redirectToKey = 'redirectTo';
export const defaultLoginRoute = 'Login';
export const defaultHomeRoute = 'Home';

const router = createRouter({
  history: createWebHistory(services().routes.baseUrl),
  routes: [
    {
      path: `/:locale(${services().locales.supportedLocales.join('|')})?`,
      component: RouterView,
      children: [
        {
          path: '',
          name: defaultHomeRoute,
          alias: ['overview'],
          component: OverviewPage,
          meta: {
            auth: {
              requireState: AuthState.Authenticated,
            },
          },
        },
        {
          path: 'login',
          name: defaultLoginRoute,
          component: LoginPage,
          meta: {
            auth: {
              requireState: AuthState.Guest,
            },
          },
        },
        {
          path: 'accounts',
          name: 'AccountsRouter',
          component: RouterView,
          meta: {
            auth: {
              requireState: AuthState.Authenticated,
            },
          },
          children: [
            {
              path: '',
              name: 'AccountList',
              component: () => import('~/ui/pages/AccountListPage.vue'),
              meta: {
                auth: {
                  requireState: AuthState.Authenticated,
                },
              },
            },
            {
              path: ':id',
              name: 'Account',
              component: () => import('~/ui/pages/AccountPage.vue'),
              meta: {
                auth: {
                  requireState: AuthState.Authenticated,
                },
              },
            },
          ],
        },
        {
          path: 'transfers',
          name: 'Transfers',
          component: () => import('~/ui/pages/TransfersPage.vue'),
          meta: {
            auth: {
              requireState: AuthState.Authenticated,
            },
          },
        },
        {
          path: 'my-settings',
          name: 'MySettings',
          component: () => import('~/ui/pages/MySettingsPage.vue'),
          meta: {
            auth: {
              requireState: AuthState.Authenticated,
            },
          },
        },
        {
          path: 'settings',
          name: 'SettingsRouter',
          component: RouterView,
          meta: {
            auth: {
              requireState: AuthState.Authenticated,
            },
          },
          children: [
            {
              path: 'system',
              name: 'SystemSettings',
              component: () => import('~/ui/pages/AdministrationPage.vue'),
              meta: {
                auth: {
                  requireState: AuthState.Authenticated,
                },
              },
            },
          ],
        },
        {
          path: 'connect',
          name: 'Connect',
          component: () => import('~/ui/pages/ConnectPage.vue'),
          meta: {
            auth: {
              requireState: AuthState.Authenticated,
            },
          },
        },
        {
          path: ':pathMatch(.*)*',
          name: 'NotFound',
          component: NotFoundPage,
          meta: {
            auth: {
              requireState: AuthState.Any,
            },
          },
        },
      ],
    },
  ],
});

router.beforeEach(initStateGuard);
router.beforeEach(i18nRouteGuard);
router.beforeEach(navigationGuard);

export { router };
