import { execSync } from 'child_process';
import { RegistryEntry, SearchRegistryResult } from '../generated/control_panel';
import { execAsync } from '../utils';
import { Principal } from '@dfinity/principal';

export enum Application {
  Station = 'station',
  Upgrader = 'upgrader',
}

export const getWasmChunkStoreId = (network: string = 'local'): Principal => {
  const maybeCanisterId = execSync(`dfx canister id wasm_chunk_store --network ${network}`)
    .toString()
    .trim();

  return Principal.fromText(maybeCanisterId);
};

export const applicationToRegistryEntryMap: Record<Application, string> = {
  [Application.Station]: '@orbit/station',
  [Application.Upgrader]: '@orbit/upgrader',
};

export const registryEntryToApplicationMap: Record<string, Application> = Object.entries(
  applicationToRegistryEntryMap,
).reduce(
  (acc, [key, value]) => {
    acc[value] = key as Application;

    return acc;
  },
  {} as Record<string, Application>,
);

export const parseRegistryApplication = (value: string): Application => {
  for (const app of Object.values(Application)) {
    if (app === value) {
      return app;
    }
  }

  throw new Error(
    `Invalid application. Must be one of: '${Object.values(Application).join(', ')}'.`,
  );
};

export const searchRegistry = async (opts: {
  name: string;
  network: string;
  identity: string;
}): Promise<RegistryEntry[]> => {
  let offset = 0;
  const entries: RegistryEntry[] = [];
  do {
    const unparsed = await execAsync(`
      dfx canister call --identity '${opts.identity}' --network '${opts.network}' --output json control_panel search_registry 'record {
        pagination = opt record { offset = opt ${offset}; limit = opt 50; };
        sort_by = opt variant { Version = variant { Desc } };
        filter_by = vec {
          variant { Name = "${opts.name}" }
        };
      }'
    `);

    const result: SearchRegistryResult = JSON.parse(unparsed);
    if ('Err' in result) {
      throw new Error(`Failed to search the registry: ${JSON.stringify(result.Err)}`);
    }

    const response = result.Ok;
    offset = Number(response.next_offset?.[0]) ?? -1;
    entries.push(...response.entries);
  } while (offset > 0);

  return entries;
};
