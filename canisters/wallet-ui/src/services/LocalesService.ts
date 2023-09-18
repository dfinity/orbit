import { Locale, appInitConfig, supportedLocales } from '~/configs';
import { AppTranslations } from '~/types';

export class LocalesService {
  static readonly localeStorageKey = 'locale';

  get supportedLocales(): Locale[] {
    return supportedLocales;
  }

  get defaultLocale(): Locale {
    return appInitConfig.locale.default as Locale;
  }

  async updatePageLocale(locale: Locale): Promise<void> {
    document.querySelector('html')?.setAttribute('lang', locale);
  }

  async saveLocale(locale: Locale): Promise<void> {
    localStorage.setItem(LocalesService.localeStorageKey, locale);
  }

  async fetchLocaleMessages(locale: Locale): Promise<AppTranslations> {
    const messages = await import(`~/locales/${locale}.json`);
    return messages.default;
  }

  maybeResolveLocationLocale(): Locale | undefined {
    const locale = window.location.pathname.replace(/^\/([^/]+).*/i, '$1');

    if (!this.isSupportedLocale(locale)) {
      return;
    }

    return locale as Locale;
  }

  isSupportedLocale(locale: string): locale is Locale {
    return this.supportedLocales.includes(locale as Locale);
  }

  resolveUserLocale(): Locale {
    const locale =
      localStorage.getItem(LocalesService.localeStorageKey) ||
      this.maybeResolveLocationLocale() ||
      window.navigator.language ||
      this.defaultLocale;
    const localeNoRegion = locale.split('-')?.[0] ?? this.defaultLocale;

    if (!this.supportedLocales.includes(localeNoRegion as Locale)) {
      return this.defaultLocale;
    }

    return localeNoRegion as Locale;
  }
}
