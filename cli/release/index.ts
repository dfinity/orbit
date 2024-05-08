import { createCommand } from 'commander';
import releasePrepare from './prepare';

const command = createCommand('release').description('Handle the release of projects');

command.addCommand(releasePrepare);

export default command;
