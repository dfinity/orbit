import { execSync } from 'child_process';

// Get the HEAD commit hash of the current git repository.
export function getCommitHash() {
  try {
    return execSync('git rev-parse --short HEAD').toString().trim();
  } catch (e) {
    console.error('Failed to get commit hash:', e);

    process.exit(1);
  }
}
