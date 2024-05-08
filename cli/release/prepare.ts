import { createCommand } from 'commander';
import { writeFile } from 'fs/promises';
import { releaseChangelog, releaseVersion } from 'nx/release';
import { dirname, isAbsolute, join } from 'path';
import { fileURLToPath } from 'url';
import { parseArgsListSplitByComma } from '../utils';
import { exec } from 'child_process';

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

const command = createCommand('prepare').description(
  'Handles versioning of projects and generating the changelogs.',
);

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
    '.release.json',
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

  if (!options.dryRun) {
    const releaseOutputPath = isAbsolute(options.releaseOutput)
      ? options.releaseOutput
      : join(__dirname, '../..', options.releaseOutput);

    console.log(`Writing release information to ${releaseOutputPath}`);

    await writeFile(
      releaseOutputPath,
      JSON.stringify(
        {
          __important__: 'DO NOT MODIFY THIS FILE. This file is automatically generated.',
          workspaceVersion: workspaceVersion,
          projectsVersionData: projectsVersionData,
        },
        null,
        2,
      ),
    );

    exec(`git add ${releaseOutputPath}`);
  }

  await releaseChangelog({
    dryRun: options.dryRun,
    firstRelease: options.firstRelease,
    projects: options.projects,
    verbose: options.verbose,
    versionData: projectsVersionData,
    version: workspaceVersion,
    createRelease: false,
    gitTag: false,
  });
});

export default command;
