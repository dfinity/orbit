import { RegistryEntry, SearchRegistryResult } from '../generated/control_panel';
import { callControlPanel, getCanisterId } from './icp';
import { Principal } from '@dfinity/principal';

export enum Application {
  Station = 'station',
  Upgrader = 'upgrader',
}

export const getWasmChunkStoreId = (network: string = 'local'): Principal =>
  getCanisterId('wasm_chunk_store', network);

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
    const result = await callControlPanel<SearchRegistryResult>({
      method: 'search_registry',
      network: opts.network,
      identity: opts.identity,
      query: true,
      arg: `(record {
        pagination = opt record { offset = opt ${offset}; limit = opt 50; };
        sort_by = opt variant { Version = variant { Desc } };
        filter_by = vec {
          variant { Name = "${opts.name}" }
        };
      })`,
    });
    if ('Err' in result) {
      throw new Error(`Failed to search the registry: ${JSON.stringify(result.Err)}`);
    }

    const response = result.Ok;
    offset = Number(response.next_offset?.[0]) ?? -1;
    entries.push(...response.entries);
  } while (offset > 0);

  return entries;
};
