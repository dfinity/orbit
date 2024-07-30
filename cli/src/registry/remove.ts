import { createCommand } from 'commander';
import { assertReplicaIsHealthy, execAsync } from '../utils';
import { DeleteRegistryEntryResult } from '~/generated/control_panel';

const command = createCommand('remove').description(
  'Remove entries from the control-panel registry.',
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
  .requiredOption('-k, --id <VALUE>', 'The id of the registry entry to remove.');

command.action(async options => {
  assertReplicaIsHealthy(options.network);

  console.log(`Removing the registry entry with id ${options.id}...`);
  const unparsed = await execAsync(`
    dfx canister call --identity '${options.identity}' --network '${options.network}'  --output json control_panel delete_registry_entry '(record {
      id = "${options.id}"
    })'
  `);

  const result: DeleteRegistryEntryResult = JSON.parse(unparsed);
  if ('Ok' in result) {
    console.log('The registry entry has been removed.');
    return;
  }

  throw new Error(`Failed to remove the registry entry: ${JSON.stringify(result.Err)}`);
});

export default command;
