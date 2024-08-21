import { describe, expect, it } from 'vitest';
import { useDefaultUpgradeModel, useUpgradeTargets } from '~/composables/system-upgrade.composable';
import { setupComponent } from '~/test.utils';
import { SystemUpgradeTargetType } from '~/types/station.types';

describe('system-upgrade.composable', () => {
  it('should return upgrade targets with correct targets', () => {
    const vm = setupComponent(() => ({ targets: useUpgradeTargets() }));

    expect(vm.targets.station.value).toEqual(SystemUpgradeTargetType.UpgradeStation);
    expect(vm.targets.upgrader.value).toEqual(SystemUpgradeTargetType.UpgradeUpgrader);
  });

  it('should return default form value with empty values', () => {
    const { modelValue } = useDefaultUpgradeModel();

    expect(modelValue).toEqual({
      target: undefined,
      wasmModule: undefined,
      wasmInitArg: undefined,
      comment: undefined,
    });
  });

  it('by default upgrade form should be invalid', () => {
    const { valid } = useDefaultUpgradeModel();

    expect(valid).toBe(false);
  });
});
