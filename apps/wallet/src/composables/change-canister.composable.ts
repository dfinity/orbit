import { computed, ComputedRef } from 'vue';
import { useI18n } from 'vue-i18n';
import { ChangeCanisterFormValue } from '~/components/change-canister/change-canister.types';
import { ChangeCanisterTargetType } from '~/types/station.types';

export const useUpgradeTargets = (): ComputedRef<{
  station: { value: ChangeCanisterTargetType; title: string };
  upgrader: { value: ChangeCanisterTargetType; title: string };
}> => {
  const i18n = useI18n();

  return computed(() => ({
    station: {
      value: ChangeCanisterTargetType.UpgradeStation,
      title: i18n.t('change_canister.targets.upgradestation'),
    },
    upgrader: {
      value: ChangeCanisterTargetType.UpgradeUpgrader,
      title: i18n.t('change_canister.targets.upgradeupgrader'),
    },
  }));
};

export const useDefaultUpgradeFormValue = (): ChangeCanisterFormValue => ({
  target: undefined,
  wasmModule: undefined,
  wasmInitArg: undefined,
  comment: undefined,
});

export const useDefaultUpgradeModel = () => ({
  modelValue: useDefaultUpgradeFormValue(),
  valid: false,
});
