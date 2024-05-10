import { execSync } from 'child_process';

// Parse a string of arguments separated by a separator and return an array of strings.
export const parseArgsListSplitByComma = (arg?: string): string[] => {
  if (!arg) {
    return [];
  }

  return arg.split(',').map((group: string) => group.trim());
};

// Hash a string using the djb2 algorithm.
export const hashString = (str: string): string => {
  let hash = 5381;
  for (let i = 0; i < str.length; i++) {
    const char = str.charCodeAt(i);
    hash = (hash << 5) - hash + char;
    hash = hash & hash;
  }
  return hash.toString();
};

export const getCurrentReleaseId = (): number => {
  try {
    const tags = execSync('git tag --list "release-*"').toString().trim();
    if (!tags) {
      return 0;
    }

    const tag = execSync('git describe --tags --match "release-*" --abbrev=0').toString().trim();
    const releaseId = parseInt(tag.replace('release-', ''), 10);
    if (isNaN(releaseId)) {
      console.error('Failed to parse release ID from tag:', tag);
      return 0;
    }

    return releaseId;
  } catch (error) {
    throw new Error(`Error retrieving current release ID: ${error}`);
  }
};
