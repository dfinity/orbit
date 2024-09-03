import { flushPromises } from '@vue/test-utils';
import { describe, expect, it, vi } from 'vitest';
import { StationService } from '~/services/station.service';
import { mount } from '~/test.utils';
import ExternalCanisterLabelAutocomplete from './ExternalCanisterLabelAutocomplete.vue';

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
    fetchExternalCanisterFilters: vi.fn().mockImplementation(() =>
      Promise.resolve({
        labels: [['production']],
        names: [],
      }),
    ),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

describe('ExternalCanisterLabelAutocomplete', () => {
  it('renders with empty list of labels', () => {
    const wrapper = mount(ExternalCanisterLabelAutocomplete);
    const autocomplete = wrapper.findComponent({ name: 'VAutocomplete' });

    expect(autocomplete.exists()).toBe(true);
    expect(autocomplete.props('items')).toEqual([]);
  });

  it('renders with selected labels', async () => {
    const wrapper = mount(ExternalCanisterLabelAutocomplete, {
      props: {
        modelValue: ['production'],
      },
    });

    await flushPromises();

    expect(wrapper.exists()).toBe(true);

    const autocomplete = wrapper.findComponent({ name: 'VAutocomplete' });
    expect(autocomplete.exists()).toBe(true);

    expect(autocomplete.props('modelValue')).toEqual(['production']);
    expect(autocomplete.props('items')).toEqual([
      {
        text: 'production',
        value: 'production',
      },
    ]);
  });
});
