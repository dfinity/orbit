import { describe, expect, it, vi } from 'vitest';
import { useUserActivity } from './user-activity';
import { setupComponent } from '../test.utils';

describe('UserActivity', () => {
  it('should be able to create a new user activity monitor', () => {
    setupComponent(() => ({
      useUserActivity: useUserActivity({
        onActive: vi.fn(),
        throttleMs: 1000,
      }),
    }));
  });

  it('does not call onActive without user activity', () => {
    const onActive = vi.fn();
    setupComponent(() => ({
      useUserActivity: useUserActivity({
        onActive,
        throttleMs: 1000,
      }),
    }));

    expect(onActive).not.toHaveBeenCalled();
  });

  it('throttles multiple onActive calls', () => {
    const onActive = vi.fn();
    setupComponent(() => ({
      useUserActivity: useUserActivity({
        onActive,
        throttleMs: 1000,
      }),
    }));

    window.dispatchEvent(new Event('mousemove'));
    window.dispatchEvent(new Event('mousemove'));
    window.dispatchEvent(new Event('mousemove'));
    expect(onActive).toHaveBeenCalledTimes(1);
  });

  it('calls onActive again after throttleMs ms passes', () => {
    vi.useFakeTimers();
    const onActive = vi.fn();
    setupComponent(() => ({
      useUserActivity: useUserActivity({
        onActive,
        throttleMs: 1000,
      }),
    }));

    window.dispatchEvent(new Event('mousemove'));
    window.dispatchEvent(new Event('mousemove'));
    expect(onActive).toHaveBeenCalledTimes(1);

    vi.advanceTimersByTime(1000);

    window.dispatchEvent(new Event('mousemove'));
    window.dispatchEvent(new Event('mousemove'));
    expect(onActive).toHaveBeenCalledTimes(2);
  });
});
