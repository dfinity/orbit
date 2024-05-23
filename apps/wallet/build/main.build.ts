import vue from '@vitejs/plugin-vue';
import { basename, dirname, resolve } from 'path';
import { defineConfig } from 'vite';
import { nodePolyfills } from 'vite-plugin-node-polyfills';
import vuetify from 'vite-plugin-vuetify';
import { resolveCanisterIds } from './core/canisters.core';
import {
  ENV,
  MODE,
  OPTIMIZED_BUILD,
  PRODUCTION,
  STATION_API_VERSION,
  SUPPORTED_LOCALTES,
} from './core/configs.core';
import { generateICAssetsJson } from './core/ic-assets.core';
import { apiCompatibilityFile } from './plugins/api-compatibility-file.plugin';
import { withVersionedEntrypoint } from './plugins/with-versioned-entrypoint.plugin';
import { getCommitHash } from './utils/git.utils';

// https://vitejs.dev/config/
export default defineConfig(_ => {
  const canisters = resolveCanisterIds();
  const commitHash = getCommitHash();

  // Generate the IC assets JSON file for the project.
  generateICAssetsJson();

  // Defaults configuration for the build.
  const mode = MODE;
  const optimized = OPTIMIZED_BUILD;
  const outDir = resolve(__dirname, '../dist');

  return {
    mode,
    base: '/',
    root: '.',
    publicDir: './public',
    appType: 'spa',
    server: {
      open: false,
    },
    preview: {
      open: true,
    },
    // Vite automatically loads .env files from the root of the project if they are prefixed with the envPrefix.
    envPrefix: 'APP_',
    plugins: [
      nodePolyfills(),
      vue(),
      vuetify({ autoImport: true }),
      apiCompatibilityFile(),
      withVersionedEntrypoint(),
    ],
    build: {
      target: 'es2020',
      sourcemap: !optimized,
      minify: optimized,
      chunkSizeWarningLimit: 500,
      outDir,
      emptyOutDir: true,
      rollupOptions: {
        input: {
          latest: resolve(__dirname, '../index.html'),
        },
        output: [
          {
            name: 'latest',
            manualChunks: id => {
              const folder = dirname(id);
              const isNodeModule = folder.includes('node_modules');

              if (
                folder.includes('/src/locales') &&
                SUPPORTED_LOCALTES.some(locale => resolve(folder, `${locale}.locale.ts`) === id)
              ) {
                const [localeName] = basename(id).split('.');
                return `locale-${localeName}`;
              }

              if (isNodeModule && folder.includes('/@dfinity')) {
                return 'ic-libs';
              }

              if (
                isNodeModule &&
                ['/vue-i18n/', '/@intlify/'].some(vendor => folder.includes(vendor))
              ) {
                return 'localization';
              }

              if (
                isNodeModule &&
                ['/vue-router/', '/pinia/', '/vuetify/', '/vue-i18n/', '/@intlify/'].some(vendor =>
                  folder.includes(vendor),
                )
              ) {
                return 'vendor';
              }
            },
          },
        ],
      },
    },
    optimizeDeps: {
      esbuildOptions: {
        define: {
          global: 'globalThis',
        },
      },
    },
    worker: {
      format: 'es',
    },
    css: {
      devSourcemap: !optimized,
    },
    test: {
      globals: true,
      environment: 'jsdom',
      setupFiles: [resolve(__dirname, './polyfills/test.polyfill.ts')],
      server: {
        deps: {
          inline: ['vuetify'],
        },
      },
    },
    define: {
      // Vite env variable replacements for the runtime.
      //
      // Make sure to use import.meta.env as the prefix since
      // vite uses that during runtime to access the variables.
      // https://vitejs.dev/guide/env-and-mode.html#env-variables
      'import.meta.env.PROD': PRODUCTION,
      'import.meta.env.DEV': !PRODUCTION,
      'import.meta.env.APP_STATION_API_VERSION': JSON.stringify(STATION_API_VERSION),
      'import.meta.env.APP_MODE': JSON.stringify(ENV.APP_MODE),
      'import.meta.env.APP_URL': JSON.stringify(ENV.APP_URL),
      'import.meta.env.APP_BUILD_MODE': JSON.stringify(mode),
      'import.meta.env.APP_BUILD_VERSION': JSON.stringify(process.env.npm_package_version),
      'import.meta.env.APP_BUILD_HASH': JSON.stringify(commitHash),
      'import.meta.env.APP_BUILD_DATE': JSON.stringify(new Date().toISOString()),
      'import.meta.env.APP_CANISTER_ID_APP_WALLET': JSON.stringify(canisters.app_wallet),
      'import.meta.env.APP_CANISTER_ID_CONTROL_PANEL': JSON.stringify(canisters.control_panel),
      'import.meta.env.APP_CANISTER_ID_INTERNET_IDENTITY': JSON.stringify(
        canisters.internet_identity,
      ),
      'import.meta.env.APP_PROVIDER_URL_INTERNET_IDENTITY': PRODUCTION
        ? JSON.stringify(ENV.APP_PROVIDER_URL_INTERNET_IDENTITY)
        : JSON.stringify(`http://${canisters.internet_identity}.localhost:4943`),
      'process.env.CANISTER_ID_CONTROL_PANEL': JSON.stringify(canisters.control_panel),
      'process.env.CANISTER_ID_ICP_INDEX': JSON.stringify(canisters.icp_index),
    },
    resolve: {
      alias: {
        '~': resolve('src'),
        '~assets': resolve('assets'),
      },
    },
  };
});
