import { describe, expect, it, vi } from 'vitest';
import { throttle, transformIdlWithOnlyVerifiedCalls, variantIs } from './helper.utils';
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
