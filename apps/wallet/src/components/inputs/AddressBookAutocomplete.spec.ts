import { describe, expect, it, vi } from 'vitest';
import { StationService } from '~/services/station.service';
import { mount } from '~/test.utils';
import AddressBookAutocomplete from './AddressBookAutocomplete.vue';
import { AddressBookEntry } from '~/generated/station/station.did';

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
    listAddressBook: vi.fn().mockImplementation(() =>
      Promise.resolve({
        address_book_entries: [],
        next_offset: [BigInt(0)],
        total: BigInt(0),
      }),
    ),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

describe('AddressBookAutocomplete', () => {
  it('renders with selected ids', () => {
    const wrapper = mount(AddressBookAutocomplete, {
      props: {
        modelValue: [
          {
            id: '1',
          } as AddressBookEntry,
        ],
      },
    });

    expect(wrapper.exists()).toBe(true);

    const autocomplete = wrapper.findComponent({ name: 'VAutocomplete' });
    expect(autocomplete.exists()).toBe(true);

    expect(autocomplete.props('modelValue')).toEqual([{ id: '1' }]);
  });

  it('renders with empty list of address book entries', async () => {
    const wrapper = mount(AddressBookAutocomplete);
    const autocomplete = wrapper.findComponent({ name: 'VAutocomplete' });

    expect(autocomplete.exists()).toBe(true);

    await wrapper.vm.$nextTick();

    expect(autocomplete.props('items')).toEqual([]);
  });
});
