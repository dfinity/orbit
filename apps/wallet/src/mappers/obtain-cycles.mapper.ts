import { CycleObtainStrategyInput } from '~/generated/station/station.did';
import { CycleObtainStrategyEnum } from '~/types/obtain-cycles.types';
import { unreachable, variantIs } from '~/utils/helper.utils';

export function cycleObtainStrategyInputToKey(
  strategy: CycleObtainStrategyInput,
): CycleObtainStrategyEnum {
  if (variantIs(strategy, 'MintFromNativeToken')) {
    return CycleObtainStrategyEnum.MintFromNativeToken;
  }
  if (variantIs(strategy, 'Disabled')) {
    return CycleObtainStrategyEnum.Disabled;
  } else {
    return unreachable(strategy);
  }
}

export function cycleObtainStrategyToSummary(strategy: CycleObtainStrategyInput): string {
  if (variantIs(strategy, 'Disabled')) {
    return 'Disabled';
  } else if (variantIs(strategy, 'MintFromNativeToken')) {
    return `Mint from ICP account "${strategy.MintFromNativeToken.account_id}"`;
  } else {
    return unreachable(strategy);
  }
}
