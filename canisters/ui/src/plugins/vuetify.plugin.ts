import { useI18n } from 'vue-i18n';
import { ThemeDefinition, createVuetify } from 'vuetify';
import { aliases, mdi } from 'vuetify/iconsets/mdi-svg';
import { createVueI18nAdapter } from 'vuetify/locale/adapters/vue-i18n';
import { Locale } from '~/configs/i18n.config';
import { SupportedTheme } from '~/types/app.types';
import { i18n } from './i18n.plugin';

// Styles for material design icons and vuetify
import '@mdi/font/css/materialdesignicons.css';

const light: ThemeDefinition = {
  dark: false,
  colors: {
    primary: '#00ffcc',
    secondary: '#030024',
    background: '#f0f0f0',
    surface: '#ffffff',
    neutral: '#b0b0b5',
    info: '#5da3cf',
    success: '#258f5c',
    warning: '#e39632',
    error: '#bd3e33',
    // custom styles
    sidebar: '#030024',
    landing: '#030024',
    'landing-surface': '#ffffff',
  },
};

const dark: ThemeDefinition = {
  dark: true,
  colors: {
    primary: '#00ffcc',
    secondary: '#0c6a6b',
    background: '#010011',
    surface: '#191822',
    neutral: '#b0b0b5',
    info: '#5da3cf',
    success: '#258f5c',
    warning: '#e39632',
    error: '#bd3e33',
    // custom styles
    sidebar: '#191822',
    landing: '#030024',
    'landing-surface': '#ffffff',
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
