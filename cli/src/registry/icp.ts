import { IDL } from '@dfinity/candid';
import { Principal } from '@dfinity/principal';
import { execFile } from 'child_process';
import { randomBytes } from 'crypto';
import { existsSync, readFileSync, writeFileSync } from 'fs';
import { tmpdir } from 'os';
import { join } from 'path';
import { promisify } from 'util';
import { idlFactory } from '../generated/control_panel.did.js';
import { getReplicaUrl, ROOT_PATH } from '../utils';

const execFileAsync = promisify(execFile);

const LOCAL_URL_PREFIXES = ['http://localhost', 'http://127.0.0.1', 'http://[::1]'];

const controlPanel = idlFactory({ IDL }) as IDL.ServiceClass;

// icp-cli hands back the raw reply bytes rather than a decoded value, so the
// reply is decoded here against the control-panel interface.
const replyTypes = (method: string): IDL.Type[] => {
  const field = controlPanel._fields.find(([name]) => name === method);
  if (!field) {
    throw new Error(`'${method}' is not a method of the control-panel interface.`);
  }

  return field[1].retTypes;
};

export const getCanisterId = (name: string, network: string): Principal => {
  const candidates = [
    join(ROOT_PATH, '.dfx', network, 'canister_ids.json'),
    join(ROOT_PATH, 'canister_ids.json'),
  ];

  for (const path of candidates) {
    if (!existsSync(path)) {
      continue;
    }

    const id = JSON.parse(readFileSync(path, 'utf-8'))?.[name]?.[network];
    if (typeof id === 'string') {
      return Principal.fromText(id);
    }
  }

  throw new Error(`No canister id recorded for '${name}' on network '${network}'.`);
};

// icp-cli identifies a network by URL plus the root key to verify replies with,
// so the URL comes from dfx.json and the key follows from whether it is local.
const networkArgs = async (network: string): Promise<string[]> => {
  const url = await getReplicaUrl(network);
  const isLocal = LOCAL_URL_PREFIXES.some(prefix => url.startsWith(prefix));

  return ['--network', url, '--root-key', isLocal ? 'fetch' : 'mainnet'];
};

export const callControlPanel = async <T>(opts: {
  method: string;
  arg: string;
  network: string;
  identity: string;
  query?: boolean;
}): Promise<T> => {
  // Passing argv directly rather than a shell string: registry entries carry
  // free-form descriptions and tags, and a quote in one of those would
  // otherwise break the Candid argument or inject into the command.
  const args = [
    'canister',
    'call',
    ...(await networkArgs(opts.network)),
    '--identity',
    opts.identity,
    '--json',
    ...(opts.query ? ['--query'] : []),
    getCanisterId('control_panel', opts.network).toText(),
    opts.method,
    opts.arg,
  ];

  const { stdout } = await execFileAsync('icp', args, { maxBuffer: 64 * 1024 * 1024 });
  const { response_bytes: hex } = JSON.parse(stdout) as { response_bytes: string };
  // Copied into its own Uint8Array: a Buffer is a view into a shared pool, and
  // the decoder reads through to the underlying ArrayBuffer.
  const [decoded] = IDL.decode(replyTypes(opts.method), Uint8Array.from(Buffer.from(hex, 'hex')));

  return decoded as T;
};

// icx-asset signs with a pem file, so the key has to come back out of whatever
// store icp-cli keeps it in. Exporting keeps this independent of the store
// layout, which differs per platform and per storage mode.
export const exportIdentityPem = async (identity: string): Promise<string> => {
  const { stdout } = await execFileAsync('icp', ['identity', 'export', identity]);
  const path = join(tmpdir(), `orbit-cli-${identity}-${randomBytes(8).toString('hex')}.pem`);
  writeFileSync(path, stdout, { encoding: 'utf-8', mode: 0o600 });

  return path;
};
