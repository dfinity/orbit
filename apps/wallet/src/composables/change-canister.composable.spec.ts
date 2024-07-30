import { describe, expect, it } from 'vitest';
import {
  useDefaultUpgradeModel,
  useUpgradeTargets,
} from '~/composables/change-canister.composable';
import { setupComponent } from '~/test.utils';
import { ChangeCanisterTargetType } from '~/types/station.types';

describe('change-canister.composable', () => {
  it('should return upgrade targets with correct targets', () => {
    const vm = setupComponent(() => ({ targets: useUpgradeTargets() }));

    expect(vm.targets.station.value).toEqual(ChangeCanisterTargetType.UpgradeStation);
    expect(vm.targets.upgrader.value).toEqual(ChangeCanisterTargetType.UpgradeUpgrader);
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
