import { flushPromises } from '@vue/test-utils';
import { describe, expect, it, vi } from 'vitest';
import { StationService } from '~/services/station.service';
import { mount } from '~/test.utils';
import IndividualUserAccessPolicies from './IndividualUserAccessPolicies.vue';

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
    listUsers: vi.fn().mockImplementation(() =>
      Promise.resolve({
        users: [
          {
            id: '1',
            name: ['Test1'],
            groups: [],
            identities: [],
            last_modification_timestamp: '',
            status: { Active: null },
          },
        ],
        privileges: [],
        next_offset: [],
        total: BigInt(1),
      }),
    ),
  };

  return {
    StationService: vi.fn(() => mock),
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
