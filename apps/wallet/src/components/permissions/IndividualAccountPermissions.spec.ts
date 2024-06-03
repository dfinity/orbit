import { flushPromises } from '@vue/test-utils';
import { describe, expect, it, vi } from 'vitest';
import { Account } from '~/generated/station/station.did';
import { StationService } from '~/services/station.service';
import { mount } from '~/test.utils';
import IndividualAccountPermissions from './IndividualAccountPermissions.vue';

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

describe('IndividualAccountPermissions', () => {
  it('renders properly', () => {
    const wrapper = mount(IndividualAccountPermissions);

    expect(wrapper.exists()).toBe(true);
  });

  it('shows permission list when specific resource is selected', async () => {
    const wrapper = mount(IndividualAccountPermissions);

    const selectInput = wrapper.find('[name="account_id"]');
    expect(selectInput.exists()).toBe(true);

    const autocomplete = wrapper.findComponent({ name: 'VAutocomplete' });
    autocomplete.vm.$emit('update:modelValue', '1');

    await flushPromises();

    expect(wrapper.find('[data-test-id="permission-list"]').exists()).toBe(true);
  });

  it('hides permission list when specific resource is not selected', async () => {
    const wrapper = mount(IndividualAccountPermissions);

    const selectInput = wrapper.find('[name="account_id"]');
    expect(selectInput.exists()).toBe(true);

    const autocomplete = wrapper.findComponent({ name: 'VAutocomplete' });
    autocomplete.vm.$emit('update:modelValue', '');

    await flushPromises();

    expect(wrapper.find('[data-test-id="permission-list"]').exists()).toBe(false);
  });
});
