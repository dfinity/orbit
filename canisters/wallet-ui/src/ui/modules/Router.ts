import { RouterView, createRouter, createWebHistory } from 'vue-router';
import AboutPage from '~/ui/components/AboutPage.vue';
import HomePage from '~/ui/components/HomePage.vue';
import NotFoundPage from '~/ui/components/NotFoundPage.vue';
import { services } from './ServiceManager';
import { i18nRouteGuard } from './I18n';

const router = createRouter({
  history: createWebHistory(services().routes.baseUrl),
  routes: [
    {
      path: `/:locale(${services().locales.supportedLocales.join('|')})?`,
      component: RouterView,
      children: [
        { path: '', component: HomePage },
        { path: 'about', component: AboutPage },
        { path: ':pathMatch(.*)*', name: 'NotFound', component: NotFoundPage },
      ],
    },
  ],
});

router.beforeEach(i18nRouteGuard);

export { router };
