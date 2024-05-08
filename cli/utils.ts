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
