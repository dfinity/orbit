import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import ChangeCanisterForm from './ChangeCanisterForm.vue';

describe('ChangeCanisterForm', () => {
  it('renders with empty form', () => {
    const wrapper = mount(ChangeCanisterForm, {
      props: {
        modelValue: {
          arg: null,
          target: null,
          wasmModule: undefined,
        },
      },
    });

    expect(wrapper.exists()).toBe(true);
    expect(wrapper.find('[name="target"]').exists()).toBe(true);
    expect(wrapper.find('[name="arg"]').exists()).toBe(true);
    expect(wrapper.find('[name="wasm"]').exists()).toBe(true);
  });
});
