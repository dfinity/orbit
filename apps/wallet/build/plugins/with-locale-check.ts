import { Plugin } from 'vite';
import { readdirSync, readFileSync } from 'fs';
import { resolve } from 'path';
import enLocale from '../../src/locales/en.locale';
import ptLocale from '../../src/locales/pt.locale';
import frLocale from '../../src/locales/fr.locale';

type LocaleKey = {
  [key: string]: LocaleKey | string;
};

function compareLocales(
  enLocale: LocaleKey,
  locale: LocaleKey,
  root: string,
  localeName: string,
): boolean {
  const enKeys = Object.keys(enLocale);
  const localeKeys = Object.keys(locale);

  let result = true;

  for (const key of enKeys) {
    const wholeKey = root ? root + '.' + key : key;

    if (typeof enLocale[key] === 'object') {
      if (!compareLocales(enLocale[key], locale[key] as LocaleKey, wholeKey, localeName)) {
        result = false;
      }
    }

    if (!localeKeys.includes(key)) {
      console.error(`Key ${wholeKey} is missing in ${localeName}`);
      result = false;
    }
  }

  return result;
}

export const withLocaleCheck = (): Plugin => {
  return {
    name: 'with-locale-check',
    async buildStart() {
      const locales = [
        { name: 'pt', locale: ptLocale },
        { name: 'fr', locale: frLocale },
      ];

      // check if there are only these locale files in the src/locales folder
      const localeFiles = readdirSync(resolve(__dirname, '../../src/locales'));
      if (localeFiles.length !== locales.length + 1) {
        console.error(
          `ERROR: There are ${localeFiles.length} locale files in the src/locales folder, expected ${locales.length + 1}.`,
        );
        process.exit(1);
      }

      let result = true;
      for (const locale of locales) {
        if (!compareLocales(enLocale, locale.locale, '', locale.name)) {
          console.error(`ERROR: Locale keys of ${locale.name} are not equal to en.locale.ts`);
          result = false;
        }
      }

      if (!result) {
        process.exit(1);
      }
    },
  };
};
