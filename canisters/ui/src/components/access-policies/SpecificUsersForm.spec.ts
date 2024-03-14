import { flushPromises } from '@vue/test-utils';
import { describe, expect, it, vi } from 'vitest';
import { WalletService } from '~/services/wallet.service';
import { mount } from '~/test.utils';
import SpecificUsersForm from './SpecificUsersForm.vue';

vi.mock('~/services/wallet.service', () => {
  const mock: Partial<WalletService> = {
    withWalletId: vi.fn().mockReturnThis(),
    listUsers: vi.fn().mockImplementation(() =>
      Promise.resolve({
        users: [],
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

describe('SpecificUsersForm', () => {
  it('renders properly', () => {
    const wrapper = mount(SpecificUsersForm, {
      props: {
        modelValue: {
          userIds: [],
          prefilledUsers: [],
        },
        valid: false,
      },
    });

    expect(wrapper.exists()).toBe(true);
  });

  it('submits the form and emits it', async () => {
    const wrapper = mount(SpecificUsersForm, {
      props: {
        modelValue: {
          userIds: [],
          prefilledUsers: [
            { id: '1', name: 'Test1', status: { Active: null } },
            { id: '2', name: 'Test2', status: { Active: null } },
          ],
        },
        valid: false,
      },
    });

    const userIdsInput = wrapper.find('input[name="user_ids"]');
    expect(userIdsInput.exists()).toBe(true);

    await wrapper.setProps({
      modelValue: {
        ...wrapper.props().modelValue,
        userIds: ['1', '2'],
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
          prefilledUsers: [
            { id: '1', name: 'Test1', status: { Active: null } },
            { id: '2', name: 'Test2', status: { Active: null } },
          ],
          userIds: ['1', '2'],
        },
      ],
    ]);
  });
});
