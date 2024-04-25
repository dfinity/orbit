import { describe, expect, it, vi } from 'vitest';
import { WalletService } from '~/services/wallet.service';
import { mount } from '~/test.utils';
import UserAutocomplete from './UserAutocomplete.vue';

vi.mock('~/services/wallet.service', () => {
  const mock: Partial<WalletService> = {
    withWalletId: vi.fn().mockReturnThis(),
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

describe('UserAutocomplete', () => {
  it('renders with selected ids', () => {
    const wrapper = mount(UserAutocomplete, {
      props: {
        modelValue: ['1'],
      },
    });

    expect(wrapper.exists()).toBe(true);

    const autocomplete = wrapper.findComponent({ name: 'VAutocomplete' });
    expect(autocomplete.exists()).toBe(true);

    expect(autocomplete.props('modelValue')).toEqual(['1']);
  });

  it('renders with empty list of users', async () => {
    const wrapper = mount(UserAutocomplete);
    const autocomplete = wrapper.findComponent({ name: 'VAutocomplete' });

    expect(autocomplete.exists()).toBe(true);

    await wrapper.vm.$nextTick();

    expect(autocomplete.props('items')).toEqual([]);
  });
});
