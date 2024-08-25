import { computed, ComputedRef } from 'vue';
import { useI18n } from 'vue-i18n';
import { SystemUpgradeFormValue } from '~/components/system-upgrade/system-upgrade.types';
import { SystemUpgradeTargetType } from '~/types/station.types';

export const useUpgradeTargets = (): ComputedRef<{
  station: { value: SystemUpgradeTargetType; title: string };
  upgrader: { value: SystemUpgradeTargetType; title: string };
}> => {
  const i18n = useI18n();

  return computed(() => ({
    station: {
      value: SystemUpgradeTargetType.UpgradeStation,
      title: i18n.t('system_upgrade.targets.upgradestation'),
    },
    upgrader: {
      value: SystemUpgradeTargetType.UpgradeUpgrader,
      title: i18n.t('system_upgrade.targets.upgradeupgrader'),
    },
  }));
};

export const useDefaultUpgradeFormValue = (): SystemUpgradeFormValue => ({
  target: undefined,
  wasmModule: undefined,
  wasmInitArg: undefined,
  comment: undefined,
});

export const useDefaultUpgradeModel = () => ({
  modelValue: useDefaultUpgradeFormValue(),
  valid: false,
});
