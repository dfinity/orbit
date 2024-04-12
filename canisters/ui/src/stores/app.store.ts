import { defineStore } from 'pinia';
import { en as designSystemFallbackMessages } from 'vuetify/locale';
import { Locale } from '~/configs/i18n.config';
import { appInitConfig } from '~/configs/init.config';
import { RouteStatusCode } from '~/configs/routes.config';
import { logger } from '~/core/logger.core';
import { i18n } from '~/plugins/i18n.plugin';
import { services } from '~/plugins/services.plugin';
import { fetchDesignSystemLocale } from '~/plugins/vuetify.plugin';
import { useSessionStore } from '~/stores/session.store';
import { GlobalNotification, SupportedTheme } from '~/types/app.types';
import { isApiError } from '~/utils/app.utils';

export interface AppStoreState {
  initialized: boolean;
  loading: boolean;
  appName: string;
  theme: SupportedTheme;
  showSidebar: boolean;
  notification: GlobalNotification;
  isMobile: boolean;
  routeStatusCode: RouteStatusCode;
  disableBackgroundPolling: boolean;
}

export const useAppStore = defineStore('app', {
  state: (): AppStoreState => {
    return {
      initialized: false,
      disableBackgroundPolling: false,
      loading: false,
      appName: appInitConfig.name,
      theme: services().theme.resolveTheme(),
      showSidebar: true,
      isMobile: false,
      notification: {
        show: false,
        message: '',
        type: 'info',
      },
      routeStatusCode: RouteStatusCode.Success,
    };
  },
  getters: {
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
      return appInitConfig.baseUrl + this.locale;
    },
  },
  actions: {
    async initialize(): Promise<void> {
      if (this.initialized) {
        return;
      }

      const session = useSessionStore();
      await session.initialize();

      this.initialized = true;
    },
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
    setIsMobile(isMobile: boolean): void {
      this.isMobile = isMobile;
      this.showSidebar = !isMobile;
    },
    toogleSidebar(): void {
      this.showSidebar = !this.showSidebar;
    },
    sendNotification({
      message,
      type,
    }: {
      message: GlobalNotification['message'];
      type: GlobalNotification['type'];
    }): void {
      this.notification = {
        show: true,
        message,
        type,
      };
    },

    sendErrorNotification(error: unknown): void {
      let message = i18n.global.t('app.request_failed_message');

      if (isApiError(error) && error.message.length > 0) {
        message = `${message}: ${error.message[0]}`;
      } else if (error instanceof Error) {
        message = `${message}: ${error.message}`;
      }
      this.sendNotification({
        type: 'error',
        message,
      });
    },
  },
});
