import { existsSync, readFileSync, statSync } from 'fs';
import { resolve } from 'path';
import { Plugin } from 'vite';
import { ENV_PREFIX, PRODUCTION } from '../core/configs.core';

// The default canister IDs that are used in the wallet application.
const DEFAULT_CANISTER_IDS = {
  internet_identity: 'rdmx6-jaaaa-aaaaa-aaadq-cai',
  icp_index: 'qhbym-qaaaa-aaaaa-aaafq-cai',
  control_panel: '65evf-oqaaa-aaaal-add6q-cai',
  app_wallet: '6uh6z-yyaaa-aaaal-add7a-cai',
};

const resolveCanisterIds = (
  icpNetwork: string,
  dfxConfigDirPath: string,
): typeof DEFAULT_CANISTER_IDS => {
  const canisters = Object.assign({}, DEFAULT_CANISTER_IDS);
  const dfxConfigFile =
    icpNetwork === 'local'
      ? resolve(dfxConfigDirPath, '.dfx/local/canister_ids.json')
      : resolve(dfxConfigDirPath, 'canister_ids.json');

  if (!existsSync(dfxConfigFile)) {
    console.warn(
      `Canister ids file not found at ${dfxConfigFile}, fallback to default canister ids.`,
    );

    return canisters;
  }

  if (!statSync(dfxConfigFile).isFile()) {
    throw new Error(`Canister ids file at ${dfxConfigFile} is not a file.`);
  }

  const config: Record<string, Record<string, string>> = JSON.parse(
    readFileSync(dfxConfigFile, 'utf-8'),
  );
  for (const [canisterName] of Object.entries(canisters)) {
    const details = config[canisterName];
    if (!details?.[icpNetwork]) {
      console.warn(
        `Canister ${canisterName} does not have a defined canister id for ${icpNetwork}`,
      );
      continue;
    }

    canisters[canisterName] = details[icpNetwork];
  }

  return canisters;
};

const findDfxConfigBasePath = (basePath: string): string => {
  let currentDir = basePath;
  while (currentDir !== '/') {
    const dfxConfigFilePath = resolve(currentDir, 'dfx.json');
    if (existsSync(dfxConfigFilePath) && statSync(dfxConfigFilePath).isFile()) {
      return currentDir;
    }

    currentDir = resolve(currentDir, '..');
  }

  throw new Error('Could not find the dfx config file directory.');
};

export const withCanisterIds = (
  opts: {
    defaultCanisterIds?: typeof DEFAULT_CANISTER_IDS;
    dfxConfigDirPath?: string;
    network?: string;
    isProduction?: boolean;
    envPrefix?: string;
  } = {},
): Plugin => {
  return {
    name: 'with-canister-ids',
    config(config) {
      const basePath = resolve(process.cwd(), config.root || '.');
      const icpNetwork =
        opts.network || process.env.BUILD_MODE || process.env.DFX_NETWORK || 'local';
      const dfxConfigDirPath = opts.dfxConfigDirPath
        ? resolve(basePath, opts.dfxConfigDirPath)
        : findDfxConfigBasePath(basePath);
      const isProduction = opts.isProduction || PRODUCTION;
      const envPrefix = opts.envPrefix || ENV_PREFIX;

      const canisters = resolveCanisterIds(icpNetwork, dfxConfigDirPath);
      const canisterEnvVars = Object.entries(canisters).reduce((acc, [key, value]) => {
        acc[`import.meta.env.${envPrefix}CANISTER_ID_${key.toUpperCase()}`] = JSON.stringify(value);
        // This is to support the generated actors that use the process.env.CANISTER_ID_* variables.
        acc[`process.env.CANISTER_ID_${key.toUpperCase()}`] = JSON.stringify(value);
        return acc;
      }, {});

      const buildEnvVars = {
        ...canisterEnvVars,
        'import.meta.env.APP_PROVIDER_URL_INTERNET_IDENTITY': isProduction
          ? JSON.stringify('https://identity.ic0.app')
          : JSON.stringify(`http://${canisters.internet_identity}.localhost:4943`),
      };

      return {
        define: { ...buildEnvVars },
        test: {
          env: {
            ...Object.entries(buildEnvVars).reduce((acc, [key, value]) => {
              const keyParts = key.split('.');
              if (keyParts.length) {
                acc[keyParts.pop()] = JSON.parse(value);
              }

              return acc;
            }, {}),
          },
        },
      };
    },
  };
};
