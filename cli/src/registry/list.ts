import { createCommand } from 'commander';
import { assertReplicaIsHealthy } from '../utils';
import {
  Application,
  applicationToRegistryEntryMap,
  parseRegistryApplication,
  searchRegistry,
} from './registry.core';

const command = createCommand('list').description(
  'List entries from the control-panel registry of a given application.',
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
  .requiredOption(
    '-a, --app <VALUE>',
    'The application to list from the registry.',
    parseRegistryApplication,
  );

command.action(async options => {
  assertReplicaIsHealthy(options.network);

  const entries = await searchRegistry({
    name: applicationToRegistryEntryMap[options.app as Application],
    network: options.network,
    identity: options.identity,
  });

  for (const entry of entries) {
    console.log('#'.repeat(80));
    console.log(`Id(${entry.id}) ${JSON.stringify(entry, null, 2)}`);
  }
});

export default command;
