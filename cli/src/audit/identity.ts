import { Identity } from '@dfinity/agent';
import { Ed25519KeyIdentity } from '@dfinity/identity';
import { createPrivateKey } from 'crypto';
import { existsSync, readFileSync } from 'fs';
import { homedir } from 'os';
import { join } from 'path';

const DFX_IDENTITY_STORE = join(homedir(), '.config/dfx/identity');

const candidatePemPaths = (identity: string): string[] => [
  join(DFX_IDENTITY_STORE, identity, `${identity}.pem`),
  join(DFX_IDENTITY_STORE, identity, 'identity.pem'),
  join(DFX_IDENTITY_STORE, identity, 'id.pem'),
];

/**
 * Loads a dfx-managed identity from its plaintext PEM and returns an agent-js
 * `Identity` suitable for signing canister requests.
 *
 * Encrypted (passphrase-protected) PEMs are not supported; the audit only
 * needs read access to `list_*` query methods, and decrypting PKCS#8 with
 * scrypt/PBKDF2 + AES would be a significant chunk of code for a read-only
 * tool. Operators with an encrypted identity should create a plaintext one
 * specifically for the audit:
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

  // Node's crypto can parse the PKCS#8 envelope; we then export the raw key
  // material in JWK form and feed the seed into agent-js' Ed25519 identity.
  let jwk;
  try {
    const key = createPrivateKey({ key: pem, format: 'pem' });
    jwk = key.export({ format: 'jwk' });
  } catch (err) {
    throw new Error(`Failed to parse PEM for identity '${name}': ${(err as Error).message}`);
  }

  if (jwk.kty !== 'OKP' || jwk.crv !== 'Ed25519') {
    throw new Error(
      `Identity '${name}' is not Ed25519 (kty=${jwk.kty}, crv=${jwk.crv}). The audit supports Ed25519 dfx identities only.`,
    );
  }

  if (!jwk.d || !jwk.x) {
    throw new Error(`Identity '${name}' PEM is missing key material.`);
  }

  const secretKey = b64urlDecode(jwk.d);
  const publicKey = b64urlDecode(jwk.x);
  const combined = new Uint8Array(secretKey.length + publicKey.length);
  combined.set(secretKey, 0);
  combined.set(publicKey, secretKey.length);
  return Ed25519KeyIdentity.fromSecretKey(combined.buffer);
};

const b64urlDecode = (input: string): Uint8Array => {
  const pad = '='.repeat((4 - (input.length % 4)) % 4);
  const base64 = (input + pad).replace(/-/g, '+').replace(/_/g, '/');
  return new Uint8Array(Buffer.from(base64, 'base64'));
};
