import { readdirSync } from 'fs';
import { basename, resolve } from 'path';
import { loadEnv } from 'vite';
import packageJson from '../../package.json';
import { execSync } from 'child_process';

export const LOCALTES_PATH = resolve(__dirname, '../..', 'src/locales');
export const SUPPORTED_LOCALTES = readdirSync(LOCALTES_PATH).map(file =>
  basename(file, '.locale.ts'),
);

export const DFX_CONFIG_PATH = resolve(__dirname, '../../../..', 'dfx.json');
export const NETWORK = process.env.DFX_NETWORK || 'local';

// The default canister IDs that are used in the wallet.
export const DEFAULT_CANISTER_IDS = {
  internet_identity: 'rdmx6-jaaaa-aaaaa-aaadq-cai',
  icp_index: 'qhbym-qaaaa-aaaaa-aaafq-cai',
  control_panel: '65evf-oqaaa-aaaal-add6q-cai',
  app_wallet: '6uh6z-yyaaa-aaaal-add7a-cai',
};

// The current version of the wallet dapp.
export const WALLET_VERSION = packageJson.version;

// Gets the current version of the station API that the wallet is using.
export const STATION_API_VERSION = execSync(
  "cargo pkgid -p station | awk -F '#' '{print $2}' | awk -F '@' '{print $2}'",
)
  .toString()
  .trim();

// Used to determine the build mode based on the environment variables.
//
// The `BUILD_MODE` environment variable is used to determine the build mode.
export const MODE = ((): string => {
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
})();

// The prefix used to load the environment variables.
export const ENV_PREFIX = 'APP_';

// Load the environment variables based on the build mode and the prefix.
export const ENV: Record<string, string> = (() => {
  // if the prefix is set, make sure to clear any values set in the process.env to not conflict with the loaded values.
  if (ENV_PREFIX.length > 0) {
    for (const key of Object.keys(process.env)) {
      if (key.startsWith(ENV_PREFIX)) {
        delete process.env[key];
      }
    }
  }

  const loadedEnv = loadEnv(MODE, resolve(__dirname, '../..'), ENV_PREFIX);
  // For the test environment, we need to set the APP_MODE to test to ensure the correct configuration is loaded.
  if (MODE === 'test') {
    loadedEnv.APP_MODE = 'test';
  }

  loadedEnv.APP_MODE = loadedEnv.APP_MODE ?? process.env.NODE_ENV;
  process.env.NODE_ENV = loadedEnv.APP_MODE;

  if (MODE === 'localhost') {
    // Since localhost is deployed in a local replica, we need to set the APP_URL to the local replica URL.
    loadedEnv.APP_URL = 'http://werw6-ayaaa-aaaaa-774aa-cai.localhost:4943';
    loadedEnv.APP_MODE = 'development';
    process.env.NODE_ENV = 'development';
  }

  return loadedEnv;
})();

// Wether the build is in production mode.
export const PRODUCTION = process.env.NODE_ENV === 'production';

// Determine if the build is optimized based on the build mode, localhost is opmitized to match the production build
// when deploying to a local replica.
export const OPTIMIZED_BUILD = PRODUCTION || MODE === 'localhost';
