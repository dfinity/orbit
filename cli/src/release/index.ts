import { createCommand } from 'commander';
import releasePrepare from './prepare';
import releasePublish from './publish';

const command = createCommand('release').description('Handle the release of projects');

command.addCommand(releasePrepare);
command.addCommand(releasePublish);

export default command;
