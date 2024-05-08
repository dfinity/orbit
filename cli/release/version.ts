import { createCommand } from 'commander';
import { releaseVersion } from 'nx/release';
import { dirname, isAbsolute, join } from 'path';
import { parseArgsListSplitByComma } from '../utils';
import { fileURLToPath } from 'url';
import { writeFileSync } from 'fs';

// Convert the import.meta.url to a file path
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const parsePreReleaseMode = (releaseMode?: string): 'alpha' | 'beta' | 'rc' | 'prod' => {
  if (!releaseMode) {
    return 'prod';
  }

  switch (releaseMode) {
    case 'alpha':
      return 'alpha';
    case 'beta':
      return 'beta';
    case 'rc':
      return 'rc';
    default:
      throw new Error(`Invalid pre-release mode: ${releaseMode}`);
  }
};

const command = createCommand('version').description('Handles versioning of projects');

command
  .option(
    '-p, --projects <VALUE>',
    'The release projects to include in the release, separated by commas. If not provided, all projects will be included.',
    parseArgsListSplitByComma,
  )
  .option('-d, --dry-run', 'Whether or not to perform a dry-run of the release process')
  .option('-v, --verbose', 'Whether or not to log verbose output')
  .option('-f, --first-release', 'Whether or not this is the first release of the project')
  .option(
    '-P, --pre-release <VALUE>',
    'Specify the type of pre-release version to use (alpha, beta or rc)',
    parsePreReleaseMode,
  )
  .option(
    '-o, --release-output <TYPE>',
    'Specify where the version information of the release should be stored',
    './.release-version.json',
  );

command.action(async options => {
  const { workspaceVersion, projectsVersionData } = await releaseVersion({
    dryRun: options.dryRun,
    verbose: options.verbose,
    firstRelease: options.firstRelease,
    projects: options.projects,
    specifier: options.preRelease !== 'prod' ? 'prerelease' : undefined,
    preid: options.preRelease !== 'prod' ? options.preRelease : undefined,
  });

  // if (options.dryRun) {
  //   return;
  // }

  const releaseOutputPath = isAbsolute(options.releaseOutput)
    ? options.releaseOutput
    : join(__dirname, '../..', options.releaseOutput);

  const releaseOutput = { workspaceVersion, projectsVersionData };

  writeFileSync(releaseOutputPath, JSON.stringify(releaseOutput, null, 2));
});

export default command;
