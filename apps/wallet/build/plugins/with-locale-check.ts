import { readdirSync } from 'fs';
import { resolve } from 'path';
import { Plugin } from 'vite';
import enLocale from '../../src/locales/en.locale';
import frLocale from '../../src/locales/fr.locale';
import ptLocale from '../../src/locales/pt.locale';

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
      if (locale[key] === undefined) {
        console.log(`Key ${wholeKey} is missing in ${localeName}`);
        result = false;
        continue;
      }

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
  let isServe = false;

  return {
    name: 'with-locale-check',
    configResolved(config) {
      // config.command is "serve" in dev, "build" in production
      isServe = config.command === 'serve';
    },

    async buildStart() {
      const errorOut = (msg: string) => {
        if (isServe) {
          // In dev, just warn (doesn’t stop the server)
          this.warn(msg);
        } else {
          // In production build, fail
          this.error(msg);
        }
      };

      const locales = [
        { name: 'pt', locale: ptLocale },
        { name: 'fr', locale: frLocale },
      ];

      // check if there are only these locale files in the src/locales folder
      const localeFiles = readdirSync(resolve(__dirname, '../../src/locales'));
      if (localeFiles.length !== locales.length + 1) {
        errorOut(
          `There are ${localeFiles.length} locale files in the src/locales folder, expected ${locales.length + 1}.`,
        );
      }

      const badLocales: string[] = [];
      for (const locale of locales) {
        if (!compareLocales(enLocale, locale.locale, '', locale.name)) {
          badLocales.push(locale.name);
        }
      }

      if (badLocales.length > 0) {
        errorOut(`Locale keys are not equal to en.locale.ts: ${badLocales.join(', ')}`);
      }
    },
  };
};
