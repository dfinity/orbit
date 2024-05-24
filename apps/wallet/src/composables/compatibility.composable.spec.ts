import { describe, vi, expect, it } from 'vitest';
import { setupComponent } from '../test.utils';
import { useCompatibilityLayer } from './compatibility.composable';
import { Principal } from '@dfinity/principal';

vi.mock('~/core/ic-agent.core', () => ({
  icAgent: {
    get: () => ({
      fetchRootKey: vi.fn(),
      readState: vi.fn(() => Promise.resolve()),
    }),
  },
}));

describe('Compatibility Composables', () => {
  it('calls fetchStationApiVersion and fetchCompatFile to get the versions', async () => {
    const vm = setupComponent(() => {
      return {
        compatibility: useCompatibilityLayer(),
      };
    });

    vi.spyOn(vm.compatibility, 'fetchStationApiVersion').mockResolvedValue('1.0.0');
    vi.spyOn(vm.compatibility, 'fetchCompatFile').mockResolvedValue({
      version: '1.0.0',
      api: {
        latest: '1.0.0',
        compatibility: {
          '1.0.0': {
            ui: ['1.0.0'],
          },
        },
      },
    });

    await vm.compatibility.checkCompatibility(Principal.anonymous(), {
      redirectIfIncompatible: false,
    });

    expect(vm.compatibility.fetchStationApiVersion).toHaveBeenCalled();
    expect(vm.compatibility.fetchCompatFile).toHaveBeenCalled();
  });

  it('returns false if compatible UI is not found', async () => {
    const vm = setupComponent(() => {
      return {
        compatibility: useCompatibilityLayer(),
      };
    });

    vi.spyOn(vm.compatibility, 'fetchStationApiVersion').mockResolvedValue('1.0.0');
    vi.spyOn(vm.compatibility, 'fetchCompatFile').mockImplementation(
      versionPath =>
        new Promise((resolve, reject) => {
          if (versionPath === 'v2.0.0') {
            return reject(new Error('Not found'));
          }

          return resolve({
            version: '1.0.0',
            api: {
              latest: '2.0.0',
              compatibility: {
                '1.0.0': {
                  ui: ['2.0.0'],
                },
              },
            },
          });
        }),
    );

    const result = await vm.compatibility.checkCompatibility(Principal.anonymous(), {
      redirectIfIncompatible: false,
    });

    expect(result).toBe(false);
  });

  it('returns undefined if already compatible', async () => {
    const vm = setupComponent(() => {
      return {
        compatibility: useCompatibilityLayer(),
      };
    });

    vi.spyOn(vm.compatibility, 'fetchStationApiVersion').mockResolvedValue('1.0.0');
    vi.spyOn(vm.compatibility, 'fetchCompatFile').mockResolvedValue({
      version: '1.0.0',
      api: {
        latest: '1.0.0',
        compatibility: {
          '1.0.0': {
            ui: ['1.0.0'],
          },
        },
      },
    });

    const result = await vm.compatibility.checkCompatibility(Principal.anonymous(), {
      redirectIfIncompatible: false,
    });

    expect(result).toBeUndefined();
  });

  it('redirects to versioned path if incompatible', async () => {
    const vm = setupComponent(() => {
      return {
        compatibility: useCompatibilityLayer(),
      };
    });

    vi.spyOn(vm.compatibility, 'fetchStationApiVersion').mockResolvedValue('1.0.0');
    vi.spyOn(vm.compatibility, 'fetchCompatFile').mockImplementation(
      versionPath =>
        new Promise(resolve => {
          if (versionPath === 'v2.0.0') {
            return resolve({
              version: '2.0.0',
              api: {
                latest: '1.0.0',
                compatibility: {
                  '1.0.0': {
                    ui: ['2.0.0'],
                  },
                },
              },
            });
          }

          return resolve({
            version: '3.0.0',
            api: {
              latest: '2.0.0',
              compatibility: {
                '1.0.0': {
                  ui: ['2.0.0'],
                },
              },
            },
          });
        }),
    );

    const result = (await vm.compatibility.checkCompatibility(Principal.anonymous(), {
      redirectIfIncompatible: false,
    })) as URL;

    expect(result).toBeInstanceOf(URL);
    expect(result.pathname).toBe('/v2.0.0');
  });

  it('redirects to unversioned path if supports the station version', async () => {
    const versionedURL = new URL(window.location.href);
    versionedURL.pathname = '/v2.0.0';
    window.location = versionedURL as unknown as Location;

    const vm = setupComponent(() => {
      return {
        compatibility: useCompatibilityLayer(),
      };
    });

    vi.spyOn(vm.compatibility, 'fetchStationApiVersion').mockResolvedValue('1.0.0');
    vi.spyOn(vm.compatibility, 'fetchCompatFile').mockImplementation(
      versionPath =>
        new Promise(resolve => {
          if (versionPath === 'v2.0.0') {
            return resolve({
              version: '2.0.0',
              api: {
                latest: '0.0.1',
                compatibility: {
                  '0.0.1': {
                    ui: ['2.0.0'],
                  },
                },
              },
            });
          }

          return resolve({
            version: '3.0.0',
            api: {
              latest: '1.0.0',
              compatibility: {
                '1.0.0': {
                  ui: ['3.0.0'],
                },
                '0.0.1': {
                  ui: ['2.0.0'],
                },
              },
            },
          });
        }),
    );

    const result = (await vm.compatibility.checkCompatibility(Principal.anonymous(), {
      redirectIfIncompatible: true,
    })) as URL;

    expect(result).toBeInstanceOf(URL);
    expect(result.pathname).toBe('/');
  });
});
