import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import SystemUpgradeForm from './SystemUpgradeForm.vue';
import { SystemUpgradeFormMode } from './system-upgrade.types';

describe('SystemUpgradeForm', () => {
  it('renders with empty form', () => {
    const wrapper = mount(SystemUpgradeForm, {
      props: {
        mode: SystemUpgradeFormMode.Advanced,
        modelValue: {
          wasmInitArg: undefined,
          target: undefined,
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
