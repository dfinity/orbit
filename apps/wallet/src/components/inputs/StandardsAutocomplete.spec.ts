import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import StandardsAutocomplete from './StandardsAutocomplete.vue';

describe('StandardsAutocomplete', () => {
  it('renders with selected ids', () => {
    const wrapper = mount(StandardsAutocomplete, {
      props: {
        modelValue: ['1'],
        blockchain: 'icp',
      },
    });

    expect(wrapper.exists()).toBe(true);

    const autocomplete = wrapper.findComponent({ name: 'VSelect' });
    expect(autocomplete.exists()).toBe(true);

    expect(autocomplete.props('modelValue')).toEqual(['1']);
  });

  it('renders with empty list of standards', async () => {
    const wrapper = mount(StandardsAutocomplete, {
      props: {
        blockchain: 'icp',
      },
    });
    const autocomplete = wrapper.findComponent({ name: 'VSelect' });

    expect(autocomplete.exists()).toBe(true);

    await wrapper.vm.$nextTick();

    expect(autocomplete.props('items')).toEqual([]);
  });
});
