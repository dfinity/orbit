import { existsSync, readFileSync } from 'fs';
import { DEFAULT_CANISTER_IDS, DFX_CONFIG_PATH, NETWORK } from './configs.core';
import { resolve } from 'path';

export const resolveCanisterIds = (): typeof DEFAULT_CANISTER_IDS => {
  const dfxConfig: {
    canisters: Record<string, unknown>;
  } = JSON.parse(readFileSync(DFX_CONFIG_PATH, 'utf-8'));
  const availableCanisters = Object.assign({}, DEFAULT_CANISTER_IDS);
  const canisters = Object.entries(dfxConfig.canisters);
  const canisterIdsFilePath =
    NETWORK === 'local'
      ? resolve(__dirname, '../../../..', '.dfx/local/canister_ids.json')
      : resolve(__dirname, '../../../..', 'canister_ids.json');

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

    if (!details?.[NETWORK]) {
      console.warn(`Canister ${canisterName} does not have a defined canister id for ${NETWORK}`);
      continue;
    }

    availableCanisters[canisterName] = details[NETWORK];
  }

  return availableCanisters;
};
