import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import AppToolbar from './AppToolbar.vue';

describe('AppToolbar', () => {
  it('shows update when new version is available', () => {
    const wrapper = mount(
      AppToolbar,
      {},
      {
        initialPiniaState: {
          session: { isAuthenticated: true },
          station: {
            privileges: [{ SystemUpgrade: null }],
            versionManagement: {
              nextStationVersion: '1.2.3',
              nextUpgraderVersion: '1.2.3',
              updateRequested: undefined,
            },
          },
        },
      },
    );

    expect(wrapper.find('[data-test-id="submit-upgrade-btn"]').exists()).toBe(true);
  });

  it('hides update if a request is already in progress', () => {
    const wrapper = mount(
      AppToolbar,
      {},
      {
        initialPiniaState: {
          session: { isAuthenticated: true },
          station: {
            privileges: [{ SystemUpgrade: null }],
            versionManagement: {
              nextStationVersion: '1.2.3',
              nextUpgraderVersion: '1.2.3',
              updateRequested: '01d71432-d654-4008-8bbe-1d16ccb949d6',
            },
          },
        },
      },
    );

    expect(wrapper.find('[data-test-id="submit-upgrade-btn"]').exists()).toBe(false);
  });

  it('hides update if no new version is available', () => {
    const wrapper = mount(
      AppToolbar,
      {},
      {
        initialPiniaState: {
          session: { isAuthenticated: true },
          station: {
            privileges: [{ SystemUpgrade: null }],
            versionManagement: {
              nextStationVersion: undefined,
              nextUpgraderVersion: undefined,
              updateRequested: undefined,
            },
          },
        },
      },
    );

    expect(wrapper.find('[data-test-id="submit-upgrade-btn"]').exists()).toBe(false);
  });

  it('hides update if user has no privileges', () => {
    const wrapper = mount(
      AppToolbar,
      {},
      {
        initialPiniaState: {
          session: { isAuthenticated: true },
          station: {
            privileges: [],
            versionManagement: {
              nextStationVersion: '1.2.3',
              nextUpgraderVersion: '1.2.3',
              updateRequested: undefined,
            },
          },
        },
      },
    );

    expect(wrapper.find('[data-test-id="submit-upgrade-btn"]').exists()).toBe(false);
  });

  it('hides update if user is not authenticated', () => {
    const wrapper = mount(
      AppToolbar,
      {},
      {
        initialPiniaState: {
          session: { isAuthenticated: false },
          station: {
            privileges: [{ SystemUpgrade: null }],
            versionManagement: {
              nextStationVersion: '1.2.3',
              nextUpgraderVersion: '1.2.3',
              updateRequested: undefined,
            },
          },
        },
      },
    );

    expect(wrapper.find('[data-test-id="submit-upgrade-btn"]').exists()).toBe(false);
  });
});
