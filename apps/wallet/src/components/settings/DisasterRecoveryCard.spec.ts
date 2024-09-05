import { flushPromises } from '@vue/test-utils';
import { describe, expect, it, vi } from 'vitest';
import { VCard, VOverlay } from 'vuetify/components';
import { SystemInfoResult } from '~/generated/station/station.did';
import { services } from '~/plugins/services.plugin';
import { mount } from '~/test.utils';
import { ExtractOk } from '~/types/helper.types';
import DisasterRecoveryCard from './DisasterRecoveryCard.vue';

vi.mock('~/services/station.service', () => ({
  StationService: vi.fn().mockImplementation(() => {
    return {};
  }),
}));

describe('DisasterRecoveryCard', () => {
  it('renders properly', () => {
    const wrapper = mount(DisasterRecoveryCard);

    expect(wrapper.exists()).toBe(true);
  });

  it('shows a warning if DR is not configured', async () => {
    services().station.systemInfo = vi.fn(() =>
      Promise.resolve({
        system: {
          disaster_recovery: [],
        },
      } as ExtractOk<SystemInfoResult>),
    );

    const wrapper = mount(DisasterRecoveryCard);

    await flushPromises();

    expect(wrapper.find('[data-test-id="dr-not-configured"]').exists()).toBe(true);

    vi.restoreAllMocks();
  });

  it('shows the DR configuration', async () => {
    services().station.systemInfo = vi.fn(() =>
      Promise.resolve({
        system: {
          disaster_recovery: [
            {
              committee: {
                quorum: 1234,
                user_group_id: '000-001',
              },
              user_group_name: ['DRGroup'],
            },
          ],
        },
      } as ExtractOk<SystemInfoResult>),
    );

    const wrapper = mount(DisasterRecoveryCard);

    await flushPromises();

    expect(wrapper.text()).toContain('DRGroup');
    expect(wrapper.text()).toContain('1234');
    expect(wrapper.text()).toContain('000-001');

    // no privilege by default
    expect(wrapper.find('[data-test-id="configure-dr-btn"]').exists()).toBe(false);
  });

  it('shows the a button to configure DR if the user has privilege', async () => {
    services().station.systemInfo = vi.fn(() =>
      Promise.resolve({
        system: {
          disaster_recovery: [
            {
              committee: {
                quorum: 1234,
                user_group_id: '000-001',
              },
              user_group_name: ['DRGroup'],
            },
          ],
        },
      } as ExtractOk<SystemInfoResult>),
    );

    const wrapper = mount(
      DisasterRecoveryCard,
      {},
      {
        initialPiniaState: {
          session: { isAuthenticated: true },
          station: {
            privileges: [{ SystemUpgrade: null }],
          },
        },
      },
    );

    await flushPromises();

    expect(wrapper.find('[data-test-id="configure-dr-btn"]').exists()).toBe(true);

    vi.restoreAllMocks();
  });

  it('opens a dialog to configure DR when the button is clicked', async () => {
    services().station.systemInfo = vi.fn(() =>
      Promise.resolve({
        system: {
          disaster_recovery: [],
        },
      } as ExtractOk<SystemInfoResult>),
    );

    services().station.createSetDisasterRecoveryCommitteeRequest = vi.fn();

    const wrapper = mount(
      DisasterRecoveryCard,
      {},
      {
        initialPiniaState: {
          session: { isAuthenticated: true },
          station: {
            privileges: [{ SystemUpgrade: null }],
          },
        },
      },
    );

    await flushPromises();

    await wrapper.find('[data-test-id="configure-dr-btn"]').trigger('click');

    const form = wrapper.findComponent(VOverlay).findComponent(VCard);
    await form.find('input[name="quorum"]').setValue('1');
    await form.findComponent({ name: 'VAutocomplete' }).setValue('000-001');

    await flushPromises();

    await form.find('[data-test-id="save-dr-btn"]').trigger('click');

    expect(services().station.createSetDisasterRecoveryCommitteeRequest).toHaveBeenCalledWith({
      quorum: 1,
      user_group_id: '000-001',
    });
  });
});
