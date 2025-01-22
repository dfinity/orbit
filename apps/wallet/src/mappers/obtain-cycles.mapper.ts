import { CycleObtainStrategyInput } from '~/generated/station/station.did';
import { CycleObtainStrategyEnum } from '~/types/obtain-cycles.types';
import { unreachable, variantIs } from '~/utils/helper.utils';
import { i18n } from '~/plugins/i18n.plugin';

export function cycleObtainStrategyInputToKey(
  strategy: CycleObtainStrategyInput,
): CycleObtainStrategyEnum {
  if (variantIs(strategy, 'MintFromNativeToken')) {
    return CycleObtainStrategyEnum.MintFromNativeToken;
  }
  if (variantIs(strategy, 'WithdrawFromCyclesLedger')) {
    return CycleObtainStrategyEnum.WithdrawFromCyclesLedger;
  }
  if (variantIs(strategy, 'Disabled')) {
    return CycleObtainStrategyEnum.Disabled;
  } else {
    return unreachable(strategy);
  }
}

export function cycleObtainStrategyToSummary(strategy: CycleObtainStrategyInput): string {
  if (variantIs(strategy, 'Disabled')) {
    return i18n.global.t('cycle_obtain_strategies.disabled');
  } else if (variantIs(strategy, 'MintFromNativeToken')) {
    return `${i18n.global.t('cycle_obtain_strategies.mintfromnativetoken')} "${strategy.MintFromNativeToken.account_id}"`;
  } else if (variantIs(strategy, 'WithdrawFromCyclesLedger')) {
    return `${i18n.global.t('cycle_obtain_strategies.withdrawfromcyclesledger')} "${strategy.WithdrawFromCyclesLedger.account_id}"`;
  } else {
    return unreachable(strategy);
  }
}
