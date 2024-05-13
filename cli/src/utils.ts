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

export const gitTagExists = (tag: string): boolean => {
  const output = execSync(`git tag --list "${tag}"`).toString().trim();

  return output === tag;
};

export const capitalize = (str: string, lower = false) =>
  (lower ? str.toLowerCase() : str).replace(/(?:^|\s|["'([{])+\S/g, match => match.toUpperCase());

export const targetExists = (project: string, target: string): boolean => {
  const output = execSync(`npx nx show projects --withTarget="${target}" -p "${project}"`)
    .toString()
    .trim();

  return output === project;
};
