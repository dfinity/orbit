export enum Locale {
  EN = 'en',
  PT = 'pt',
}

const allLocales = Object.values(Locale);
const envEnabledLocales = (import.meta.env.APP_SUPPORTED_LOCALES || Locale.EN.toString()).split(
  ',',
);
const supportedLocales: Locale[] = allLocales.filter(locale => envEnabledLocales.includes(locale));
if (!supportedLocales.includes(Locale.EN)) {
  supportedLocales.push(Locale.EN);
}

const defaultLocale = Locale.EN;

export { defaultLocale, supportedLocales };
