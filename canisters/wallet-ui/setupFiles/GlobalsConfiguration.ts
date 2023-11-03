import { beforeEach } from 'vitest';
import { webcrypto } from 'node:crypto';

beforeEach(() => {
  globalThis.location = new URL('http://orbit.icp') as unknown as Location;
  globalThis.crypto = webcrypto as unknown as Crypto;
});
