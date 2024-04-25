import { flushPromises } from '@vue/test-utils';
import { describe, expect, it, vi } from 'vitest';
import { WalletService } from '~/services/wallet.service';
import { mount } from '~/test.utils';
import IndividualUserGroupAccessPolicies from './IndividualUserGroupAccessPolicies.vue';

vi.mock('~/services/wallet.service', () => {
  const mock: Partial<WalletService> = {
    withWalletId: vi.fn().mockReturnThis(),
    listUserGroups: vi.fn().mockImplementation(() =>
      Promise.resolve({
        user_groups: [
          {
            id: '1',
            name: 'Test1',
          },
        ],
        privileges: [],
        next_offset: [],
        total: BigInt(1),
      }),
    ),
  };

  return {
    WalletService: vi.fn(() => mock),
  };
});

describe('IndividualUserGroupAccessPolicies', () => {
  it('renders properly', () => {
    const wrapper = mount(IndividualUserGroupAccessPolicies);

    expect(wrapper.exists()).toBe(true);
  });

  it('shows access policy list when specific resource is selected', async () => {
    const wrapper = mount(IndividualUserGroupAccessPolicies);

    const selectInput = wrapper.find('[name="user_group_id"]');
    expect(selectInput.exists()).toBe(true);

    const autocomplete = wrapper.findComponent({ name: 'VAutocomplete' });
    autocomplete.vm.$emit('update:modelValue', '1');

    await flushPromises();

    expect(wrapper.find('[data-test-id="access-policy-list"]').exists()).toBe(true);
  });

  it('hides access policy list when specific resource is not selected', async () => {
    const wrapper = mount(IndividualUserGroupAccessPolicies);

    const selectInput = wrapper.find('[name="user_group_id"]');
    expect(selectInput.exists()).toBe(true);

    const autocomplete = wrapper.findComponent({ name: 'VAutocomplete' });
    autocomplete.vm.$emit('update:modelValue', '');

    await flushPromises();

    expect(wrapper.find('[data-test-id="access-policy-list"]').exists()).toBe(false);
  });
});
