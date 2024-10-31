import vue from '@vitejs/plugin-vue';
import { basename, dirname, resolve } from 'path';
import { defineConfig } from 'vite';
import vuetify from 'vite-plugin-vuetify';
import wasm from 'vite-plugin-wasm';
import {
  ENV,
  MODE,
  OPTIMIZED_BUILD,
  PRODUCTION,
  STATION_API_VERSION,
  SUPPORTED_LOCALES,
} from './core/configs.core';
import { withCanisterIds } from './plugins/with-canister-ids';
import { withApiCompatibilityFile } from './plugins/with-compatibility-file.plugin';
import { withIcAssetsFile } from './plugins/with-ic-assets.plugin';
import { withVersionedEntrypoint } from './plugins/with-versioned-entrypoint.plugin';
import { getCommitHash } from './utils/git.utils';

// https://vitejs.dev/config/
export default defineConfig(_ => {
  const commitHash = getCommitHash();

  // Defaults configuration for the build.
  const mode = MODE;
  const optimized = OPTIMIZED_BUILD;
  const outDir = resolve(__dirname, '../dist');
  const isProduction = PRODUCTION;

  return {
    mode,
    base: '/',
    root: '.',
    publicDir: './public',
    appType: 'spa',
    server: {
      open: false,
      fs: {
        // required to be able to test packages locally with pnpm link
        strict: false,
      },
    },
    preview: {
      open: true,
    },
    // Vite automatically loads .env files from the root of the project if they are prefixed with the envPrefix.
    envPrefix: 'APP_',
    plugins: [
      wasm(),
      vue(),
      vuetify({ autoImport: true }),
      withCanisterIds({ isProduction }),
      withApiCompatibilityFile(),
      withVersionedEntrypoint(),
      withIcAssetsFile(isProduction && MODE !== 'localhost'),
    ],
    build: {
      target: 'es2022',
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

              if (isNodeModule && folder.includes('/@dfinity/didc')) {
                // This ensures that the didc library is not included in the main ic libs chunk,
                // this is because the didc library includes a wasm file that is loaded at runtime and
                // makes the first load of the application slower.
                return `ic-didc`;
              }

              if (
                folder.includes('/src/locales') &&
                SUPPORTED_LOCALES.some(locale => resolve(folder, `${locale}.locale.ts`) === id)
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
        plugins: [],
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
      setupFiles: [resolve(__dirname, './polyfills/test.polyfills.ts')],
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
      'import.meta.env.PROD': isProduction,
      'import.meta.env.DEV': !isProduction,
      'import.meta.env.APP_STATION_API_VERSION': JSON.stringify(STATION_API_VERSION),
      'import.meta.env.APP_MODE': JSON.stringify(ENV.APP_MODE),
      'import.meta.env.APP_URL': JSON.stringify(ENV.APP_URL),
      'import.meta.env.APP_BUILD_MODE': JSON.stringify(mode),
      'import.meta.env.APP_BUILD_VERSION': JSON.stringify(process.env.npm_package_version),
      'import.meta.env.APP_BUILD_HASH': JSON.stringify(commitHash),
      // This enables the JIT compilation for the vue-i18n plugin as specified in their documentation
      // https://vue-i18n.intlify.dev/guide/advanced/optimization#jit-compilation, it is required to skip
      // the compilation of the locales at runtime which needs unsafe-eval to be enabled in the CSP.
      //
      // Version 10 which is still in beta has this feature enabled by default, once it is stable we can bump and
      // remove this define.
      __INTLIFY_JIT_COMPILATION__: true,
    },
    resolve: {
      alias: {
        '~': resolve('src'),
        '~assets': resolve('assets'),
        '~build': resolve('build'),
      },
    },
  };
});
