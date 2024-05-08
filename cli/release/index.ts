import { createCommand } from 'commander';
import releaseVersion from './version';

const command = createCommand('release').description('Handle the release of projects');

command.addCommand(releaseVersion);

export default command;
