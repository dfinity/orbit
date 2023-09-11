import { Locale, supportedLocales, appInitConfig } from '~/configs';

export class LocalesService {
  get supportedLocales(): Locale[] {
    return supportedLocales;
  }

  get defaultLocale(): Locale {
    return appInitConfig.locale.default as Locale;
  }

  async updatePageLocale(locale: Locale): Promise<void> {
    document.querySelector('html')?.setAttribute('lang', locale);
  }

  async fetchLocaleMessages(locale: string): Promise<unknown> {
    const messages = await import(`~/locales/${locale}.json`);

    return messages.default;
  }

  maybeResolveLocationLocale(): Locale | undefined {
    const locale = window.location.pathname.replace(/^\/([^/]+).*/i, '$1');

    if (this.isSupportedLocale(locale)) {
      return;
    }

    return locale as Locale;
  }

  isSupportedLocale(locale: string): boolean {
    return this.supportedLocales.includes(locale as Locale);
  }

  resolveUserLocale(): Locale {
    const locale =
      this.maybeResolveLocationLocale() || window.navigator.language || this.defaultLocale;
    const localeNoRegion = locale.split('-')?.[0] ?? this.defaultLocale;

    if (!this.supportedLocales.includes(localeNoRegion as Locale)) {
      return this.defaultLocale;
    }

    return localeNoRegion as Locale;
  }
}
