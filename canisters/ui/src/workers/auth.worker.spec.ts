import { describe, expect, it, vi } from 'vitest';
import { AuthWorkerImpl, DelegationChecker, POLL_INTERVAL_MS } from './auth.worker';

describe('Auth Worker', async () => {
  it('should not run by default', () => {
    vi.useFakeTimers();

    const mockDelegationChecker = {
      getDelegationState: async () => 'valid',
    } satisfies DelegationChecker;

    const spy = vi.spyOn(mockDelegationChecker, 'getDelegationState');

    new AuthWorkerImpl(mockDelegationChecker);

    vi.advanceTimersByTime(POLL_INTERVAL_MS * 2);
    expect(spy).not.toHaveBeenCalled();
  });

  it('should start running after start', () => {
    vi.useFakeTimers();

    const mockDelegationChecker = {
      getDelegationState: async () => 'valid',
    } satisfies DelegationChecker;

    const spy = vi.spyOn(mockDelegationChecker, 'getDelegationState');

    const worker = new AuthWorkerImpl(mockDelegationChecker);

    worker.start();

    vi.advanceTimersByTime(POLL_INTERVAL_MS * 2);

    expect(spy).toHaveBeenCalled();

    worker.stop();
  });

  it('should stop running after stop', () => {
    vi.useFakeTimers();

    const mockDelegationChecker = {
      getDelegationState: async () => 'valid',
    } satisfies DelegationChecker;

    const spy = vi.spyOn(mockDelegationChecker, 'getDelegationState');

    const worker = new AuthWorkerImpl(mockDelegationChecker);

    worker.start();

    vi.advanceTimersByTime(POLL_INTERVAL_MS * 1.5);

    expect(spy).toHaveBeenCalled();
    spy.mockClear();

    worker.stop();

    vi.advanceTimersByTime(POLL_INTERVAL_MS * 1.5);

    expect(spy).not.toHaveBeenCalled();
  });
});
