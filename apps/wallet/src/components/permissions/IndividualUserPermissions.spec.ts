import { flushPromises } from '@vue/test-utils';
import { describe, expect, it, vi } from 'vitest';
import { StationService } from '~/services/station.service';
import { mount } from '~/test.utils';
import IndividualUserPermissions from './IndividualUserPermissions.vue';

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

describe('IndividualUserPermissions', () => {
  it('renders properly', () => {
    const wrapper = mount(IndividualUserPermissions);

    expect(wrapper.exists()).toBe(true);
  });

  it('shows permission list when specific resource is selected', async () => {
    const wrapper = mount(IndividualUserPermissions);

    const selectInput = wrapper.find('[name="user_id"]');
    expect(selectInput.exists()).toBe(true);

    const autocomplete = wrapper.findComponent({ name: 'VAutocomplete' });
    autocomplete.vm.$emit('update:modelValue', '1');

    await flushPromises();

    expect(wrapper.find('[data-test-id="access-policy-list"]').exists()).toBe(true);
  });

  it('hides permission list when specific resource is not selected', async () => {
    const wrapper = mount(IndividualUserPermissions);

    const selectInput = wrapper.find('[name="user_id"]');
    expect(selectInput.exists()).toBe(true);

    const autocomplete = wrapper.findComponent({ name: 'VAutocomplete' });
    autocomplete.vm.$emit('update:modelValue', '');

    await flushPromises();

    expect(wrapper.find('[data-test-id="access-policy-list"]').exists()).toBe(false);
  });
});
