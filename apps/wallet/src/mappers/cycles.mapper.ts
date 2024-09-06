import { CyclesUnit } from '~/types/app.types';
import { unreachable } from '~/utils/helper.utils';

export const toCyclesUnit = (cycles: bigint, unit: CyclesUnit): bigint => {
  if (cycles === BigInt(0)) {
    return BigInt(0);
  }

  switch (unit) {
    case CyclesUnit.Trillion:
      return cycles / BigInt(1_000_000_000_000);
    case CyclesUnit.Billion:
      return cycles / BigInt(1_000_000_000);
    case CyclesUnit.Million:
      return cycles / BigInt(1_000_000);
    case CyclesUnit.Thousand:
      return cycles / BigInt(1_000);
    case CyclesUnit.Smallest:
      return cycles;
    default:
      return unreachable(unit);
  }
};

export const fromCyclesUnit = (cycles: bigint, unit: CyclesUnit): bigint => {
  switch (unit) {
    case CyclesUnit.Trillion:
      return cycles * BigInt(1_000_000_000_000);
    case CyclesUnit.Billion:
      return cycles * BigInt(1_000_000_000);
    case CyclesUnit.Million:
      return cycles * BigInt(1_000_000);
    case CyclesUnit.Thousand:
      return cycles * BigInt(1_000);
    case CyclesUnit.Smallest:
      return cycles;
    default:
      return unreachable(unit);
  }
};
