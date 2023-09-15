import { defineStore } from 'pinia';
import { useDisplay } from 'vuetify';
import { en as designSystemFallbackMessages } from 'vuetify/locale';
import { appInitConfig } from '~/configs';
import { Locale } from '~/configs/I18n';
import { logger } from '~/core';
import { SupportedTheme } from '~/types';
import { fetchDesignSystemLocale, i18n, services } from '~/ui/modules';

export interface SettingsStoreState {
  appName: string;
  theme: SupportedTheme;
  showSidebar: boolean;
}

export const useSettingsStore = defineStore('settings', {
  state: (): SettingsStoreState => {
    return {
      appName: appInitConfig.name,
      theme: services().theme.resolveTheme(),
      showSidebar: true,
    };
  },
  getters: {
    isMobile(): boolean {
      const { mobile } = useDisplay();

      return mobile.value;
    },
    isDarkTheme(): boolean {
      return this.theme === SupportedTheme.Dark;
    },
    supportedLocales(): Locale[] {
      return services().locales.supportedLocales.sort();
    },
    locale(): Locale {
      return i18n.global.locale.value as Locale;
    },
    baseUrl(): string {
      return services().routes.baseUrl + this.locale;
    },
  },
  actions: {
    async useLocale(locale: Locale, persist = false): Promise<void> {
      const isLoadedLocale = i18n.global.availableLocales.includes(locale);
      if (isLoadedLocale && i18n.global.locale.value === locale) {
        // do nothing if the locale is the same
        return;
      }

      if (!isLoadedLocale) {
        const messages = await services().locales.fetchLocaleMessages(locale);
        const designSystemMessages = await fetchDesignSystemLocale(locale).catch(e => {
          logger.error(`Failed to load design system locale for ${locale}`, e);

          return designSystemFallbackMessages;
        });
        i18n.global.setLocaleMessage(locale, {
          ...messages,
          $vuetify: designSystemMessages,
        });
      }

      i18n.global.locale.value = locale;
      services().locales.updatePageLocale(locale);
      if (persist) {
        await services().locales.saveLocale(locale);
      }
    },
    async toogleTheme(): Promise<void> {
      const theme = this.isDarkTheme ? SupportedTheme.Light : SupportedTheme.Dark;
      this.theme = theme;

      services().theme.updateUserTheme(theme);
    },
    toogleSidebar(): void {
      this.showSidebar = !this.showSidebar;
    },
  },
});
