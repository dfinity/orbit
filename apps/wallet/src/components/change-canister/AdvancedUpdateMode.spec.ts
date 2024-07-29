import { VueWrapper } from '@vue/test-utils';
import { describe, expect, it } from 'vitest';
import { ChangeCanisterFormValue } from '~/components/change-canister/change-canister.types';
import { mount } from '~/test.utils';
import { ChangeCanisterTargetType } from '~/types/station.types';
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
      InstanceType<typeof AdvancedUpdateMode> & { upgradeTarget: ChangeCanisterTargetType }
    >;

    // picks up the change to upgrade station
    wrapper.vm.upgradeTarget = ChangeCanisterTargetType.UpgradeStation;
    await wrapper.vm.$nextTick();
    const modelValue = wrapper.emitted('update:modelValue')?.[0]?.[0] as ChangeCanisterFormValue;
    expect(modelValue.target).toEqual({ UpgradeStation: null });

    // picks up the change to upgrade upgrader
    wrapper.vm.upgradeTarget = ChangeCanisterTargetType.UpgradeUpgrader;
    await wrapper.vm.$nextTick();
    const modelValue2 = wrapper.emitted('update:modelValue')?.[1]?.[0] as ChangeCanisterFormValue;
    expect(modelValue2.target).toEqual({ UpgradeUpgrader: null });
  });
});
