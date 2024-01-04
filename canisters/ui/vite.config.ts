import { NodeModulesPolyfillPlugin } from '@esbuild-plugins/node-modules-polyfill';
import inject from '@rollup/plugin-inject';
import vue from '@vitejs/plugin-vue';
import { existsSync, readFileSync, readdirSync } from 'fs';
import { basename, dirname, resolve } from 'path';
import vuetify from 'vite-plugin-vuetify';
import { defineConfig } from 'vite';
import dfxConfig from '../../dfx.json';

const network = process.env.DFX_NETWORK ?? 'local';

const resolveCanisterIds = (): Map<string, string> => {
  const availableCanisters = new Map<string, string>();
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
    if (!details?.[network]) {
      console.warn(`Canister ${canisterName} does not have a defined canister id for ${network}`);
      continue;
    }

    availableCanisters.set(canisterName, details[network]);
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
  const supportedLocales = readdirSync(localesPath).map(file => basename(file, '.json'));
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
    optimizeDeps: {
      esbuildOptions: {
        define: {
          global: 'globalThis',
        },
        plugins: [
          NodeModulesPolyfillPlugin(),
          {
            name: 'fix-node-globals-polyfill',
            setup(build) {
              build.onResolve({ filter: /_virtual-process-polyfill_\.js/ }, ({ path }) => ({
                path,
              }));
            },
          },
        ],
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
      'import.meta.env.APP_CANISTER_ID_UI': JSON.stringify(canisters.get('wallet_ui')),
      'import.meta.env.APP_CANISTER_ID_CONTROL_PANEL': JSON.stringify(
        canisters.get('control_panel'),
      ),
      'import.meta.env.APP_CANISTER_ID_INTERNET_IDENTITY': JSON.stringify(
        canisters.get('internet_identity'),
      ),
      'import.meta.env.APP_PROVIDER_URL_INTERNET_IDENTITY': isProduction
        ? process.env.APP_PROVIDER_URL_INTERNET_IDENTITY
        : JSON.stringify(`http://${canisters.get('internet_identity')}.localhost:4943`),
      'process.env.CANISTER_ID_CONTROL_PANEL': JSON.stringify(canisters.get('control_panel')),
      'process.env.CANISTER_ID_ICP_INDEX': JSON.stringify(canisters.get('icp_index')),
    },
    resolve: {
      alias: {
        '~': resolve('src'),
      },
    },
  };
});
