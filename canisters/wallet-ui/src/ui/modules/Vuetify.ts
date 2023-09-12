import { useI18n } from 'vue-i18n';
import { createVuetify } from 'vuetify';
import { aliases, mdi } from 'vuetify/iconsets/mdi';
import { createVueI18nAdapter } from 'vuetify/locale/adapters/vue-i18n';
import 'vuetify/styles';
import { Locale } from '~/configs';
import { i18n } from './I18n';

// Vuetify is the default UI framework used within this application,
// please refer to the documentation for more information at https://vuetifyjs.com/
const vuetify = createVuetify({
  ssr: false,
  icons: {
    defaultSet: 'mdi',
    aliases,
    sets: {
      mdi,
    },
  },
  locale: {
    adapter: createVueI18nAdapter({ i18n, useI18n }),
  },
});

export const fetchDesignSystemLocale = async (locale: Locale): Promise<unknown> => {
  const vuetifyLocale = await import(`../../../node_modules/vuetify/lib/locale/${locale}.mjs`);

  return vuetifyLocale.default;
};

export { vuetify };
