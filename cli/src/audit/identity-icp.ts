import { AnonymousIdentity, Identity } from '@dfinity/agent';
import { DelegationChain, DelegationIdentity, Ed25519KeyIdentity } from '@dfinity/identity';
import { spawnSync } from 'child_process';
import { generateKeyPairSync } from 'crypto';
import { existsSync, readFileSync, unlinkSync, writeFileSync } from 'fs';
import { homedir, tmpdir } from 'os';
import { join } from 'path';
import { b64urlDecode, parseEd25519Pem } from './identity';

const ICP_IDENTITY_STORE_PATHS = [
  // macOS — verified location used by org.dfinity.icp-cli.
  join(homedir(), 'Library/Application Support/org.dfinity.icp-cli/identity'),
  // Linux / XDG fallbacks for portability.
  join(homedir(), '.local/share/org.dfinity.icp-cli/identity'),
  join(homedir(), '.config/org.dfinity.icp-cli/identity'),
];

interface IcpIdentityEntry {
  kind: 'keyring' | 'internet-identity' | 'anonymous' | 'hsm';
  principal?: string;
  algorithm?: 'ed25519' | 'secp256k1';
}

interface IcpIdentityList {
  v: number;
  identities: Record<string, IcpIdentityEntry>;
}

const findIcpStore = (): string => {
  const found = ICP_IDENTITY_STORE_PATHS.find(existsSync);
  if (!found) {
    throw new Error(
      `icp-cli identity store not found. Tried:\n` +
        ICP_IDENTITY_STORE_PATHS.map(p => `  ${p}`).join('\n') +
        `\nInstall icp-cli first.`,
    );
  }
  return found;
};

const readIdentityList = (): IcpIdentityList => {
  const path = join(findIcpStore(), 'identity_list.json');
  if (!existsSync(path)) {
    throw new Error(`icp-cli identity_list.json missing at ${path}`);
  }
  return JSON.parse(readFileSync(path, 'utf8')) as IcpIdentityList;
};

const runIcp = (args: string[]): { stdout: string; stderr: string; status: number | null } => {
  const result = spawnSync('icp', args, { encoding: 'utf8' });
  if (result.error) {
    throw new Error(
      `Failed to invoke icp CLI: ${result.error.message}. Install icp-cli or ensure it is on PATH.`,
    );
  }
  return { stdout: result.stdout, stderr: result.stderr, status: result.status };
};

/**
 * Generates a short-lived Ed25519 session keypair and returns both the SPKI-form
 * public-key PEM (consumed by `icp identity delegation sign --key-pem`) and an
 * agent-js `Ed25519KeyIdentity` configured with the matching secret key.
 */
const generateSessionKey = (): {
  pubKeyPem: string;
  identity: Ed25519KeyIdentity;
} => {
  const { publicKey, privateKey } = generateKeyPairSync('ed25519');
  const pubKeyPem = publicKey.export({ format: 'pem', type: 'spki' }) as string;
  const jwk = privateKey.export({ format: 'jwk' });
  if (typeof jwk.d !== 'string' || typeof jwk.x !== 'string') {
    throw new Error('Failed to extract Ed25519 session-key material from Node crypto.');
  }
  const secret = b64urlDecode(jwk.d);
  const pub = b64urlDecode(jwk.x);
  const combined = new Uint8Array(secret.length + pub.length);
  combined.set(secret, 0);
  combined.set(pub, secret.length);
  return { pubKeyPem, identity: Ed25519KeyIdentity.fromSecretKey(combined.buffer) };
};

/**
 * Loads an identity from the `icp-cli` identity store and returns an agent-js
 * `Identity` suitable for signing canister calls.
 *
 * - `kind: anonymous` → `AnonymousIdentity`.
 * - `kind: keyring` (Ed25519) → exports the PEM via `icp identity export` and
 *   parses it like a dfx-managed PEM. Secp256k1 keyring identities require an
 *   additional dependency and are rejected with a clear error today.
 * - `kind: internet-identity` → generates an ephemeral Ed25519 session key,
 *   asks icp to sign a delegation chain for it (`icp identity delegation sign`),
 *   and assembles a `DelegationIdentity`. The session key never leaves this
 *   process.
 * - `kind: hsm` → unsupported (PKCS#11 access is out of scope).
 */
export const loadIcpIdentity = (name: string): Identity => {
  const list = readIdentityList();
  const entry = list.identities[name];
  if (!entry) {
    const available = Object.keys(list.identities).join(', ') || '(none)';
    throw new Error(`icp identity '${name}' not found. Available: ${available}`);
  }

  if (entry.kind === 'anonymous') {
    return new AnonymousIdentity();
  }

  if (entry.kind === 'hsm') {
    throw new Error(
      `icp identity '${name}' is HSM-backed. PKCS#11 isn't supported by orbit-cli audit yet.`,
    );
  }

  if (entry.kind === 'keyring') {
    if (entry.algorithm && entry.algorithm !== 'ed25519') {
      throw new Error(
        `icp identity '${name}' uses ${entry.algorithm}. orbit-cli audit currently supports Ed25519 keyring identities only. Use --identity-source icp with an Ed25519 identity, or use --identity-source dfx with a plaintext Ed25519 identity.`,
      );
    }
    const { stdout, stderr, status } = runIcp(['identity', 'export', name]);
    if (status !== 0) {
      throw new Error(`'icp identity export ${name}' failed: ${stderr.trim()}`);
    }
    return parseEd25519Pem(stdout, name);
  }

  if (entry.kind === 'internet-identity') {
    const { pubKeyPem, identity: session } = generateSessionKey();
    const pubKeyPath = join(
      tmpdir(),
      `orbit-cli-session-${process.pid}-${process.hrtime.bigint()}.pem`,
    );
    writeFileSync(pubKeyPath, pubKeyPem);
    try {
      const { stdout, stderr, status } = runIcp([
        'identity',
        'delegation',
        'sign',
        '--identity',
        name,
        '--key-pem',
        pubKeyPath,
        '--duration',
        '1h',
      ]);
      if (status !== 0) {
        throw new Error(
          `'icp identity delegation sign --identity ${name}' failed: ${stderr.trim()}`,
        );
      }
      const chain = DelegationChain.fromJSON(stdout);
      return DelegationIdentity.fromDelegation(session, chain);
    } finally {
      try {
        unlinkSync(pubKeyPath);
      } catch {
        // Best-effort cleanup; ignore missing file or platform quirks.
      }
    }
  }

  throw new Error(`Unknown icp identity kind: ${(entry as { kind: string }).kind}`);
};
