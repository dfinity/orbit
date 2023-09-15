import { RouterView, createRouter, createWebHistory } from 'vue-router';
import HomePage from '~/ui/pages/HomePage.vue';
import LoginPage from '~/ui/pages/LoginPage.vue';
import NotFoundPage from '~/ui/pages/NotFoundPage.vue';
import { i18nRouteGuard } from './I18n';
import { services } from './ServiceManager';

const router = createRouter({
  history: createWebHistory(services().routes.baseUrl),
  routes: [
    {
      path: `/:locale(${services().locales.supportedLocales.join('|')})?`,
      component: RouterView,
      children: [
        { path: '', name: 'home', alias: ['home'], component: HomePage },
        { path: 'login', name: 'login', component: LoginPage },
        { path: ':pathMatch(.*)*', name: 'NotFound', component: NotFoundPage },
      ],
    },
  ],
});

router.beforeEach(i18nRouteGuard);

export { router };
