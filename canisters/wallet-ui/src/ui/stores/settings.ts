import { defineStore } from 'pinia';
import { services } from '~/ui/services';
import { Locale } from '~/configs/i18n';
import { i18n } from '~/ui/i18n';

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    locale: services.locales.resolveUserLocale(),
  }),
  getters: {
    baseUrl: (state): string => {
      return services.routes.baseUrl + state.locale;
    },
  },
  actions: {
    setLocale(locale: Locale) {
      this.locale = locale;

      i18n.global.locale.value = locale;
      services.locales.updatePageLocale(locale);
    },
  },
});
