import vue from '@vitejs/plugin-vue';
import { nodePolyfills } from 'vite-plugin-node-polyfills';
import { existsSync, readFileSync, readdirSync } from 'fs';
import { basename, dirname, resolve } from 'path';
import { defineConfig } from 'vite';
import vuetify from 'vite-plugin-vuetify';

const network = process.env.DFX_NETWORK ?? 'local';
const defaultCanisterIds = {
  internet_identity: 'rdmx6-jaaaa-aaaaa-aaadq-cai',
  icp_index: 'qhbym-qaaaa-aaaaa-aaafq-cai',
  control_panel: '65evf-oqaaa-aaaal-add6q-cai',
  ui: '6uh6z-yyaaa-aaaal-add7a-cai',
};

const resolveCanisterIds = (
  fallbackCanisterIds: typeof defaultCanisterIds = defaultCanisterIds,
): typeof defaultCanisterIds => {
  const dfxConfig: {
    canisters: Record<string, unknown>;
  } = JSON.parse(readFileSync(resolve(__dirname, '../..', 'dfx.json'), 'utf-8'));
  const availableCanisters = Object.assign({}, fallbackCanisterIds);
  const canisters = Object.entries(dfxConfig.canisters);
  const canisterIdsFilePath =
    network === 'local'
      ? resolve(__dirname, '../..', '.dfx/local/canister_ids.json')
      : resolve(__dirname, '../..', 'canister_ids.json');

  if (!existsSync(canisterIdsFilePath)) {
    console.warn(`Canister ids file not found at ${canisterIdsFilePath}`);
    return availableCanisters;
  }

  const config: Record<string, Record<string, string>> = JSON.parse(
    readFileSync(canisterIdsFilePath, 'utf-8'),
  );
  for (const [canisterName] of canisters) {
    const details = config[canisterName];
    if (!availableCanisters[canisterName]) {
      // only use the canister if explicitly defined
      continue;
    }

    if (!details?.[network]) {
      console.warn(`Canister ${canisterName} does not have a defined canister id for ${network}`);
      continue;
    }

    availableCanisters[canisterName] = details[network];
  }

  return availableCanisters;
};

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => {
  const isDevelopment =
    process.env.NODE_ENV && process.env.NODE_ENV.length
      ? process.env.NODE_ENV === 'development'
      : mode === 'development';
  const isProduction = !isDevelopment;
  mode = isProduction ? 'production' : 'development';
  const localesPath = resolve(__dirname, 'src/locales');
  const supportedLocales = readdirSync(localesPath).map(file => basename(file, '.ts'));
  const canisters = resolveCanisterIds();

  return {
    mode,
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
              supportedLocales.some(locale => resolve(folder, `${locale}.ts`) === id)
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
        plugins: [nodePolyfills()],
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
      devSourcemap: !isProduction,
    },
    test: {
      globals: true,
      environment: 'jsdom',
      setupFiles: ['./setup/globals.config.ts'],
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
      'import.meta.env.APP_VERSION': JSON.stringify(`v${process.env.npm_package_version}`),
      'import.meta.env.APP_CANISTER_ID_UI': JSON.stringify(canisters.ui),
      'import.meta.env.APP_CANISTER_ID_CONTROL_PANEL': JSON.stringify(canisters.control_panel),
      'import.meta.env.APP_CANISTER_ID_INTERNET_IDENTITY': JSON.stringify(
        canisters.internet_identity,
      ),
      'import.meta.env.APP_PROVIDER_URL_INTERNET_IDENTITY': isProduction
        ? process.env.APP_PROVIDER_URL_INTERNET_IDENTITY
        : JSON.stringify(`http://${canisters.internet_identity}.localhost:4943`),
      'process.env.CANISTER_ID_CONTROL_PANEL': JSON.stringify(canisters.control_panel),
      'process.env.CANISTER_ID_ICP_INDEX': JSON.stringify(canisters.icp_index),
    },
    resolve: {
      alias: {
        '~': resolve('src'),
      },
    },
  };
});
