import AccountAccessSettings from './AccountAccessSettings.vue';
import { mount } from '~/test.utils';
import { describe, expect, it, vi } from 'vitest';
import { Allow } from '~/generated/station/station.did';
import { StationService } from '~/services/station.service';

const allowAuthenticated = (): Allow => ({
  auth_scope: { Authenticated: null },
  user_groups: [],
  users: [],
});

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
    listUserGroups: vi.fn().mockImplementation(() =>
      Promise.resolve({
        user_groups: [],
        privileges: [],
        next_offset: [BigInt(0)],
        total: BigInt(0),
      }),
    ),
    listUsers: vi.fn().mockImplementation(() =>
      Promise.resolve({
        users: [],
        next_offset: [BigInt(0)],
        total: BigInt(0),
      }),
    ),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

describe('AccountAccessSettings', () => {
  it('mounts with default values', () => {
    const wrapper = mount(AccountAccessSettings, {
      props: {
        modelValue: {
          configuration: allowAuthenticated(),
          read: allowAuthenticated(),
          transfer: allowAuthenticated(),
        },
      },
    });

    expect(wrapper.find('[data-test-id="read-access"]').exists()).toBeTruthy();
    expect(wrapper.find('[data-test-id="update-access"]').exists()).toBeTruthy();
    expect(wrapper.find('[data-test-id="transfer-access"]').exists()).toBeTruthy();
  });
});
