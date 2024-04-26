import { describe, expect, it, vi } from 'vitest';
import { StationService } from '~/services/station.service';
import { mount } from '~/test.utils';
import UserGroupAutocomplete from './UserGroupAutocomplete.vue';

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
    listUserGroups: vi.fn().mockImplementation(() =>
      Promise.resolve({
        user_groups: [],
        next_offset: [BigInt(0)],
        total: BigInt(0),
      }),
    ),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

describe('UserGroupAutocomplete', () => {
  it('renders with selected ids', () => {
    const wrapper = mount(UserGroupAutocomplete, {
      props: {
        modelValue: ['1'],
      },
    });

    expect(wrapper.exists()).toBe(true);

    const autocomplete = wrapper.findComponent({ name: 'VAutocomplete' });
    expect(autocomplete.exists()).toBe(true);

    expect(autocomplete.props('modelValue')).toEqual(['1']);
  });

  it('renders with empty list of user groups', async () => {
    const wrapper = mount(UserGroupAutocomplete);
    const autocomplete = wrapper.findComponent({ name: 'VAutocomplete' });

    expect(autocomplete.exists()).toBe(true);

    await wrapper.vm.$nextTick();

    expect(autocomplete.props('items')).toEqual([]);
  });
});
