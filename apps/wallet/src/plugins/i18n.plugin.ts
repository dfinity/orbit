import { LocaleMessages, createI18n } from 'vue-i18n';
import { NavigationGuard } from 'vue-router';
import { en as vuetifyEn } from 'vuetify/locale';
import { Locale } from '~/configs/i18n.config';
import { appInitConfig } from '~/configs/init.config';
import en from '~/locales/en.locale';
import { useAppStore } from '~/stores/app.store';
import { AppTranslations } from '~/types/app.types';
import { services, type Services } from './services.plugin';

// i18n is used for internationalization, please refer to the documentation at https://vue-i18n.intlify.dev/
const i18n = createI18n({
  // Vuetify does not support legacy mode of vue i18n
  legacy: false,
  locale: services().locales.resolveUserLocale(),
  fallbackLocale: appInitConfig.locale.default || Locale.EN,
  globalInjection: true,
  messages: {
    [Locale.EN]: {
      ...en,
      $vuetify: vuetifyEn,
    },
  } as LocaleMessages<AppTranslations>,
});

const routeGuard =
  (services: Services, getAppStore: () => ReturnType<typeof useAppStore>): NavigationGuard =>
  async (to, _from, next) => {
    const app = getAppStore();
    const paramLocale = to.params.locale ? String(to.params.locale) : undefined;

    if (!paramLocale) {
      return next({
        path: `${app.baseUrl}${to.path}`,
        query: to.query,
        hash: to.hash,
      });
    }

    if (services.locales.isSupportedLocale(paramLocale)) {
      await app.useLocale(paramLocale);
    }

    if (!services.locales.isSupportedLocale(paramLocale)) {
      return next({
        path: app.baseUrl,
        query: to.query,
        hash: to.hash,
      });
    }

    return next();
  };

export { i18n, routeGuard as i18nRouteGuard };
