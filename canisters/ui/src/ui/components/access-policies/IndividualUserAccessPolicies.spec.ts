import { flushPromises } from '@vue/test-utils';
import { describe, expect, it, vi } from 'vitest';
import { User } from '~/generated/wallet/wallet.did';
import { useUsersAutocomplete } from '~/ui/composables/autocomplete.composable';
import { createMockRef, mount } from '~/ui/test.utils';
import IndividualUserAccessPolicies from './IndividualUserAccessPolicies.vue';

vi.mock('~/ui/composables/autocomplete.composable', () => {
  const mock: Partial<ReturnType<typeof useUsersAutocomplete>> = {
    searchItems: vi.fn(),
    loading: createMockRef(false),
    results: createMockRef<User[]>([
      {
        id: '1',
        name: ['Test1'],
        groups: [],
        identities: [],
        last_modification_timestamp: '',
        status: { Active: null },
      },
    ]),
  };

  return {
    useUsersAutocomplete: vi.fn(() => mock),
  };
});

describe('IndividualUserAccessPolicies', () => {
  it('renders properly', () => {
    const wrapper = mount(IndividualUserAccessPolicies);

    expect(wrapper.exists()).toBe(true);
  });

  it('shows access policy list when specific resource is selected', async () => {
    const wrapper = mount(IndividualUserAccessPolicies);

    const selectInput = wrapper.find('[name="user_id"]');
    expect(selectInput.exists()).toBe(true);

    const autocomplete = wrapper.findComponent({ name: 'VAutocomplete' });
    autocomplete.vm.$emit('update:modelValue', '1');

    await flushPromises();

    expect(wrapper.find('[data-test-id="access-policy-list"]').exists()).toBe(true);
  });

  it('hides access policy list when specific resource is not selected', async () => {
    const wrapper = mount(IndividualUserAccessPolicies);

    const selectInput = wrapper.find('[name="user_id"]');
    expect(selectInput.exists()).toBe(true);

    const autocomplete = wrapper.findComponent({ name: 'VAutocomplete' });
    autocomplete.vm.$emit('update:modelValue', '');

    await flushPromises();

    expect(wrapper.find('[data-test-id="access-policy-list"]').exists()).toBe(false);
  });
});
