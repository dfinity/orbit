import inject from '@rollup/plugin-inject';
import vue from '@vitejs/plugin-vue';
import { existsSync, readFileSync, readdirSync } from 'fs';
import { basename, dirname, resolve } from 'path';
import vuetify from 'vite-plugin-vuetify';
import { defineConfig } from 'vitest/config';
import dfxConfig from './dfx.json';

const network = process.env.DFX_NETWORK ?? 'local';

const resolveCanisterIds = (): Map<string, string> => {
  const availableCanisters = new Map<string, string>();
  const canisters = Object.entries(dfxConfig.canisters);
  const canisterIdsFilePath =
    network === 'local'
      ? resolve(__dirname, '.dfx/local/canister_ids.json')
      : resolve(__dirname, 'canister_ids.json');

  if (!existsSync(canisterIdsFilePath)) {
    console.warn(`Canister ids file not found at ${canisterIdsFilePath}`);
    return availableCanisters;
  }

  const config: Record<string, Record<string, string>> = JSON.parse(
    readFileSync(canisterIdsFilePath, 'utf-8'),
  );
  for (const [canisterName] of canisters) {
    const details = config[canisterName];
    if (!details[network]) {
      throw new Error(
        `Canister ${canisterName} does not have a defined canister id for ${network}`,
      );
    }

    availableCanisters.set(canisterName, details[network]);
  }

  return availableCanisters;
};

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => {
  const isProduction = mode === 'production';
  const localesPath = resolve(__dirname, 'src/locales');
  const supportedLocales = readdirSync(localesPath).map(file => basename(file, '.json'));
  const canisters = resolveCanisterIds();

  return {
    base: '/',
    root: '.',
    publicDir: './public',
    appType: 'spa',
    // Vite automatically loads .env files from the root of the project
    // if they are prefixed with the envPrefix.
    envPrefix: 'APP_',
    plugins: [vue(), vuetify({ autoImport: true })],
    build: {
      target: 'es2020',
      sourcemap: !isProduction,
      minify: isProduction,
      chunkSizeWarningLimit: 500,
      outDir: './dist',
      emptyOutDir: true,
      rollupOptions: {
        input: {
          main: './index.html',
        },
        output: {
          manualChunks: id => {
            const folder = dirname(id);
            const isNodeModule = folder.includes('node_modules');

            if (
              folder.includes('/src/locales') &&
              supportedLocales.some(locale => resolve(folder, `${locale}.json`) === id)
            ) {
              const [localeName] = basename(id).split('.');
              return `locale-${localeName}`;
            }

            if (
              isNodeModule &&
              [
                '/@vue/',
                '/vue/',
                '/vue-router/',
                '/vue-demi/',
                '/pinia/',
                '/vuetify/',
                '/vue-i18n/',
                '/@intlify/',
              ].some(vendor => folder.includes(vendor))
            ) {
              return 'vendor';
            }
          },
        },
        plugins: [
          inject({
            modules: {
              // Polyfill Buffer for production build
              Buffer: ['buffer', 'Buffer'],
            },
          }),
        ],
      },
    },
    css: {
      devSourcemap: !isProduction,
    },
    test: {
      globals: true,
      environment: 'happy-dom',
      setupFiles: ['./setupFiles/GlobalsConfiguration.ts'],
    },
    define: {
      // Vite env variable replacements for the runtime.
      //
      // Make sure to use import.meta.env as the prefix since
      // vite uses that during runtime to access the variables.
      // https://vitejs.dev/guide/env-and-mode.html#env-variables
      'import.meta.env.APP_VERSION': JSON.stringify(`v${process.env.npm_package_version}`),
      'import.meta.env.APP_WALLET_UI_CANISTER_ID': JSON.stringify(canisters.get('wallet_ui')),
      'import.meta.env.APP_INTERNET_IDENTITY_CANISTER_ID': JSON.stringify(
        canisters.get('internet_identity'),
      ),
      'import.meta.env.APP_INTERNET_IDENTITY_PROVIDER_URL': JSON.stringify(
        `http://${canisters.get('internet_identity')}.localhost:4943`,
      ),
    },
    resolve: {
      alias: {
        '~': resolve('src'),
      },
    },
  };
});
