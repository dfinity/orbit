import { Identity } from '@dfinity/agent';
import { Ed25519KeyIdentity } from '@dfinity/identity';
import { createPrivateKey } from 'crypto';
import { existsSync, readFileSync } from 'fs';
import { homedir } from 'os';
import { join } from 'path';
import { loadIcpIdentity } from './identity-icp';

const DFX_IDENTITY_STORE = join(homedir(), '.config/dfx/identity');

const candidatePemPaths = (identity: string): string[] => [
  join(DFX_IDENTITY_STORE, identity, `${identity}.pem`),
  join(DFX_IDENTITY_STORE, identity, 'identity.pem'),
  join(DFX_IDENTITY_STORE, identity, 'id.pem'),
];

export type IdentitySource = 'dfx' | 'icp';

/**
 * Loads a signing identity from either dfx's PEM store or the icp-cli identity
 * store, depending on `source`. See `identity-icp.ts` for the icp variant.
 */
export const loadIdentity = (source: IdentitySource, name: string): Identity => {
  if (source === 'icp') return loadIcpIdentity(name);
  return loadDfxIdentity(name);
};

/**
 * Loads a dfx-managed identity from its plaintext PEM. Encrypted PEMs are
 * refused; the audit only needs read access, so create a plaintext identity
 * for it:
 *
 *   dfx identity new orbit-audit --storage-mode plaintext
 */
export const loadDfxIdentity = (name: string): Identity => {
  if (!existsSync(DFX_IDENTITY_STORE)) {
    throw new Error(
      `dfx identity store not found at ${DFX_IDENTITY_STORE}. Install dfx or create the identity directory first.`,
    );
  }

  const pemPath = candidatePemPaths(name).find(existsSync);
  if (!pemPath) {
    throw new Error(`No PEM file found for dfx identity '${name}'.`);
  }

  const pem = readFileSync(pemPath, 'utf8');
  if (pem.includes('ENCRYPTED PRIVATE KEY')) {
    throw new Error(
      `Identity '${name}' is passphrase-protected. The audit only needs read access; create a plaintext identity for it:\n` +
        `  dfx identity new ${name}-audit --storage-mode plaintext\n` +
        `then re-run with --identity ${name}-audit.`,
    );
  }
  return parseEd25519Pem(pem, name);
};

/**
 * Parses an unencrypted Ed25519 PKCS#8 PEM into an `Ed25519KeyIdentity`.
 * Shared between the dfx and icp identity loaders.
 */
export const parseEd25519Pem = (pem: string, name: string): Identity => {
  let jwk;
  try {
    const key = createPrivateKey({ key: pem, format: 'pem' });
    jwk = key.export({ format: 'jwk' });
  } catch (err) {
    throw new Error(`Failed to parse PEM for identity '${name}': ${(err as Error).message}`);
  }

  if (jwk.kty !== 'OKP' || jwk.crv !== 'Ed25519') {
    throw new Error(
      `Identity '${name}' is not Ed25519 (kty=${jwk.kty}, crv=${jwk.crv}). The audit supports Ed25519 identities only.`,
    );
  }
  if (typeof jwk.d !== 'string' || typeof jwk.x !== 'string') {
    throw new Error(`Identity '${name}' PEM is missing key material.`);
  }

  // agent-js Ed25519KeyIdentity.fromSecretKey takes the 32-byte seed only;
  // the public key is derived internally.
  const secret = b64urlDecode(jwk.d);
  return Ed25519KeyIdentity.fromSecretKey(secret.buffer);
};

export const b64urlDecode = (input: string): Uint8Array => {
  const pad = '='.repeat((4 - (input.length % 4)) % 4);
  const base64 = (input + pad).replace(/-/g, '+').replace(/_/g, '/');
  return new Uint8Array(Buffer.from(base64, 'base64'));
};
