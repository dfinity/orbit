import { beforeEach } from 'vitest';
import { webcrypto } from 'node:crypto';
import resizeObserver from 'resize-observer-polyfill';

beforeEach(() => {
  globalThis.location = new URL('http://orbit.icp') as unknown as Location;
  global.ResizeObserver = resizeObserver;
  Object.defineProperty(globalThis, 'crypto', {
    configurable: true,
    enumerable: true,
    get: () => webcrypto,
  });
});
