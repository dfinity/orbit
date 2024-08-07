import { describe, expect, it, vi } from 'vitest';
import {
  isSemanticVersion,
  removeBasePathFromPathname,
  throttle,
  toArrayBuffer,
  transformIdlWithOnlyVerifiedCalls,
  variantIs,
} from './helper.utils';
import { idlFactory } from '~/generated/control-panel';
import { IDL } from '@dfinity/candid';

describe('Core utils', () => {
  describe('throttle', () => {
    it('calls a function once immediately', () => {
      const fn = vi.fn();
      const throttled = throttle(fn, 1000);

      throttled();

      expect(fn).toHaveBeenCalledTimes(1);
    });

    it('does not call a function more than once immediately', () => {
      const fn = vi.fn();
      const throttled = throttle(fn, 1000);

      throttled();
      throttled();
      throttled();

      expect(fn).toHaveBeenCalledTimes(1);
    });

    it('calls a function a second time only after a delay', async () => {
      vi.useFakeTimers();
      const fn = vi.fn();
      const throttled = throttle(fn, 1000);

      throttled();
      throttled();
      throttled();
      throttled();

      expect(fn).toHaveBeenCalledTimes(1);

      vi.advanceTimersByTime(1000);

      expect(fn).toHaveBeenCalledTimes(2);
    });
  });
});

describe('Candid utils', () => {
  describe('variantIs', () => {
    it('returns true if the key is a variant', () => {
      const obj: { key?: string } = { key: 'value' };
      expect(variantIs(obj, 'key')).toBe(true);
    });

    it('returns false if the key is not the selected variant', () => {
      const obj: { key?: string; anotherKey?: string } = { key: 'value' };
      expect(variantIs(obj, 'anotherKey')).toBe(false);
    });
  });

  describe('transformIdlWithOnlyVerifiedCalls', () => {
    it('transforms the IDL to only include verified calls', () => {
      const original_service = idlFactory({ IDL });
      const service = transformIdlWithOnlyVerifiedCalls(idlFactory)({ IDL });

      expect(
        original_service._fields.some(field => field?.[1].annotations.includes('query')),
      ).toBeTruthy();

      expect(service._fields.some(field => field?.[1].annotations.includes('query'))).toBeFalsy();
    });
  });
});

describe('Semver utils', () => {
  describe('isSemanticVersion', () => {
    it.each([
      '1.0.0',
      '1.0.0-beta',
      '1.0.0-beta+build',
      '1.0.0-beta.1',
      '1.0.0-beta.1+build',
      '1.0.0+build',
    ])('returns true for valid semantic version `%s`', version => {
      expect(isSemanticVersion(version)).toBe(true);
    });

    it('returns true for valid semantic version with prefix `v1.0.0`', () => {
      expect(isSemanticVersion('v1.0.0', 'v')).toBe(true);
    });

    it('returns true for valid semantic version without prefix `1.0.0-beta`', () => {
      expect(isSemanticVersion('1.0.0-beta', '')).toBe(true);
    });

    it.each([
      '',
      'invalid',
      '1.0',
      '1.0-beta',
      '1.0-beta+build',
      '1.0-beta.1',
      '1.0-beta.1+build',
      '1.0+build',
    ])('returns false for invalid semantic version `%s`', version => {
      expect(isSemanticVersion(version)).toBe(false);
    });

    it('returns false for invalid semantic version with prefix `v1.0`', () => {
      expect(isSemanticVersion('v1.0', 'v')).toBe(false);
    });
  });
});

describe('Url utils', () => {
  describe('removeBasePathFromPathname', () => {
    it('removes the base path from the pathname', () => {
      const pathname = '/base/pathname';
      const basePath = '/base';

      expect(removeBasePathFromPathname(pathname, basePath)).toBe('/pathname');
    });

    it('does not remove the base path if it is not at the start of the pathname', () => {
      const pathname = '/pathname/base';
      const basePath = '/base';

      expect(removeBasePathFromPathname(pathname, basePath)).toBe(pathname);
    });

    it('adds a leading slash if the updated path does not have one', () => {
      const pathname = 'pathname';
      const basePath = '';

      expect(removeBasePathFromPathname(pathname, basePath)).toBe('/pathname');
    });
  });
});

describe('ArrayBuffer utils', () => {
  describe('toArrayBuffer', () => {
    it('converts a Uint8Array to an ArrayBuffer', () => {
      const input = new Uint8Array([1, 2, 3, 4, 5]);
      const output = toArrayBuffer(input);

      expect(output).toBeInstanceOf(ArrayBuffer);
      expect(new Uint8Array(output)).toEqual(input);
    });

    it('converts a number[] to an ArrayBuffer', () => {
      const input = [1, 2, 3, 4, 5];
      const output = toArrayBuffer(input);

      expect(output).toBeInstanceOf(ArrayBuffer);
      expect(new Uint8Array(output)).toEqual(new Uint8Array(input));
    });
  });
});
