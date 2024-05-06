import { releaseChangelog, releaseVersion } from 'nx/release';
import { program } from 'commander';

program
  .description('Release a new version of projects in the workspace')
  .option(
    '-d, --dryRun',
    'Whether or not to perform a dry-run of the release process, defaults to true',
    true,
  )
  .option('-v, --verbose', 'Whether or not to log verbose output, defaults to false', false)
  .option('-f, --firstRelease', 'Whether or not this is the first release of the project', false)
  .action(async options => {
    console.log('options', options);
    const { workspaceVersion, projectsVersionData } = await releaseVersion({
      dryRun: options.dryRun,
      verbose: options.verbose,
      firstRelease: options.firstRelease,
    });

    console.log('workspaceVersion', workspaceVersion);
    console.log('projectsVersionData', projectsVersionData);

    // await releaseChangelog({
    //   versionData: projectsVersionData,
    //   version: workspaceVersion,
    //   dryRun: options.dryRun,
    //   verbose: options.verbose,
    // });
  })
  .parse();
