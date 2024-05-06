import { releaseChangelog, releaseVersion } from 'nx/release';
import { program } from 'commander';

program
  .description('Release a new version of projects in the workspace')
  .option('-d, --dry-run', 'Whether or not to perform a dry-run of the release process')
  .option('-v, --verbose', 'Whether or not to log verbose output')
  .option('-f, --first-release', 'Whether or not this is the first release of the project')
  .action(async options => {
    const { workspaceVersion, projectsVersionData } = await releaseVersion({
      dryRun: options.dryRun,
      verbose: options.verbose,
      firstRelease: options.firstRelease,
    });

    await releaseChangelog({
      versionData: projectsVersionData,
      version: workspaceVersion,
      dryRun: options.dryRun,
      verbose: options.verbose,
    });
  })
  .parse();
