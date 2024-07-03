import { createCommand } from 'commander';
import registryPublish from './publish';
import registryRemove from './remove';
import registryList from './list';

const command = createCommand('registry').description(
  'Manage entries to the control-panel registry.',
);

command.addCommand(registryList);
command.addCommand(registryPublish);
command.addCommand(registryRemove);

export default command;
