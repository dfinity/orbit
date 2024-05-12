import { createCommand } from 'commander';
import { isAbsolute, join } from 'path';
// import { releaseChangelog } from 'nx/release';
import { fileExists } from 'nx/src/utils/fileutils';
// import { getCurrentReleaseId } from '../utils';
// import { execSync } from 'child_process';
import { readFileSync } from 'fs';
import { ReleaseDetails } from '~/release/types';

const command = createCommand('publish').description(
  'Handles the publishing of a given release. This command should be run after the release has been prepared.',
);

command
  .option('-v, --verbose', 'Whether or not to log verbose output')
  .option(
    '-p, --release-file <TYPE>',
    'Specify the file where the release information is stored',
    '.release.json',
  );

command.action(async options => {
  const releaseFilePath = isAbsolute(options.releaseFile)
    ? options.releaseFile
    : join(__dirname, '../../..', options.releaseFile);

  if (!releaseFilePath.endsWith('.json')) {
    throw new Error('Invalid release file path. Must be a JSON file.');
  }

  if (!fileExists(releaseFilePath)) {
    console.warn(`The release file at ${releaseFilePath} does not exist. Skipping release.`);

    return;
  }

  const release = JSON.parse(readFileSync(releaseFilePath, 'utf-8')) as ReleaseDetails;

  console.log(`Publishing release ${release.releaseId}...`);
  console.log('Versions:', release.versions);
  console.log('Changes:', release.changes);

  // const currentReleaseId = getCurrentReleaseId();
  // const expectedNextReleaseId = currentReleaseId + 1;

  // if (currentReleaseId === releaseId) {
  //   console.log(`The current workspace is already at release ${releaseId}. Skipping release.`);

  //   return;
  // }

  // if (releaseId !== expectedNextReleaseId) {
  //   throw new Error(
  //     `The release ID in the release file is not the next release ID. Expected next release to be ${expectedNextReleaseId}, but was ${releaseId}.`,
  //   );
  // }

  // execSync(`git tag release-${releaseId}`);

  // await releaseChangelog({
  //   verbose: options.verbose,
  //   versionData: versions,
  //   gitCommit: false,
  //   gitTag: true,
  //   firstRelease: true,
  //   createRelease: 'github',
  // });
});

export default command;
