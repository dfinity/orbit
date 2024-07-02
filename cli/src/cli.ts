import { program } from 'commander';
import { version } from '../package.json';
import release from './release';
import registry from './registry';

program
  .storeOptionsAsProperties(false)
  .version(version)
  .name('orbit-cli')
  .description('The Orbit CLI includes tools for managing projects in the workspace')
  .command('path')
  .description('Print the path to the Orbit CLI')
  .action(() => console.log(__dirname));

program.addCommand(release);
program.addCommand(registry);

program
  .parseAsync()
  .then(() => process.exit(0))
  .catch(error => {
    console.error(`The command failed with an error: ${error}`);
    process.exit(1);
  });
