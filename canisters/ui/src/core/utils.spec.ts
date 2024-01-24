import { describe, expect, it, vi } from 'vitest';
import { throttle } from '.';

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
