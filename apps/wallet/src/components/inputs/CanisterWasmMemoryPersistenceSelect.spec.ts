import { describe, expect, it } from 'vitest';
import { VSelect } from 'vuetify/components';
import { mount } from '~/test.utils';
import CanisterWasmMemoryPersistenceSelect from './CanisterWasmMemoryPersistenceSelect.vue';

describe('CanisterWasmMemoryPersistenceSelect', () => {
  it('renders a select', () => {
    const wrapper = mount(CanisterWasmMemoryPersistenceSelect, {
      props: { modelValue: undefined },
    });

    expect(wrapper.findComponent(VSelect).exists()).toBe(true);
  });

  it('maps the candid value to the select option', () => {
    expect(
      mount(CanisterWasmMemoryPersistenceSelect, { props: { modelValue: undefined } })
        .findComponent(VSelect)
        .props('modelValue'),
    ).toBe('default');

    expect(
      mount(CanisterWasmMemoryPersistenceSelect, { props: { modelValue: { keep: null } } })
        .findComponent(VSelect)
        .props('modelValue'),
    ).toBe('keep');

    expect(
      mount(CanisterWasmMemoryPersistenceSelect, { props: { modelValue: { replace: null } } })
        .findComponent(VSelect)
        .props('modelValue'),
    ).toBe('replace');
  });

  it('emits the candid union value when an option is selected', async () => {
    const wrapper = mount(CanisterWasmMemoryPersistenceSelect, {
      props: { modelValue: undefined },
    });
    const select = wrapper.findComponent(VSelect);

    select.vm.$emit('update:modelValue', 'keep');
    await wrapper.vm.$nextTick();
    expect(wrapper.emitted('update:modelValue')?.at(-1)?.[0]).toEqual({ keep: null });

    select.vm.$emit('update:modelValue', 'replace');
    await wrapper.vm.$nextTick();
    expect(wrapper.emitted('update:modelValue')?.at(-1)?.[0]).toEqual({ replace: null });
  });

  it('emits undefined when the default option is selected', async () => {
    const wrapper = mount(CanisterWasmMemoryPersistenceSelect, {
      props: { modelValue: { keep: null } },
    });

    wrapper.findComponent(VSelect).vm.$emit('update:modelValue', 'default');
    await wrapper.vm.$nextTick();

    expect(wrapper.emitted('update:modelValue')?.at(-1)?.[0]).toBeUndefined();
  });
});
