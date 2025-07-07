import { Principal } from '@icp-sdk/core/principal';
import { flushPromises } from '@vue/test-utils';
import { describe, expect, it, vi } from 'vitest';
import { StationService } from '~/services/station.service';
import { mount } from '~/test.utils';
import ExternalCanisterAutocomplete from './ExternalCanisterAutocomplete.vue';

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
    fetchExternalCanisterFilters: vi.fn().mockImplementation(() =>
      Promise.resolve({
        labels: [],
        names: [[{ name: 'dapp', canister_id: Principal.anonymous() }]],
      }),
    ),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

describe('ExternalCanisterAutocomplete', () => {
  it('renders with empty list of canisters', () => {
    const wrapper = mount(ExternalCanisterAutocomplete);
    const autocomplete = wrapper.findComponent({ name: 'VAutocomplete' });

    expect(autocomplete.exists()).toBe(true);
    expect(autocomplete.props('items')).toEqual([]);
  });

  it('renders with selected canisters', async () => {
    const wrapper = mount(ExternalCanisterAutocomplete, {
      props: {
        modelValue: [Principal.anonymous().toText()],
        items: [{ text: 'dapp', value: Principal.anonymous().toText() }],
      },
    });

    await flushPromises();

    expect(wrapper.exists()).toBe(true);

    const autocomplete = wrapper.findComponent({ name: 'VAutocomplete' });
    expect(autocomplete.exists()).toBe(true);

    expect(autocomplete.props('modelValue')).toEqual([Principal.anonymous().toText()]);
    expect(autocomplete.props('items')).toEqual([
      {
        text: 'dapp',
        value: Principal.anonymous().toText(),
      },
    ]);
  });
});
