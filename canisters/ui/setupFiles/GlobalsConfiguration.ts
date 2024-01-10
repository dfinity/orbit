import { beforeEach } from 'vitest';
import { webcrypto } from 'node:crypto';

beforeEach(() => {
  globalThis.location = new URL('http://orbit.icp') as unknown as Location;
  Object.defineProperty(globalThis, 'crypto', {
    configurable: true,
    enumerable: true,
    get: () => webcrypto,
  });
});
