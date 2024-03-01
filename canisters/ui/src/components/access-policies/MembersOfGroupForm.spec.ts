import { flushPromises } from '@vue/test-utils';
import { describe, expect, it, vi } from 'vitest';
import { WalletService } from '~/services/wallet.service';
import { mount } from '~/test.utils';
import MembersOfGroupForm from './MembersOfGroupForm.vue';

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
  };

  return {
    WalletService: vi.fn(() => mock),
  };
});

describe('MembersOfGroupForm', () => {
  it('renders properly', () => {
    const wrapper = mount(MembersOfGroupForm, {
      props: {
        modelValue: {
          policyId: null,
          groupIds: [],
          prefilledGroups: [],
        },
        valid: false,
      },
    });

    expect(wrapper.exists()).toBe(true);
  });

  it('submits the form and emits it', async () => {
    const wrapper = mount(MembersOfGroupForm, {
      props: {
        modelValue: {
          policyId: null,
          groupIds: [],
          prefilledGroups: [
            { id: '1', name: 'Test1' },
            { id: '2', name: 'Test2' },
          ],
        },
        valid: false,
      },
    });

    const autocompleteInput = wrapper.find('input[name="group_ids"]');
    expect(autocompleteInput.exists()).toBe(true);

    await wrapper.setProps({
      modelValue: {
        ...wrapper.props().modelValue,
        groupIds: ['1'],
      },
    });

    const form = wrapper.findComponent({ ref: 'form' });
    await form.trigger('submit');

    await flushPromises();

    expect(wrapper.emitted('submit')).toBeTruthy();
    expect(wrapper.emitted('submit')).toEqual([
      [
        {
          policyId: null,
          prefilledGroups: [
            { id: '1', name: 'Test1' },
            { id: '2', name: 'Test2' },
          ],
          groupIds: ['1'],
        },
      ],
    ]);
  });
});
