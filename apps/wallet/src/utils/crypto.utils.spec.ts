import { describe } from 'node:test';
import { expect, it } from 'vitest';
import { arrayBufferToHashHex, arrayBufferToHex, hexStringToArrayBuffer } from './crypto.utils';

describe('arrayBufferToHex', () => {
  it('converts an ArrayBuffer to a hex string', () => {
    const buffer = new Uint8Array([0, 1, 10, 16, 255]).buffer;
    const hexString = arrayBufferToHex(buffer);
    expect(hexString).toBe('00010a10ff');
  });
});

describe('arrayBufferToHashHex', () => {
  it('hashes an ArrayBuffer and returns a hex string', async () => {
    const buffer = new TextEncoder().encode('test').buffer;
    const hexString = await arrayBufferToHashHex(buffer, 'SHA-256');

    // This assertion checks if the hexString looks like a hex value.
    expect(hexString).toMatch(/^[a-f0-9]{64}$/);
    expect(hexString).toMatchSnapshot();
  });
});

describe('hexStringToArrayBuffer', () => {
  it('converts a hex string to an ArrayBuffer', () => {
    const hexString = '00010a10ff';
    const arrayBuffer = hexStringToArrayBuffer(hexString);
    const resultBytes = new Uint8Array(arrayBuffer);

    expect(resultBytes).toEqual(new Uint8Array([0, 1, 10, 16, 255]));
  });

  it('throws an error for hex strings with odd length', () => {
    const hexString = '123';

    expect(() => {
      hexStringToArrayBuffer(hexString);
    }).toThrow('Hex string must have an even length');
  });
});
