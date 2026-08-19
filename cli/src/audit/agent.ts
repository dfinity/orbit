import { Actor, ActorSubclass, HttpAgent, Identity } from '@dfinity/agent';
import { readFile } from 'fs/promises';
import { join } from 'path';
import { ROOT_PATH } from '../utils';
// `idlFactory` is the runtime IDL constructor generated from
// `core/station/api/spec.did`. Both the .js and its companion .d.ts live in
// `./generated/`; regenerate them with `pnpm run generate-station-types`.
import { idlFactory } from './generated/station.did.js';

const MAINNET_HOSTS: Record<string, string> = {
  ic: 'https://icp-api.io',
};

const resolveHost = async (network: string): Promise<string> => {
  if (MAINNET_HOSTS[network]) return MAINNET_HOSTS[network];

  // Fall back to dfx.json for local-replica style networks.
  const dfxJsonPath = join(ROOT_PATH, 'dfx.json');
  const raw = await readFile(dfxJsonPath, 'utf8').catch(() => null);
  if (!raw) {
    throw new Error(`Unknown network '${network}' and dfx.json not found at ${dfxJsonPath}.`);
  }
  const dfxJson = JSON.parse(raw) as {
    networks?: Record<string, { providers?: string[]; bind?: string }>;
  };
  const cfg = dfxJson.networks?.[network];
  if (!cfg) {
    throw new Error(`Network '${network}' is not defined in dfx.json.`);
  }
  if (cfg.providers && cfg.providers.length > 0) return cfg.providers[0];
  if (cfg.bind) return cfg.bind.startsWith('http') ? cfg.bind : `http://${cfg.bind}`;
  throw new Error(`Network '${network}' has no providers or bind URL configured.`);
};

export const createStationActor = async (
  canisterId: string,
  network: string,
  identity: Identity,
): Promise<ActorSubclass> => {
  const host = await resolveHost(network);
  const agent = new HttpAgent({ host, identity });
  if (network !== 'ic') {
    // Local replicas need their root key fetched to verify certificates.
    await agent.fetchRootKey();
  }
  return Actor.createActor(idlFactory, { agent, canisterId });
};
