import { RouterView, createRouter, createWebHistory } from 'vue-router';
import { initStateGuard, navigationGuard } from '~/ui/modules';
import HomePage from '~/ui/pages/HomePage.vue';
import LoginPage from '~/ui/pages/LoginPage.vue';
import NotFoundPage from '~/ui/pages/NotFoundPage.vue';
import { AuthState } from '~/ui/types';
import { i18nRouteGuard } from './I18n';
import { services } from './ServiceManager';

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
          alias: ['home'],
          component: HomePage,
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
          path: 'wallets',
          name: 'Wallets',
          component: () => import('~/ui/pages/WalletsPage.vue'),
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
