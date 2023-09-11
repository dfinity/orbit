import { defineStore } from 'pinia';
import { Locale } from '~/configs/I18n';
import { i18n, services } from '~/ui/modules';

export const useSettingsStore = defineStore('settings', {
  state: () => ({}),
  getters: {
    locale(): Locale {
      return i18n.global.locale.value as Locale;
    },
    baseUrl(): string {
      return services().routes.baseUrl + this.locale;
    },
  },
  actions: {
    async useLocale(locale: Locale, _persist = false): Promise<void> {
      if (i18n.global.locale.value === locale) {
        // do nothing if the locale is the same
        return;
      }

      if (!i18n.global.availableLocales.includes(locale)) {
        const messages = await services().locales.fetchLocaleMessages(locale);
        i18n.global.setLocaleMessage(locale, messages);
      }

      i18n.global.locale.value = locale;
      services().locales.updatePageLocale(locale);
    },
  },
});
