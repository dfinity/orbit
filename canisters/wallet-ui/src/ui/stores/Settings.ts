import { defineStore } from 'pinia';
import { Locale } from '~/configs/I18n';
import { fetchDesignSystemLocale, i18n, services } from '~/ui/modules';
import { en as designSystemFallbackMessages } from 'vuetify/locale';

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
      const isLoadedLocale = i18n.global.availableLocales.includes(locale);
      if (isLoadedLocale && i18n.global.locale.value === locale) {
        // do nothing if the locale is the same
        return;
      }

      if (!isLoadedLocale) {
        const messages = await services().locales.fetchLocaleMessages(locale);
        const designSystemMessages = await fetchDesignSystemLocale(locale).catch(e => {
          console.error(`Failed to load design system locale for ${locale}`, e);

          return designSystemFallbackMessages;
        });
        i18n.global.setLocaleMessage(locale, {
          ...messages,
          $vuetify: designSystemMessages,
        });
      }

      i18n.global.locale.value = locale;
      services().locales.updatePageLocale(locale);
    },
  },
});
