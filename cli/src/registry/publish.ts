import { execSync } from 'child_process';
import { createCommand } from 'commander';
import { rmSync } from 'fs';
import { writeFile } from 'fs/promises';
import { tmpdir } from 'os';
import { join } from 'path';
import { AddRegistryEntryResult, EditRegistryEntryResult } from '../generated/control_panel';
import {
  assertReplicaIsHealthy,
  cargoProjectVersion,
  execAsync,
  readFileIntoUint8Array,
  toBlobString,
} from '../utils';
import {
  Application,
  applicationToRegistryEntryMap,
  parseRegistryApplication,
  registryEntryToApplicationMap,
  searchRegistry,
} from './registry.core';

const artifactsRootPath = join(__dirname, '../../../artifacts');

interface PackageDedependency {
  name: string;
  version: string;
}

interface Package {
  name: string;
  description: string;
  tags: string[];
  categories: string[];
  metadata: Record<string, string>;
  version: string;
  module: Uint8Array;
  dependencies: PackageDedependency[];
}

const getAppRegistryEntry = async (
  app: Application,
  network: string,
  shouldBuildArtifacts: boolean,
): Promise<Package> => {
  const upgraderVersion = cargoProjectVersion(Application.Upgrader);

  switch (app) {
    case Application.Station: {
      const stationVersion = cargoProjectVersion(Application.Station);

      if (shouldBuildArtifacts) {
        console.log(`Creating release artifacts for project: station...`);
        execSync(`BUILD_MODE='${network}' npx nx run station:create-artifacts`);
      }

      return {
        name: applicationToRegistryEntryMap.station,
        description:
          'A secure and multichain asset management platform for teams and institutions.',
        tags: ['latest'],
        categories: ['finance', 'iam', 'multichain', 'asset-management'],
        dependencies: [
          {
            name: applicationToRegistryEntryMap.upgrader,
            version: upgraderVersion,
          },
        ],
        module: await readFileIntoUint8Array(join(artifactsRootPath, 'station/station.wasm.gz')),
        version: stationVersion,
        metadata: {
          url: 'https://github.com/dfinity/orbit',
        },
      };
    }
    case Application.Upgrader: {
      if (shouldBuildArtifacts) {
        console.log(`Creating release artifacts for project: upgrader...`);
        execSync(`BUILD_MODE='${network}' npx nx run upgrader:create-artifacts`);
      }

      return {
        name: applicationToRegistryEntryMap.upgrader,
        description: 'Securely handles upgrades and disaster recovery for the Orbit Station.',
        tags: ['latest'],
        categories: ['finance', 'iam', 'multichain', 'asset-management'],
        dependencies: [],
        module: await readFileIntoUint8Array(join(artifactsRootPath, 'upgrader/upgrader.wasm.gz')),
        version: upgraderVersion,
        metadata: {
          url: 'https://github.com/dfinity/orbit',
        },
      };
    }
    default:
      throw new Error(`Unsupported application: ${app}`);
  }
};

const command = createCommand('publish').description(
  'Handles the publishing of the Orbit canisters to the control-panel registry.',
);

command
  .option(
    '-n, --network <TYPE>',
    'The network to use for the registry operations. Defaults to `local`.',
    'local',
  )
  .option(
    '-i, --identity <TYPE>',
    'The identity to use for the registry operations. Defaults to `default`.',
    'default',
  )
  .option(
    '-b, --build',
    'Whether to build the release artifacts before publishing. Defaults to `false`.',
    false,
  )
  .requiredOption(
    '-a, --app <VALUE>',
    'The application to add to the registry. Must be either `station` or `upgrader`.',
    parseRegistryApplication,
  );

// Saves the argument in a temporary file and returns the path to the file.
const saveArgumentInTempFile = async (argument: string): Promise<string> => {
  const tempFilePath = join(tmpdir(), 'orbit-cli-argument-' + Math.random().toString(36).slice(2));

  await writeFile(tempFilePath, argument, {
    encoding: 'utf-8',
  });

  return tempFilePath;
};

