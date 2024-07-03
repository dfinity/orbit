import { exec, execSync } from 'child_process';
import { readFile } from 'fs/promises';
import { promisify } from 'util';

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

export const execAsync = async (command: string): Promise<string> => {
  return promisify(exec)(command).then(({ stdout }) => stdout);
};

export const cargoProjectVersion = (projectName: string): string => {
  return execSync(
    `cargo metadata --format-version 1 | jq -r '.packages[] | select(.name == "${projectName}") | .version'`,
  )
    .toString()
    .trim();
};

export const readFileIntoUint8Array = async (filePath: string): Promise<Uint8Array> => {
  const buffer = await readFile(filePath);
  const uint8Array = new Uint8Array(buffer);

  return uint8Array;
};

export const toBlobString = (buffer: Uint8Array): string => {
  return Array.from(buffer)
    .map(byte => `\\${byte.toString(16).padStart(2, '0')}`)
    .join('');
};

export const assertReplicaIsHealthy = async (network: string): Promise<void> => {
  const ping: { replica_health_status?: string } = JSON.parse(
    await execAsync(`dfx ping '${network}'`),
  );

  if (ping.replica_health_status?.toLowerCase() !== 'healthy') {
    throw new Error('The replica is not healthy.');
  }
};
