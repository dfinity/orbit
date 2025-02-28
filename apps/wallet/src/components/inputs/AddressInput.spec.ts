import { describe, expect, it, vi } from 'vitest';
import { StationService } from '~/services/station.service';
import { mount } from '~/test.utils';
import AddressInput from './AddressInput.vue';
import { AddressBookEntry } from '~/generated/station/station.did';
import { flushPromises } from '@vue/test-utils';

const validIcpAddress1 = '5c76bc95e544204de4928e4d901e52b49df248b9c346807040e7af75aa61f4b3';
const validIcpAddress2 = '3d84432911eab50b97dfea077339ba2ffe2cfa075d6ae5da0bfabe4df6264b41';

const addressBookEntry = {
  id: '1',
  address: validIcpAddress1,
  address_owner: 'owner',
} as AddressBookEntry;

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
    listAddressBook: vi.fn().mockImplementation(() =>
      Promise.resolve({
        address_book_entries: [addressBookEntry],
        next_offset: [BigInt(0)],
        total: BigInt(0),
      }),
    ),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

describe('AddressInput', () => {
  it('renders with empty address', () => {
    const wrapper = mount(AddressInput, {
      props: {
        blockchain: 'icp',
      },
    });

    expect(wrapper.exists()).toBe(true);

    const input = wrapper.findComponent({ name: 'VCombobox' });
    expect(input.exists()).toBe(true);

    expect(input.props('modelValue')).toEqual(null);
  });

  it('renders with starting address', async () => {
    const wrapper = mount(AddressInput, {
      props: {
        blockchain: 'icp',
        modelValue: validIcpAddress1,
      },
    });
    const input = wrapper.findComponent({ name: 'VCombobox' });

    expect(input.exists()).toBe(true);

    await wrapper.vm.$nextTick();

    expect(input.props('modelValue')).toEqual(validIcpAddress1);
  });

  it('loads addresses from the address book', async () => {
    vi.useFakeTimers();

    const wrapper = mount(AddressInput, {
      props: {
        blockchain: 'icp',
      },
    });
    const input = wrapper.findComponent({ name: 'VCombobox' });

    expect(input.exists()).toBe(true);

    vi.advanceTimersByTime(1000);
    await wrapper.vm.$nextTick();
    await flushPromises();

    expect(input.props('items')).toEqual([addressBookEntry]);
  });

  it('accepts new addresses', async () => {
    const wrapper = mount(AddressInput, {
      props: {
        blockchain: 'icp',
      },
    });
    const input = wrapper.findComponent({ name: 'VCombobox' });
    await input.find('input').setValue(validIcpAddress2);
    expect(wrapper.emitted('update:modelValue')).toEqual([[validIcpAddress2]]);
  });
});
