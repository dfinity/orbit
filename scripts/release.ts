import { releaseChangelog, releaseVersion } from 'nx/release';
import { program } from 'commander';
import { execSync } from 'child_process';

// Function to get the first commit of a Git repository synchronously,
// throwing an error if the command fails.
function getFirstRepositoryCommit(): string {
  try {
    const stdout = execSync('git rev-list --max-parents=0 HEAD --reverse | head -1');
    return stdout.toString().trim();
  } catch (error) {
    throw new Error(`Failed to get the first commit: ${error.message}`);
  }
}

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

    // If this is the first release, we don't have a previous version to compare against
    // so we need to get the very first git commit in the history of the repository.
    const from = options.firstRelease ? getFirstRepositoryCommit() : undefined;

    await releaseChangelog({
      versionData: projectsVersionData,
      version: workspaceVersion,
      dryRun: options.dryRun,
      verbose: options.verbose,
      from,
    });
  })
  .parse();
