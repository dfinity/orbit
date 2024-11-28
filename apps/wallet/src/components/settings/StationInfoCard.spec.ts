import { Principal } from '@dfinity/principal';
import { describe, expect, it, vi } from 'vitest';
import { services } from '~/plugins/services.plugin';
import { ControlPanelService } from '~/services/control-panel.service';
import { useSessionStore } from '~/stores/session.store';
import { useStationStore } from '~/stores/station.store';
import { mount } from '~/test.utils';
import StationInfoCard from './StationInfoCard.vue';

import { SystemInfoResult } from '~/generated/station/station.did';
import { ExtractOk } from '~/types/helper.types';
import { flushPromises } from '@vue/test-utils';

const testUpgraderId = 'r7inp-6aaaa-aaaaa-aaabq-cai';

services().station.systemInfo = vi.fn(() =>
  Promise.resolve({
    system: {
      upgrader_id: Principal.fromText(testUpgraderId),
    },
  } as ExtractOk<SystemInfoResult>),
);

vi.mock('~/services/control-panel.service', () => {
  const mock: Partial<ControlPanelService> = {
    manageUserStations: vi.fn().mockReturnThis(),
  };

  return {
    ControlPanelService: vi.fn(() => mock),
  };
});

describe('StationInfoCard', () => {
  function initStation(principal: Principal, name: string) {
    const stationStore = useStationStore();
    const sessionStore = useSessionStore();

    sessionStore.$patch({
      data: {
        stations: [{ canisterId: principal.toText(), name }],
        selected: { canisterId: principal.toText(), hasAccess: true },
      },
    });
    stationStore.$patch({ canisterId: principal.toText() });
  }

  function addStation(principal: Principal, isMain: boolean, name: string) {
    const sessionStore = useSessionStore();
    const stationStore = useStationStore();

    sessionStore.$patch({
      data: {
        stations: isMain
          ? [{ canisterId: principal.toText(), name }, ...sessionStore.data.stations]
          : [...sessionStore.data.stations, { canisterId: principal.toText(), name }],
      },
    });
    stationStore.$patch({ canisterId: principal.toText() });
  }

  function selectStation(principal: Principal, hasAccess: boolean = true) {
    const sessionStore = useSessionStore();
    sessionStore.$patch({
      data: { selected: { canisterId: principal.toText(), hasAccess } },
    });
  }

  const stationCanisterId1 = Principal.fromUint8Array(new Uint8Array([1, 2, 3]));
  const stationCanisterId2 = Principal.fromUint8Array(new Uint8Array([1, 2, 4]));

  it('renders properly', () => {
    const wrapper = mount(StationInfoCard);

    expect(wrapper.exists()).toBe(true);
  });

  it('renders the user selected station name', async () => {
    const wrapper = mount(StationInfoCard);

    const session = useSessionStore();
    session.data.stations = [
      {
        canisterId: Principal.anonymous().toText(),
        name: 'Personal',
        labels: [],
      },
    ];

    await wrapper.vm.$nextTick();

    const nameLine = wrapper.find('[data-test-id="user-selected-station-name"]');

    expect(nameLine.exists()).toBeTruthy();
    expect(nameLine.text()).toContain('Personal');
  });

  it('renders the station default name', async () => {
    const wrapper = mount(StationInfoCard);
    const station = useStationStore();
    const session = useSessionStore();

    station.canisterId = Principal.anonymous().toText();
    station.configuration.details = {
      ...station.configuration.details,
      name: 'Station',
    };

    session.data.stations = [
      {
        canisterId: Principal.anonymous().toText(),
        name: 'Personal',
        labels: [],
      },
    ];

    const nameLine = wrapper.find('[data-test-id="station-name"]');

    await wrapper.vm.$nextTick();

    expect(nameLine.exists()).toBeTruthy();
    expect(nameLine.text()).toContain('Station');
  });

  it('shows a remove station button', () => {
    const wrapper = mount(StationInfoCard);
    expect(wrapper.find('[data-test-id="remove-station-btn"]').exists()).toBe(true);
  });

  it('is not disabled if the station is the main station', async () => {
    const wrapper = mount(StationInfoCard);
    initStation(stationCanisterId1, 'TEST WALLET');
    await wrapper.vm.$nextTick();
    const button = wrapper.find('[data-test-id="remove-station-btn"]');
    expect(button.attributes('disabled')).toBeUndefined();
  });

  it('is not disabled if the station is the only station', async () => {
    const wrapper = mount(StationInfoCard);

    initStation(stationCanisterId1, 'TEST WALLET');
    addStation(stationCanisterId2, false, 'TEST WALLET 2');
    selectStation(stationCanisterId2);
    await wrapper.vm.$nextTick();

    const button = wrapper.find('[data-test-id="remove-station-btn"]');

    expect(button.attributes('disabled')).toBeUndefined();
  });

  it('calls manageUserStations with the station to remove when the dialog is confirmed', async () => {
    const wrapper = mount(StationInfoCard);

    initStation(stationCanisterId1, 'TEST WALLET');
    addStation(stationCanisterId2, false, 'TEST WALLET 2');
    selectStation(stationCanisterId2);
    await wrapper.vm.$nextTick();
    wrapper.find('[data-test-id="remove-station-btn"]').trigger('click');
    await wrapper.vm.$nextTick();

    document
      .querySelector('[data-test-id="action-btn-default-submit-btn"]')
      ?.dispatchEvent(new Event('click'));

    expect(services().controlPanel.manageUserStations).toHaveBeenCalledWith(
      expect.objectContaining({
        Remove: [stationCanisterId2],
      }),
    );
  });

  it('shows the upgrader id', async () => {
    const wrapper = mount(
      StationInfoCard,
      {},
      {
        initialPiniaState: {
          session: { isAuthenticated: true },
          station: {
            privileges: [{ SystemInfo: null }],
          },
        },
      },
    );

    await flushPromises();

    expect(wrapper.text().includes(testUpgraderId)).toBe(true);
  });
});
