import { defineStore } from 'pinia';
import { Locale } from '~/configs/I18n';
import { i18n, services } from '~/ui/modules';

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    locale: services().locales.resolveUserLocale(),
  }),
  getters: {
    baseUrl: (state): string => {
      return services().routes.baseUrl + state.locale;
    },
  },
  actions: {
    setLocale(locale: Locale) {
      this.locale = locale;

      i18n.global.locale.value = locale;
      services().locales.updatePageLocale(locale);
    },
  },
});
