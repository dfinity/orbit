import { releaseChangelog, releaseVersion } from 'nx/release';
import { program } from 'commander';

// Determine the type of pre-release version to use if applicable, falling back to 'prod' if not provided
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

const parseProjects = (projects?: string): string[] | undefined => {
  if (!projects) {
    return undefined;
  }

  return projects.split(',').map((group: string) => group.trim());
};

program
  .description('Release a new version of projects in the workspace')
  .option(
    '-p, --projects <VALUE>',
    'The release projects to include in the release, separated by commas. If not provided, all projects will be included.',
    parseProjects,
  )
  .option('-d, --dry-run', 'Whether or not to perform a dry-run of the release process')
  .option('-v, --verbose', 'Whether or not to log verbose output')
  .option('-f, --first-release', 'Whether or not this is the first release of the project')
  .option(
    '-P, --pre-release <VALUE>',
    'Specify the type of pre-release version to use (alpha, beta or rc)',
    parsePreReleaseMode,
  )
  .action(async options => {
    const { workspaceVersion, projectsVersionData } = await releaseVersion({
      dryRun: options.dryRun,
      verbose: options.verbose,
      firstRelease: options.firstRelease,
      projects: options.projects,
      specifier: options.preRelease !== 'prod' ? 'prerelease' : undefined,
      preid: options.preRelease !== 'prod' ? options.preRelease : undefined,
    });

    await releaseChangelog({
      versionData: projectsVersionData,
      version: workspaceVersion,
      dryRun: options.dryRun,
      verbose: options.verbose,
      firstRelease: options.firstRelease,
      projects: options.projects,
    });
  })
  .parse();
