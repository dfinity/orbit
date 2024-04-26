import { Principal } from '@dfinity/principal';
import { describe, expect, it, vi } from 'vitest';
import { services } from '~/plugins/services.plugin';
import { ControlPanelService } from '~/services/control-panel.service';
import { useSessionStore } from '~/stores/session.store';
import { useStationStore } from '~/stores/station.store';
import { mount } from '~/test.utils';
import StationInfoCard from './StationInfoCard.vue';

vi.mock('~/services/control-panel.service', () => {
  const mock: Partial<ControlPanelService> = {
    editUser: vi.fn().mockReturnThis(),
  };

  return {
    ControlPanelService: vi.fn(() => mock),
  };
});

describe('StationInfoCard', () => {
  function initStation(principal: Principal, isMain: boolean, name: string | null) {
    const stationStore = useStationStore();
    const sessionStore = useSessionStore();

    sessionStore.$patch({
      data: {
        stations: [{ canisterId: principal.toText(), main: isMain, name }],
        selected: { canisterId: principal.toText(), hasAccess: true },
      },
    });
    stationStore.$patch({ canisterId: principal.toText() });
  }

  function addStation(principal: Principal, isMain: boolean, name: string | null) {
    const sessionStore = useSessionStore();
    const stationStore = useStationStore();

    sessionStore.$patch({
      data: {
        stations: [
          ...sessionStore.data.stations,
          { canisterId: principal.toText(), main: isMain, name },
        ],
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

  it('renders the station name', async () => {
    const wrapper = mount(StationInfoCard);

    const session = useSessionStore();
    session.data.stations = [
      {
        canisterId: Principal.anonymous().toText(),
        main: true,
        name: 'Personal',
      },
    ];

    await wrapper.vm.$nextTick();

    const nameLine = wrapper.find('[data-test-id="station-name"]');

    expect(nameLine.exists()).toBeTruthy();
    expect(nameLine.text()).toContain('Personal');
  });

  it('shows a remove station button', () => {
    const wrapper = mount(StationInfoCard);
    expect(wrapper.find('[data-test-id="remove-station-btn"]').exists()).toBe(true);
  });

  it('is disabled if the station is the main station', async () => {
    const wrapper = mount(StationInfoCard);
    initStation(stationCanisterId1, true, 'TEST WALLET');
    await wrapper.vm.$nextTick();
    const button = wrapper.find('[data-test-id="remove-station-btn"]');
    expect(button.attributes('disabled')).toBeDefined();
  });

  it('is not disabled if the station is the only station', async () => {
    const wrapper = mount(StationInfoCard);

    initStation(stationCanisterId1, true, 'TEST WALLET');
    addStation(stationCanisterId2, false, 'TEST WALLET 2');
    selectStation(stationCanisterId2);
    await wrapper.vm.$nextTick();

    const button = wrapper.find('[data-test-id="remove-station-btn"]');

    expect(button.attributes('disabled')).toBeUndefined();
  });

  it('calls editUser without the removed station when the dialog is confirmed', async () => {
    const wrapper = mount(StationInfoCard);

    initStation(stationCanisterId1, true, 'TEST WALLET');
    addStation(stationCanisterId2, false, 'TEST WALLET 2');
    selectStation(stationCanisterId2);
    await wrapper.vm.$nextTick();
    wrapper.find('[data-test-id="remove-station-btn"]').trigger('click');
    await wrapper.vm.$nextTick();

    document
      .querySelector('[data-test-id="action-btn-default-submit-btn"]')
      ?.dispatchEvent(new Event('click'));

    expect(services().controlPanel.editUser).toHaveBeenCalledWith(
      expect.objectContaining({
        stations: [
          [
            {
              canister_id: stationCanisterId1,
              name: ['TEST WALLET'],
            },
          ],
        ],
      }),
    );
  });
});
