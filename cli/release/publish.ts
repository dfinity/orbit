import { createCommand } from 'commander';
import { dirname, isAbsolute, join } from 'path';
import { fileURLToPath } from 'url';
import { releaseChangelog } from 'nx/release';
import { fileExists } from 'nx/src/utils/fileutils';

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

  const { projectsVersionData, workspaceVersion } = await import(releaseFilePath);

  await releaseChangelog({
    verbose: options.verbose,
    versionData: projectsVersionData,
    version: workspaceVersion,
    gitCommit: false,
    gitTag: true,
    firstRelease: true,
    createRelease: 'github',
  });
});

export default command;
