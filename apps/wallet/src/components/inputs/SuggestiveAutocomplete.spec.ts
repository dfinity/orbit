import { flushPromises } from '@vue/test-utils';
import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import { SelectItem } from '~/types/helper.types';
import SuggestiveAutocomplete from './SuggestiveAutocomplete.vue';

describe('SuggestiveAutocomplete', () => {
  it('renders with no values', () => {
    const input = mount(SuggestiveAutocomplete);
    const searchInput = input.findComponent({ name: 'VTextField' });

    expect(searchInput.exists()).toBe(true);
    expect(searchInput.props('modelValue')).toEqual('');
  });

  it('selected values are not available in the selection list', async () => {
    const input = mount(SuggestiveAutocomplete, {
      props: {
        modelValue: ['testing'],
        items: [
          {
            text: 'staging',
            value: 'staging',
          },
        ],
      },
    });

    const vm = input.vm as typeof input.vm & { unselectedItems: SelectItem<string>[] };
    expect(vm.unselectedItems).toEqual([{ text: 'staging', value: 'staging' }]);
  });

  it('on search unselected items are updated', async () => {
    const input = mount(SuggestiveAutocomplete, {
      props: {
        modelValue: [],
        items: [{ text: 'staging', value: 'staging' }],
        fetchItems: () => [{ text: 'testing', value: 'testing' }],
      },
    });

    const vm = input.vm as typeof input.vm & { unselectedItems: SelectItem<string>[] };
    expect(vm.unselectedItems).toEqual([{ text: 'staging', value: 'staging' }]);

    await input.findComponent({ name: 'VTextField' }).setValue('test');
    await flushPromises();

    expect(vm.unselectedItems).toEqual([{ text: 'testing', value: 'testing' }]);
  });

  it('on search if no items are found, the list is empty', async () => {
    const input = mount(SuggestiveAutocomplete, {
      props: {
        modelValue: [],
        items: [{ text: 'staging', value: 'staging' }],
        fetchItems: () => [],
      },
    });

    const vm = input.vm as typeof input.vm & { unselectedItems: SelectItem<string>[] };

    await input.findComponent({ name: 'VTextField' }).setValue('test');
    await flushPromises();

    expect(vm.unselectedItems).toEqual([]);
  });

  it('disables the creation of new items', async () => {
    const input = mount(SuggestiveAutocomplete, {
      props: {
        modelValue: [],
        items: [{ text: 'staging', value: 'staging' }],
        create: false,
      },
    });
    const searchInput = input.findComponent({ name: 'VTextField' });

    await searchInput.setValue('test');
    await searchInput.trigger('keydown.enter');

    expect(input.emitted('update:modelValue')).toBeUndefined();
  });

  it('enable the creation of new items', async () => {
    const input = mount(SuggestiveAutocomplete, {
      props: {
        modelValue: [],
        items: [{ text: 'staging', value: 'staging' }],
        create: true,
      },
    });
    const searchInput = input.findComponent({ name: 'VTextField' });

    await searchInput.setValue('test');
    await searchInput.trigger('keydown.enter');

    expect(input.emitted('update:modelValue')?.[0]?.[0]).toEqual(['test']);
  });
});
