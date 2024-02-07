import { LocaleMessages, createI18n } from 'vue-i18n';
import { NavigationGuard } from 'vue-router';
import { en as vuetifyEn } from 'vuetify/locale';
import { Locale } from '~/configs/i18n.config';
import { appInitConfig } from '~/configs/init.config';
import en from '~/locales/en.locale';
import { useAppStore } from '~/stores/app.store';
import { AppTranslations } from '~/types/app.types';
import { services } from './services.module';

// i18n is used for internationalization, please refer to the documentation at https://vue-i18n.intlify.dev/
const i18n = createI18n({
  // Vuetify does not support legacy mode of vue i18n
  legacy: false,
  locale: services().locales.resolveUserLocale(),
  fallbackLocale: appInitConfig.locale.default,
  globalInjection: true,
  messages: {
    [Locale.EN]: {
      ...en,
      $vuetify: vuetifyEn,
    },
  } as LocaleMessages<AppTranslations>,
});

const routeGuard: NavigationGuard = async (to, _from, next) => {
  const paramLocale = to.params.locale ? String(to.params.locale) : undefined;
  const app = useAppStore();
  if (!paramLocale) {
    return next({
      path: `/${app.locale}${to.path === '/' ? '' : to.path}`,
      query: to.query,
      hash: to.hash,
    });
  }

  if (services().locales.isSupportedLocale(paramLocale)) {
    await app.useLocale(paramLocale);
  }

  if (!services().locales.isSupportedLocale(paramLocale)) {
    return next({
      path: `/${app.locale}`,
      query: to.query,
      hash: to.hash,
    });
  }

  return next();
};

export { i18n, routeGuard as i18nRouteGuard };
