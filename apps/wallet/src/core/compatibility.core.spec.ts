import { Principal } from '@dfinity/principal';
import { describe, expect, it, vi } from 'vitest';
import { createCompatibilityLayer } from './compatibility.core';

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
    const compatibility = createCompatibilityLayer();

    vi.spyOn(compatibility, 'fetchStationApiVersion').mockResolvedValue('1.0.0');
    vi.spyOn(compatibility, 'fetchCompatFile').mockResolvedValue({
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

    await compatibility.checkCompatibility(Principal.anonymous(), {
      redirectIfIncompatible: false,
    });

    expect(compatibility.fetchStationApiVersion).toHaveBeenCalled();
    expect(compatibility.fetchCompatFile).toHaveBeenCalled();
  });

  it('returns false if compatible UI is not found', async () => {
    const compatibility = createCompatibilityLayer();

    vi.spyOn(compatibility, 'fetchStationApiVersion').mockResolvedValue('1.0.0');
    vi.spyOn(compatibility, 'fetchCompatFile').mockImplementation(
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

    const result = await compatibility.checkCompatibility(Principal.anonymous(), {
      redirectIfIncompatible: false,
    });

    expect(result).toBe(false);
  });

  it('returns undefined if already compatible', async () => {
    const compatibility = createCompatibilityLayer();

    vi.spyOn(compatibility, 'fetchStationApiVersion').mockResolvedValue('1.0.0');
    vi.spyOn(compatibility, 'fetchCompatFile').mockResolvedValue({
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

    const result = await compatibility.checkCompatibility(Principal.anonymous(), {
      redirectIfIncompatible: false,
    });

    expect(result).toBeUndefined();
  });

  it('redirects to versioned path if incompatible', async () => {
    const compatibility = createCompatibilityLayer();

    vi.spyOn(compatibility, 'fetchStationApiVersion').mockResolvedValue('1.0.0');
    vi.spyOn(compatibility, 'fetchCompatFile').mockImplementation(
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

    const result = (await compatibility.checkCompatibility(Principal.anonymous(), {
      redirectIfIncompatible: false,
    })) as URL;

    expect(result).toBeInstanceOf(URL);
    expect(result.pathname).toBe('/v2.0.0');
  });

  it('redirects to unversioned path if supports the station version', async () => {
    const versionedURL = new URL(window.location.href);
    versionedURL.pathname = '/v2.0.0';
    window.location = versionedURL as unknown as Location;

    const compatibility = createCompatibilityLayer();

    vi.spyOn(compatibility, 'fetchStationApiVersion').mockResolvedValue('1.0.0');
    vi.spyOn(compatibility, 'fetchCompatFile').mockImplementation(
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

    const result = (await compatibility.checkCompatibility(Principal.anonymous(), {
      redirectIfIncompatible: true,
    })) as URL;

    expect(result).toBeInstanceOf(URL);
    expect(result.pathname).toBe('/');
  });
});
