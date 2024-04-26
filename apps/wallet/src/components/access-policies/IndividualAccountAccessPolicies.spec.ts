import { flushPromises } from '@vue/test-utils';
import { describe, expect, it, vi } from 'vitest';
import { Account } from '~/generated/station/station.did';
import { StationService } from '~/services/station.service';
import { mount } from '~/test.utils';
import IndividualAccountAccessPolicies from './IndividualAccountAccessPolicies.vue';

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
    listAccounts: vi.fn().mockImplementation(() =>
      Promise.resolve({
        accounts: [
          {
            id: '1',
            name: 'Test1',
          } as Account,
        ],
        next_offset: [BigInt(0)],
        total: BigInt(0),
      }),
    ),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

describe('IndividualAccountAccessPolicies', () => {
  it('renders properly', () => {
    const wrapper = mount(IndividualAccountAccessPolicies);

    expect(wrapper.exists()).toBe(true);
  });

  it('shows access policy list when specific resource is selected', async () => {
    const wrapper = mount(IndividualAccountAccessPolicies);

    const selectInput = wrapper.find('[name="account_id"]');
    expect(selectInput.exists()).toBe(true);

    const autocomplete = wrapper.findComponent({ name: 'VAutocomplete' });
    autocomplete.vm.$emit('update:modelValue', '1');

    await flushPromises();

    expect(wrapper.find('[data-test-id="access-policy-list"]').exists()).toBe(true);
  });

  it('hides access policy list when specific resource is not selected', async () => {
    const wrapper = mount(IndividualAccountAccessPolicies);

    const selectInput = wrapper.find('[name="account_id"]');
    expect(selectInput.exists()).toBe(true);

    const autocomplete = wrapper.findComponent({ name: 'VAutocomplete' });
    autocomplete.vm.$emit('update:modelValue', '');

    await flushPromises();

    expect(wrapper.find('[data-test-id="access-policy-list"]').exists()).toBe(false);
  });
});
