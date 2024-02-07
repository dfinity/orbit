import { flushPromises } from '@vue/test-utils';
import { describe, expect, it, vi } from 'vitest';
import { User } from '~/generated/wallet/wallet.did';
import { useUsersAutocomplete } from '~/ui/composables/autocomplete.composable';
import { createMockRef, mount } from '~/ui/test.utils';
import SpecificUsersForm from './SpecificUsersForm.vue';

vi.mock('~/ui/composables/autocomplete.composable', () => {
  const mock: Partial<ReturnType<typeof useUsersAutocomplete>> = {
    searchItems: vi.fn(),
    loading: createMockRef(false),
    results: createMockRef<User[]>([]),
  };

  return {
    useUsersAutocomplete: vi.fn(() => mock),
  };
});

describe('SpecificUsersForm', () => {
  it('renders properly', () => {
    const wrapper = mount(SpecificUsersForm, {
      props: {
        modelValue: {
          policyId: null,
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
          policyId: null,
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