command.action(async options => {
  assertReplicaIsHealthy(options.network);

  // Determine whether to build the release artifacts before publishing. This is `false` by default since
  // artifacts are expected to be built before with a deterministic build.
  const shouldBuildArtifacts = options.build;

  // Prepare the registry entries for the application and its dependencies to be added.
  console.log(`Preparing registry entries for ${options.app} and its dependencies...`);
  const app = await getAppRegistryEntry(options.app, options.network, shouldBuildArtifacts);
  const registryEntries: Package[] = [];
  for (const dependency of app.dependencies) {
    registryEntries.push(
      await getAppRegistryEntry(
        registryEntryToApplicationMap[dependency.name],
        options.network,
        shouldBuildArtifacts,
      ),
    );
  }
  registryEntries.push(app);

  // Add the registry entries.
  console.log(`Adding registry entries for ${options.app} and its dependencies...`);
  for (const entry of registryEntries) {
    const foundEntries = await searchRegistry({
      name: entry.name,
      network: options.network,
      identity: options.identity,
    });

    const maybeRegistryId = foundEntries.find(
      foundEntry =>
        'WasmModule' in foundEntry.value && foundEntry.value.WasmModule.version === entry.version,
    )?.id;

    if (maybeRegistryId) {
      console.log(`Updating the registry entry for ${entry.name} with id(${maybeRegistryId})...`);
      const argumentFile = await saveArgumentInTempFile(`
        record {
          id = "${maybeRegistryId}";
          entry = record {
            description = opt "${entry.description}";
            tags = opt vec { ${entry.tags.map(tag => `"${tag}"`).join('; ')} };
            categories = opt vec { ${entry.categories.map(category => `"${category}"`).join('; ')} };
            metadata = opt vec {
              record {
                key = "url";
                value = "${entry.metadata.url}";
              }
            };
            value = opt variant {
              WasmModule = record {
                version = "${entry.version}";
                wasm_module = blob "${toBlobString(entry.module)}";
                dependencies = vec { ${entry.dependencies.map(dependency => `record { name = "${dependency.name}"; version = "${dependency.version}"; }`).join('; ')} }
              }
            }
          }
        }  
      `);

      const unparsed = await execAsync(`
        dfx canister call --identity '${options.identity}' --network '${options.network}' --output json control_panel edit_registry_entry --argument-file '${argumentFile}'
      `);

      rmSync(argumentFile);

      const result: EditRegistryEntryResult = JSON.parse(unparsed);
      if ('Err' in result) {
        throw new Error(`Failed to update the registry entry: ${JSON.stringify(result.Err)}`);
      }

      console.log(`Registry with id ${maybeRegistryId} has been updated for ${entry.name}.`);
    } else {
      console.log(`Adding the registry entry for ${entry.name}...`);
      const argumentFile = await saveArgumentInTempFile(`
        record {
          entry = record {
            name = "${entry.name}";
            description = "${entry.description}";
            tags = vec { ${entry.tags.map(tag => `"${tag}"`).join('; ')} };
            categories = vec { ${entry.categories.map(category => `"${category}"`).join('; ')} };
            metadata = vec {
              record {
                key = "url";
                value = "${entry.metadata.url}";
              }
            };
            value = variant {
              WasmModule = record {
                version = "${entry.version}";
                wasm_module = blob "${toBlobString(entry.module)}";
                dependencies = vec { ${entry.dependencies.map(dependency => `record { name = "${dependency.name}"; version = "${dependency.version}"; }`).join('; ')} }
              }
            }
          }
        }
      `);

      const unparsed = await execAsync(`
        dfx canister call --identity '${options.identity}' --network '${options.network}' --output json control_panel add_registry_entry --argument-file '${argumentFile}'
      `);

      rmSync(argumentFile);

      const result: AddRegistryEntryResult = JSON.parse(unparsed);
      if ('Err' in result) {
        throw new Error(`Failed to add the registry entry: ${JSON.stringify(result.Err)}`);
      }

      console.log(`Registry entry for ${entry.name} has been added with id ${result.Ok.entry.id}.`);
    }
  }
});

export default command;
