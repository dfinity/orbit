import { createCommand } from 'commander';
import { dirname, isAbsolute, join } from 'path';
import { fileURLToPath } from 'url';
import { releaseChangelog } from 'nx/release';
import { fileExists } from 'nx/src/utils/fileutils';
import { getCurrentReleaseId } from '../utils';
import { execSync } from 'child_process';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

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
    : join(__dirname, '../..', options.releaseFile);

  if (!releaseFilePath.endsWith('.json')) {
    throw new Error('Invalid release file path. Must be a JSON file.');
  }

  if (!fileExists(releaseFilePath)) {
    throw new Error(`Release file not found at path: ${releaseFilePath}`);
  }

  const { projectsVersionData, releaseId } = await import(releaseFilePath);
  const currentReleaseId = getCurrentReleaseId();

  if (releaseId !== currentReleaseId + 1) {
    throw new Error(
      `The release ID in the release file is not the next release ID. Expected: ${currentReleaseId + 1}, Actual: ${releaseId}`,
    );
  }

  await releaseChangelog({
    verbose: options.verbose,
    versionData: projectsVersionData,
    gitCommit: false,
    gitTag: true,
    firstRelease: true,
    createRelease: 'github',
  });

  execSync(`git tag release-${releaseId}`);
});

export default command;
