import { createCommand } from 'commander';
import releaseBump from './bump';

const command = createCommand('release').description('Handle the release of projects');

command.addCommand(releaseBump);

export default command;
