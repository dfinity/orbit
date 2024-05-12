import { execSync } from 'child_process';
import { createCommand } from 'commander';
import { readFileSync } from 'fs';
import { fileExists } from 'nx/src/utils/fileutils';
import { isAbsolute, join } from 'path';
import { ReleaseDetails } from './types';
import { gitTagExists } from '../utils';

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

  // filter out projects that already have a release tag associated with them
  const projectsWithoutReleaseTags = Object.entries(release.changes ?? {}).filter(
    ([, changelog]) => !gitTagExists(changelog.releaseVersion.gitTag),
  );

  for (const [project, changelog] of projectsWithoutReleaseTags) {
    console.log(`Creating release tag for project: ${project}...`);
    const releaseTagMessage = changelog.releaseVersion.isPrerelease
      ? `Pre-release ${changelog.releaseVersion.rawVersion}`
      : `Release ${changelog.releaseVersion.rawVersion}`;

    execSync(`git tag "${changelog.releaseVersion.gitTag}" -m "${releaseTagMessage}"`);
  }

  // push tags to remote and force update
  // execSync('git push --tags --force');
});

export default command;
