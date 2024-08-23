import { describe, expect, it, vi } from 'vitest';
import { mockPinia, mount } from '~/test.utils';
import RegistryUpdateMode from './RegistryUpdateMode.vue';
import { useStationStore } from '~/stores/station.store';

describe('RegistryUpdateMode', () => {
  it('on mount triggers the check for new updates', () => {
    const pinia = mockPinia({ activate: true });
    const station = useStationStore();
    vi.spyOn(station, 'checkVersionUpdates').mockImplementation(() => Promise.resolve());

    const wrapper = mount(
      RegistryUpdateMode,
      { props: { modelValue: {} } },
      { plugins: { pinia } },
    );

    expect(wrapper.exists()).toBe(true);
    expect(station.checkVersionUpdates).toHaveBeenCalled();
  });

  it('shows already in latest version screen', () => {
    const pinia = mockPinia({
      activate: true,
      initialState: {
        station: {
          versionManagement: {
            loading: false,
            nextStationVersion: undefined,
            nextUpgraderVersion: undefined,
          },
        },
      },
    });

    const station = useStationStore();
    vi.spyOn(station, 'checkVersionUpdates').mockImplementation(() => Promise.resolve());

    const wrapper = mount(
      RegistryUpdateMode,
      { props: { modelValue: {} } },
      { plugins: { pinia } },
    );

    expect(wrapper.find('[data-test-id="latest-screen"]').exists()).toBe(true);
  });

  it('shows update available screen', () => {
    const pinia = mockPinia({
      activate: true,
      initialState: {
        station: {
          versionManagement: {
            loading: false,
            nextStationVersion: '1.2.3',
            nextUpgraderVersion: '1.2.3',
          },
        },
      },
    });

    const station = useStationStore();
    vi.spyOn(station, 'checkVersionUpdates').mockImplementation(() => Promise.resolve());

    const wrapper = mount(
      RegistryUpdateMode,
      { props: { modelValue: {} } },
      { plugins: { pinia } },
    );

    expect(wrapper.find('[data-test-id="update-available-screen"]').exists()).toBe(true);
  });

  it('shows loading screen while version updates is in progress', () => {
    const pinia = mockPinia({
      activate: true,
      initialState: {
        station: {
          versionManagement: {
            loading: true,
          },
        },
      },
    });

    const station = useStationStore();
    vi.spyOn(station, 'checkVersionUpdates').mockImplementation(() => Promise.resolve());

    const wrapper = mount(
      RegistryUpdateMode,
      { props: { modelValue: {} } },
      { plugins: { pinia } },
    );

    expect(station.checkVersionUpdates).not.toHaveBeenCalled();
    expect(wrapper.find('[data-test-id="loading-screen"]').exists()).toBe(true);
  });
});
