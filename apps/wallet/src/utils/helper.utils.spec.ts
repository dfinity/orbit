import { describe, expect, it, vi } from 'vitest';
import {
  compactArray,
  isSemanticVersion,
  parseLocationQuery,
  parseToBigIntOrUndefined,
  removeBasePathFromPathname,
  throttle,
  toArrayBuffer,
  transformIdlWithOnlyVerifiedCalls,
  variantIs,
} from './helper.utils';
import { idlFactory } from '~/generated/control-panel';
import { IDL } from '@dfinity/candid';
import { LocationQuery } from 'vue-router';

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

describe('Array utils', () => {
  describe('compactArray', () => {
    it('removes all null and undefined values from an array', () => {
      const array = [1, null, 2, undefined, 3, 4, null, 5];
      const result = compactArray(array);

      expect(result).toEqual([1, 2, 3, 4, 5]);
    });

    it('removes all null, undefined, and empty strings from an array', () => {
      const array = [1, null, 2, undefined, 3, 4, null, 5, ''];
      const result = compactArray(array, { removeEmptyStrings: true });

      expect(result).toEqual([1, 2, 3, 4, 5]);
    });

    it('only includes items that are in the set', () => {
      const array = [1, 2, 3, 4, 5];
      const include = new Set([1, 3, 5]);
      const result = compactArray(array, { include });

      expect(result).toEqual([1, 3, 5]);
    });
  });
});

describe('Location query utils', () => {
  describe('parseLocationQuery', () => {
    it('parses a location query object', () => {
      const query: LocationQuery = {};
      query.arg1 = 'value';
      query.arg2 = ['value1', 'value2'];
      query.arg3 = '';
      query.arg4 = null;
      query.arg5;
      query.arg6 = ['value1', '', 'value2', null];

      const result = parseLocationQuery(query);

      expect(result).toEqual({
        arg1: ['value'],
        arg2: ['value1', 'value2'],
        arg6: ['value1', 'value2'],
      });
    });
  });
});

describe('BigInt utils', () => {
  describe('parseToBigIntOrUndefined', () => {
    it('null returns undefined', () => {
      expect(parseToBigIntOrUndefined(null)).toBeUndefined();
    });

    it('undefined returns undefined', () => {
      expect(parseToBigIntOrUndefined(undefined)).toBeUndefined();
    });

    it('string returns BigInt', () => {
      expect(parseToBigIntOrUndefined('100')).toBe(BigInt(100));
    });

    it('number returns BigInt', () => {
      expect(parseToBigIntOrUndefined(100)).toBe(BigInt(100));
    });

    it('bigint returns BigInt', () => {
      expect(parseToBigIntOrUndefined(BigInt(100))).toBe(BigInt(100));
    });

    it('invalid string returns undefined', () => {
      expect(parseToBigIntOrUndefined('invalid')).toBeUndefined();
    });

    it('empty string returns undefined', () => {
      expect(parseToBigIntOrUndefined('')).toBeUndefined();
    });
  });
});
