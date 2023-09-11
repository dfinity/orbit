import { LocaleMessages, createI18n } from 'vue-i18n';
import { NavigationGuard } from 'vue-router';
import { Locale, appInitConfig } from '~/configs';
import en from '~/locales/en.json';
import { useSettingsStore } from '~/ui/stores';
import { services } from './ServiceManager';
import { AppTranslations } from '~/types';

// i18n is used for internationalization, please refer to the documentation at https://vue-i18n.intlify.dev/
const i18n = createI18n({
  // Vuetify does not support legacy mode of vue i18n
  legacy: false,
  locale: services().locales.resolveUserLocale(),
  fallbackLocale: appInitConfig.locale.default,
  globalInjection: true,
  messages: {
    [Locale.EN]: en,
  } as LocaleMessages<AppTranslations>,
});

const routeGuard: NavigationGuard = async (to, _from, next) => {
  const paramLocale = to.params.locale ? String(to.params.locale) : undefined;
  const settings = useSettingsStore();
  if (!paramLocale) {
    return next(`/${settings.locale}${to.path === '/' ? '' : to.path}`);
  }

  await settings.useLocale(paramLocale as Locale);
  
  if (!services().locales.isSupportedLocale(paramLocale)) {
    return next(`/${settings.locale}`);
  }

  return next();
};

export { i18n, routeGuard as i18nRouteGuard };
