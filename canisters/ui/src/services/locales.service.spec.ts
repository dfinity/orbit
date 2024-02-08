import { describe, expect, it } from 'vitest';
import { Locale } from '~/configs/i18n.config';
import { LocalesService } from './locales.service';

describe('LocalesService', async () => {
  it.each([
    { location: new URL('https://example.com/en'), locale: 'en' },
    { location: new URL('https://example.com/en/'), locale: 'en' },
    { location: new URL('https://example.com/pt/en/inner-path'), locale: 'pt' },
    { location: new URL('https://example.com/my-path/'), locale: undefined },
    { location: new URL('https://example.com/en/12/path/test/pt/path'), locale: 'en' },
  ])(
    `should resolve to $locale for user location($location.href)`,
    async ({ location, locale }) => {
      const localesService = new LocalesService();
      window.location = location as unknown as Location;
      const resolvedLocationLocale = localesService.maybeResolveLocationLocale();

      expect(resolvedLocationLocale).toBe(locale);
    },
  );

  it(`should be supported locale`, async () => {
    const localesService = new LocalesService();

    expect(localesService.isSupportedLocale('en')).toBeTruthy();
    expect(localesService.isSupportedLocale('pt')).toBeTruthy();
  });

  it(`should not be supported locale`, async () => {
    const localesService = new LocalesService();

    expect(localesService.isSupportedLocale('es')).toBeFalsy();
  });

  it(`should not be supported locale`, async () => {
    const localesService = new LocalesService();

    await localesService.updatePageLocale(Locale.PT);
    expect(document.querySelector('html')?.getAttribute('lang')).toBe(Locale.PT);

    await localesService.updatePageLocale(Locale.EN);
    expect(document.querySelector('html')?.getAttribute('lang')).toBe(Locale.EN);
  });
});
