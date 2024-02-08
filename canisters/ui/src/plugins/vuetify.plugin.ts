import { useI18n } from 'vue-i18n';
import { ThemeDefinition, createVuetify } from 'vuetify';
import { aliases, mdi } from 'vuetify/iconsets/mdi-svg';
import { createVueI18nAdapter } from 'vuetify/locale/adapters/vue-i18n';
import { Locale } from '~/configs/i18n.config';
import { SupportedTheme } from '~/types/app.types';
import { i18n } from './i18n.plugin';

const light: ThemeDefinition = {
  dark: false,
  colors: {
    background: '#f6f6f6',
    'background-border': '#9f9f9f',
    surface: '#fefefe',
    primary: '#00183b',
    'primary-variant': '#264674',
    secondary: '#09f381',
    'secondary-variant': '#053f36',
    error: '#B00020',
    info: '#2196F3',
    success: '#4CAF50',
    warning: '#FB8C00',
  },
};

const dark: ThemeDefinition = {
  dark: true,
  colors: {
    background: '#2B2D42',
    'background-border': '#424761',
    surface: '#313449',
    primary: '#325a9b',
    'primary-variant': '#3071c8',
    secondary: '#36d399',
    'secondary-variant': '#2E665d',
    error: '#cf6679',
    info: '#64B5f6',
    success: '#81c784',
    warning: '#ffA726',
  },
};

// Vuetify is the default UI framework used within this application,
// please refer to the documentation for more information at https://vuetifyjs.com/
const vuetify = (theme: SupportedTheme = SupportedTheme.Light) =>
  createVuetify({
    ssr: false,
    theme: {
      defaultTheme: theme,
      variations: {
        colors: ['primary', 'primary-variant', 'secondary', 'secondary-variant'],
        darken: 2,
        lighten: 2,
      },
      themes: {
        light,
        dark,
      },
    },
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
    display: {
      mobileBreakpoint: 'md',
      thresholds: {
        xs: 0,
        sm: 600,
        md: 960,
        lg: 1280,
        xl: 1920,
        xxl: 2560,
      },
    },
  });

export const fetchDesignSystemLocale = async (locale: Locale): Promise<unknown> => {
  const vuetifyLocale = await import(`../../node_modules/vuetify/lib/locale/${locale}.mjs`);

  return vuetifyLocale.default;
};

export { vuetify };
