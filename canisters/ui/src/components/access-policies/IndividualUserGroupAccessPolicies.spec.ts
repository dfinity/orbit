import { flushPromises } from '@vue/test-utils';
import { describe, expect, it, vi } from 'vitest';
import { UserGroup } from '~/generated/wallet/wallet.did';
import { useUserGroupsAutocomplete } from '~/composables/autocomplete.composable';
import { createMockRef, mount } from '~/test.utils';
import IndividualUserGroupAccessPolicies from './IndividualUserGroupAccessPolicies.vue';

vi.mock('~/composables/autocomplete.composable', () => {
  const mock: Partial<ReturnType<typeof useUserGroupsAutocomplete>> = {
    searchItems: vi.fn(),
    loading: createMockRef(false),
    results: createMockRef<UserGroup[]>([
      {
        id: '1',
        name: 'Test1',
      },
    ]),
  };

  return {
    useUserGroupsAutocomplete: vi.fn(() => mock),
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
