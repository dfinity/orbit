import { createPinia, setActivePinia } from 'pinia';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { useAppStore } from '~/stores/app.store';

const sessionInitialize = vi.fn<[], Promise<void>>();

vi.mock('~/stores/session.store', () => ({
  useSessionStore: () => ({
    initialize: sessionInitialize,
  }),
}));

describe('AppStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    sessionInitialize.mockReset();
  });

  it('initializes the session only once when initialize is called concurrently', async () => {
    let resolveSession: () => void = () => {};
    sessionInitialize.mockImplementation(
      () =>
        new Promise<void>(resolve => {
          resolveSession = resolve;
        }),
    );

    const app = useAppStore();

    const first = app.initialize();
    const second = app.initialize();

    expect(sessionInitialize).toHaveBeenCalledTimes(1);
    expect(app.initialized).toBe(false);

    resolveSession();
    await Promise.all([first, second]);

    expect(app.initialized).toBe(true);
    expect(sessionInitialize).toHaveBeenCalledTimes(1);
  });

  it('does not initialize the session again once initialized', async () => {
    sessionInitialize.mockResolvedValue();

    const app = useAppStore();
    await app.initialize();
    await app.initialize();

    expect(sessionInitialize).toHaveBeenCalledTimes(1);
    expect(app.initialized).toBe(true);
  });

  it('allows retrying the initialization if the session initialization throws', async () => {
    sessionInitialize.mockRejectedValueOnce(new Error('boom')).mockResolvedValueOnce();

    const app = useAppStore();

    await expect(app.initialize()).rejects.toThrow('boom');
    expect(app.initialized).toBe(false);

    await app.initialize();

    expect(app.initialized).toBe(true);
    expect(sessionInitialize).toHaveBeenCalledTimes(2);
  });
});
