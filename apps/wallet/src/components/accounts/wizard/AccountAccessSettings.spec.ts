import { describe } from 'node:test';
import AccountAccessSettings from './AccountAccessSettings.vue';
import { mount } from '~/test.utils';
import { expect, it, vi } from 'vitest';
import { Allow } from '~/generated/station/station.did';
import { WalletService } from '~/services/wallet.service';

const allowAuthenticated = (): Allow => ({
  auth_scope: { Authenticated: null },
  user_groups: [],
  users: [],
});

vi.mock('~/services/wallet.service', () => {
  const mock: Partial<WalletService> = {
    withWalletId: vi.fn().mockReturnThis(),
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
    WalletService: vi.fn(() => mock),
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
