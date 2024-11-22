import { exec, execSync, spawnSync } from 'child_process';
import { existsSync } from 'fs';
import { readFile } from 'fs/promises';
import { homedir } from 'os';
import { join } from 'path';
import { promisify } from 'util';

export const ROOT_PATH = join(__dirname, '../..');
export const DFX_PATH = join(ROOT_PATH, 'dfx.json');
export const DFX_DEFAULT_IDENTITY_STORE_PATH = join(homedir(), '.config/dfx/identity');

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

export const commandExists = (command: string): boolean => {
  const result = spawnSync('command', ['-v', command], { stdio: 'ignore' });
  return result.status === 0;
};

export const assertCommandExists = (command: string): void => {
  if (!commandExists(command)) {
    throw new Error(`Command '${command}' does not exist.`);
  }
};

export const getReplicaUrl = async (network: string): Promise<string> => {
  const dfxFile = JSON.parse(await readFile(DFX_PATH, 'utf-8'));

  if (!dfxFile?.networks?.[network]) {
    throw new Error(`Network '${network}' not found in dfx.json.`);
  }

  if (dfxFile.networks[network].providers && dfxFile.networks[network].providers.length > 0) {
    return dfxFile.networks[network].providers[0];
  }

  if (dfxFile.networks[network].bind) {
    const bind = dfxFile.networks[network].bind.startsWith('http')
      ? dfxFile.networks[network].bind
      : `http://${dfxFile.networks[network].bind}`;

    const validLocalBinds = ['http://localhost', 'http://127.0.0.1', 'http://[::1]'];
    if (!validLocalBinds.some(validLocalBind => bind.startsWith(validLocalBind))) {
      throw new Error(
        `Network '${network}' has a bind URL that is not a valid local bind: ${bind}.`,
      );
    }

    return bind;
  }

  throw new Error(`Network '${network}' does not have a replica URL.`);
};

export const getIdentityPemFilePath = async (identity: string): Promise<string> => {
  if (!existsSync(DFX_DEFAULT_IDENTITY_STORE_PATH)) {
    throw new Error('Identity store not found.');
  }

  const possiblePaths = [
    join(DFX_DEFAULT_IDENTITY_STORE_PATH, identity, `${identity}.pem`),
    join(DFX_DEFAULT_IDENTITY_STORE_PATH, identity, `identity.pem`),
    join(DFX_DEFAULT_IDENTITY_STORE_PATH, identity, `id.pem`),
  ];

  for (const path of possiblePaths) {
    if (existsSync(path)) {
      return path;
    }
  }

  throw new Error(`PEM file not found for identity: ${identity}`);
};
