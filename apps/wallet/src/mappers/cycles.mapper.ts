import { CyclesUnit } from '~/types/app.types';
import { unreachable } from '~/utils/helper.utils';

const TRILLION = BigInt(1_000_000_000_000);
const BILLION = BigInt(1_000_000_000);
const MILLION = BigInt(1_000_000);
const THOUSAND = BigInt(1_000);

/**
 * Converts cycles to a human readable format based on the unit provided and the number of decimals.
 *
 * Important: This function may lose precision when converting from cycles to a larger
 * unit (e.g. from cycles to trillions).
 */
export const toCyclesUnit = (cycles: bigint, unit: CyclesUnit): number => {
  if (cycles === BigInt(0)) {
    return 0;
  }

  switch (unit) {
    case CyclesUnit.Trillion: {
      const remainder = cycles % TRILLION;
      const cyclesInTrillion = cycles / TRILLION;
      const cyclesInBillion = remainder / BILLION;

      return Number(cyclesInTrillion) + Number(cyclesInBillion) / 1_000;
    }
    case CyclesUnit.Billion: {
      const remainder = cycles % BILLION;
      const cyclesInBillion = cycles / BILLION;
      const cyclesInMillion = remainder / MILLION;

      return Number(cyclesInBillion) + Number(cyclesInMillion) / 1_000;
    }
    case CyclesUnit.Million: {
      const remainder = cycles % MILLION;
      const cyclesInMillion = cycles / MILLION;
      const cyclesInThousand = remainder / THOUSAND;

      return Number(cyclesInMillion) + Number(cyclesInThousand) / 1_000;
    }
    case CyclesUnit.Smallest:
      return Number(cycles);
    default:
      return unreachable(unit);
  }
};

/**
 * Converts cycles to a bigint based on the unit provided, if the provided unit has decimals,
 * the cycles amount will include the decimals.
 *
 * Important: This function may lose precision when converting from a larger unit to cycles
 * or a floating number when using the smallest unit.
 */
export const fromCyclesUnit = (cycles: number, unit: CyclesUnit): bigint => {
  switch (unit) {
    case CyclesUnit.Trillion: {
      const cyclesInTrillion = Math.floor(cycles);
      const cyclesInBillion = Math.floor((cycles - cyclesInTrillion) * 1_000);

      return BigInt(cyclesInTrillion) * TRILLION + BigInt(cyclesInBillion) * BILLION;
    }
    case CyclesUnit.Billion: {
      const cyclesInBillion = Math.floor(cycles);
      const cyclesInMillion = Math.floor((cycles - cyclesInBillion) * 1_000);

      return BigInt(cyclesInBillion) * BILLION + BigInt(cyclesInMillion) * MILLION;
    }
    case CyclesUnit.Million: {
      const cyclesInMillion = Math.floor(cycles);
      const cyclesInThousand = Math.floor((cycles - cyclesInMillion) * 1_000);

      return BigInt(cyclesInMillion) * MILLION + BigInt(cyclesInThousand) * THOUSAND;
    }
    case CyclesUnit.Smallest: {
      return BigInt(Math.floor(cycles));
    }
    default:
      return unreachable(unit);
  }
};

export const cyclesUnitFromNumber = (cycles: bigint): CyclesUnit => {
  if (cycles >= 1_000_000_000_000) {
    return CyclesUnit.Trillion;
  }
  if (cycles >= 1_000_000_000) {
    return CyclesUnit.Billion;
  }
  if (cycles >= 1_000_000) {
    return CyclesUnit.Million;
  }
  return CyclesUnit.Smallest;
};

export const formatCycles = (cycles: bigint): string => {
  const unit = cyclesUnitFromNumber(cycles);
  return `${toCyclesUnit(cycles, unit).toFixed(3)} ${unit}`;
};
