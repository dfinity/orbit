import { flushPromises } from '@vue/test-utils';
import { describe, expect, it, vi } from 'vitest';
import { UserGroup } from '~/generated/wallet/wallet.did';
import { useUserGroupsAutocomplete } from '~/ui/composables/autocomplete.composable';
import { createMockRef, mount } from '~/ui/test.utils';
import MembersOfGroupForm from './MembersOfGroupForm.vue';

vi.mock('~/ui/composables/autocomplete.composable', () => {
  const mock: Partial<ReturnType<typeof useUserGroupsAutocomplete>> = {
    searchItems: vi.fn(),
    loading: createMockRef(false),
    results: createMockRef<UserGroup[]>([]),
  };

  return {
    useUserGroupsAutocomplete: vi.fn(() => mock),
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
