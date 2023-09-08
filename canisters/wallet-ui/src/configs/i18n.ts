export enum Locale {
  EN = 'en',
  PT = 'pt',
}

const allLocales = Object.values(Locale);
const envEnabledLocales = (import.meta.env.APP_SUPPORTED_LOCALES || Locale.EN.toString()).split(
  ',',
);
const supportedLocales: Locale[] = allLocales.filter(locale => envEnabledLocales.includes(locale));

let defaultLocale = Locale.EN;
if (supportedLocales.includes(import.meta.env.APP_DEFAULT_LOCALE as Locale)) {
  defaultLocale = import.meta.env.APP_DEFAULT_LOCALE as Locale;
}

export { defaultLocale, supportedLocales };
