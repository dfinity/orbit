import { existsSync, lstatSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from 'fs';
import { basename, join } from 'path';

export function copyFileSync(source: string, target: string) {
  let targetFile = target;

  // If target is a directory, a new file with the same name will be created
  if (existsSync(target)) {
    if (lstatSync(target).isDirectory()) {
      targetFile = join(target, basename(source));
    }
  }

  writeFileSync(targetFile, readFileSync(source));
}

/**
 * Copy folder recursively to target path.
 */
export function copyFolderRecursiveSync(
  source: string,
  target: string,
  ignoreIfMatch: RegExp[] = [],
  useBasepath = false,
) {
  let files = [];

  // Check if folder needs to be created or integrated
  const targetFolder = useBasepath ? join(target, basename(source)) : target;
  if (!existsSync(targetFolder)) {
    mkdirSync(targetFolder);
  }

  // Copy files and directories
  if (lstatSync(source).isDirectory()) {
    files = readdirSync(source);
    for (const file of files) {
      const curSource = join(source, file);

      if (ignoreIfMatch.some(pattern => pattern.test(curSource))) {
        continue;
      }

      if (lstatSync(curSource).isDirectory()) {
        copyFolderRecursiveSync(curSource, targetFolder, ignoreIfMatch, true);
      } else {
        copyFileSync(curSource, targetFolder);
      }
    }
  }
}
