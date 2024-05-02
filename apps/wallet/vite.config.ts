import vue from '@vitejs/plugin-vue';
import { nodePolyfills } from 'vite-plugin-node-polyfills';
import { existsSync, readFileSync, readdirSync, writeFileSync } from 'fs';
import { basename, dirname, resolve } from 'path';
import { defineConfig, loadEnv } from 'vite';
import vuetify from 'vite-plugin-vuetify';
import { execSync } from 'child_process';

// This function is used to determine the build mode based on the environment variables.
//
// The `BUILD_MODE` environment variable is used to determine the build mode.
const getMode = (): string => {
  if (process.env.BUILD_MODE && process.env.BUILD_MODE.length) {
    if (process.env.BUILD_MODE === 'local') {
      return 'localhost';
    }

    return process.env.BUILD_MODE;
  }

  if (process.env.NODE_ENV && process.env.NODE_ENV.length) {
    return process.env.NODE_ENV;
  }

  return 'production';
};

function getCommitHash() {
  try {
    return execSync('git rev-parse --short HEAD').toString().trim();
  } catch (e) {
    console.error('Failed to get commit hash:', e);

    process.exit(1);
  }
}

function initBuildEnv() {
  const env = loadEnv(mode, process.cwd(), 'APP_');
  // For the test environment, we need to set the APP_MODE to test to ensure the correct configuration is loaded.
  if (mode === 'test') {
    env.APP_MODE = 'test';
  }

  env.APP_MODE = env.APP_MODE ?? process.env.NODE_ENV;
  process.env.NODE_ENV = env.APP_MODE;

  if (mode === 'localhost') {
    // Since localhost is deployed in a local replica, we need to set the APP_URL to the local replica URL.
    env.APP_URL = 'http://werw6-ayaaa-aaaaa-774aa-cai.localhost:4943';
    env.APP_MODE = 'development';
    process.env.NODE_ENV = 'development';
  }

  return env;
}

const network = process.env.DFX_NETWORK ?? 'local';
const mode = getMode();
const commitHash = getCommitHash();
const env = initBuildEnv();

const defaultCanisterIds = {
  internet_identity: 'rdmx6-jaaaa-aaaaa-aaadq-cai',
  icp_index: 'qhbym-qaaaa-aaaaa-aaafq-cai',
  control_panel: '65evf-oqaaa-aaaal-add6q-cai',
  app_wallet: '6uh6z-yyaaa-aaaal-add7a-cai',
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

const getContentSecurityPolicy = (isProduction: boolean): string => {
  const csp: Record<string, string[]> = {
    'default-src': ["'none'"],
    'script-src': ["'self'", "'unsafe-eval'"],
    'connect-src': ["'self'", 'https://icp-api.io', 'https://ic0.app', 'https://icp0.io'],
    'img-src': ["'self'", 'data:'],
    'font-src': ["'self'"],
    'object-src': ["'none'"],
    'base-uri': ["'self'"],
    'style-src': ["'self'", "'unsafe-inline'"],
    'media-src': ["'self'", 'data:', 'blob:'],
    'form-action': ["'self'"],
    'frame-ancestors': ["'none'"],
    'upgrade-insecure-requests': [],
  };

  if (!isProduction) {
    csp['connect-src'].push('localhost:4943');
  }

  return Object.entries(csp)
    .map(([key, value]) => {
      return `${key} ${value.join(' ')}`;
    })
    .join('; ');
};

const generateICAssetsJson = (
  isProduction: boolean,
  assetsDir = 'public',
  fileName = '.ic-assets.json',
) => {
  const assetsJsonDir = resolve(__dirname, assetsDir, fileName);
  const assetsConfig = {
    well_known: {
      match: '.well-known',
      ignore: false,
    },
    all: {
      match: '**/*',
      headers: {
        'X-Frame-Options': 'DENY',
        'X-Content-Type-Options': 'nosniff',
        'Referrer-Policy': 'same-origin',
        'Content-Security-Policy': getContentSecurityPolicy(isProduction),
        'Strict-Transport-Security': 'max-age=31536000; includeSubDomains',
        'X-XSS-Protection': '1; mode=block',
      },
      allow_raw_access: false,
    },
    fonts: {
      match: '**/fonts/**/*',
      headers: {
        'Cache-Control': 'max-age=31536000',
      },
    },
    assets: {
      match: '**/assets/**/*',
      headers: {
        'Cache-Control': 'max-age=604800',
      },
    },
    images: {
      match: '**/images/**/*',
      headers: {
        'Cache-Control': 'max-age=604800',
      },
    },
  };

  const icAssetsJson = Object.values(assetsConfig);
  writeFileSync(assetsJsonDir, JSON.stringify(icAssetsJson, null, 2), {
    encoding: 'utf-8',
  });
};

// https://vitejs.dev/config/
export default defineConfig(_ => {
  const isProduction = process.env.NODE_ENV === 'production';
  const localesPath = resolve(__dirname, 'src/locales');
  const supportedLocales = readdirSync(localesPath).map(file => basename(file, '.locale.ts'));
  const canisters = resolveCanisterIds();

  // Generate .ic-assets.json file, which is used to configure the asset canister headers.
  generateICAssetsJson(isProduction);

  // Determine if the build is optimized based on the build mode, localhost is opmitized to match the production build
  // when deploying to a local replica.
  const productionBuild = isProduction || mode === 'localhost';

  return {
    mode: mode,
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
      sourcemap: !productionBuild,
      minify: productionBuild,
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
              supportedLocales.some(locale => resolve(folder, `${locale}.locale.ts`) === id)
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
      devSourcemap: !productionBuild,
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
      'import.meta.env.PROD': isProduction,
      'import.meta.env.DEV': !isProduction,
      'import.meta.env.APP_MODE': JSON.stringify(env.APP_MODE),
      'import.meta.env.APP_URL': JSON.stringify(env.APP_URL),
      'import.meta.env.APP_BUILD_MODE': JSON.stringify(mode),
      'import.meta.env.APP_BUILD_VERSION': JSON.stringify(process.env.npm_package_version),
      'import.meta.env.APP_BUILD_HASH': JSON.stringify(commitHash),
      'import.meta.env.APP_BUILD_DATE': JSON.stringify(new Date().toISOString()),
      'import.meta.env.APP_CANISTER_ID_APP_WALLET': JSON.stringify(canisters.app_wallet),
      'import.meta.env.APP_CANISTER_ID_CONTROL_PANEL': JSON.stringify(canisters.control_panel),
      'import.meta.env.APP_CANISTER_ID_INTERNET_IDENTITY': JSON.stringify(
        canisters.internet_identity,
      ),
      'import.meta.env.APP_PROVIDER_URL_INTERNET_IDENTITY': isProduction
        ? JSON.stringify(env.APP_PROVIDER_URL_INTERNET_IDENTITY)
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
