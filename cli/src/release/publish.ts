import { execSync } from 'child_process';
import { createCommand } from 'commander';
import { existsSync, readFileSync } from 'fs';
import { isAbsolute, join } from 'path';
import configuration from '../config';
import { capitalize, gitTagExists, targetExists } from '../utils';
import { ReleaseDetails } from './types';

const artifactsRootPath = join(__dirname, '../../../artifacts');

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

  if (!existsSync(releaseFilePath)) {
    console.warn(`The release file at ${releaseFilePath} does not exist. Skipping release.`);

    return;
  }

  const release = JSON.parse(readFileSync(releaseFilePath, 'utf-8')) as ReleaseDetails;

  // filter out projects that already have a release tag associated with them
  const projectsWithoutReleaseTags = Object.entries(release.changes ?? {}).filter(
    ([, changelog]) => !gitTagExists(changelog.releaseVersion.gitTag),
  );

  // create release artifacts for projects
  for (const [project, _] of projectsWithoutReleaseTags) {
    if (targetExists(project, 'create-artifacts')) {
      console.log(`Creating release artifacts for project: ${project}...`);
      execSync(`BUILD_MODE='${configuration.icp.network}' ./scripts/docker-build.sh --${project}`);
    }
  }

  // create release tags for projects
  for (const [project, changelog] of projectsWithoutReleaseTags) {
    console.log(`Creating release tag for project: ${project}...`);

    execSync(
      `git tag "${changelog.releaseVersion.gitTag}" -m "Release ${changelog.releaseVersion.rawVersion}"`,
    );
  }

  execSync('git push origin --tags');

  for (const [project, changelog] of projectsWithoutReleaseTags) {
    console.log(`Creating release page for project: ${project}...`);
    const releaseTitle =
      capitalize(project.replace(/[_-]/g, ' '), true) + ' ' + changelog.releaseVersion.rawVersion;

    let ghReleaseCommand = `gh release create "${changelog.releaseVersion.gitTag}"`;
    if (targetExists(project, 'create-artifacts')) {
      ghReleaseCommand += ` ${artifactsRootPath}/${project}/*`;
    }

    ghReleaseCommand += ` -t "${releaseTitle}" -n "${changelog.contents}"`;
    if (changelog.releaseVersion.isPrerelease) {
      ghReleaseCommand += ' --prerelease';
    }

    execSync(ghReleaseCommand);
  }
});

export default command;
