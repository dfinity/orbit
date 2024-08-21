import { VueWrapper } from '@vue/test-utils';
import { describe, expect, it } from 'vitest';
import { SystemUpgradeFormValue } from '~/components/system-upgrade/system-upgrade.types';
import { mount } from '~/test.utils';
import { SystemUpgradeTargetType } from '~/types/station.types';
import AdvancedUpdateMode from './AdvancedUpdateMode.vue';

describe('AdvancedUpdateMode', () => {
  it('renders with empty form', () => {
    const wrapper = mount(AdvancedUpdateMode, {
      props: {
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

  it('when target type changes the modelValue picks up the change', async () => {
    const wrapper = mount(AdvancedUpdateMode, {
      props: {
        modelValue: {
          wasmInitArg: undefined,
          target: undefined,
          wasmModule: undefined,
        },
      },
    }) as unknown as VueWrapper<
      InstanceType<typeof AdvancedUpdateMode> & { upgradeTarget: SystemUpgradeTargetType }
    >;

    // picks up the change to upgrade station
    wrapper.vm.upgradeTarget = SystemUpgradeTargetType.UpgradeStation;
    await wrapper.vm.$nextTick();
    const modelValue = wrapper.emitted('update:modelValue')?.[0]?.[0] as SystemUpgradeFormValue;
    expect(modelValue.target).toEqual({ UpgradeStation: null });

    // picks up the change to upgrade upgrader
    wrapper.vm.upgradeTarget = SystemUpgradeTargetType.UpgradeUpgrader;
    await wrapper.vm.$nextTick();
    const modelValue2 = wrapper.emitted('update:modelValue')?.[1]?.[0] as SystemUpgradeFormValue;
    expect(modelValue2.target).toEqual({ UpgradeUpgrader: null });
  });
});
