import { TimeUnit } from '~/types/app.types';
import { unreachable } from '~/utils/helper.utils';

const YEAR_IN_SECONDS = BigInt(60 * 60 * 24 * 365);
const MONTH_IN_SECONDS = BigInt(60 * 60 * 24 * 31); // fixed on 31 for simplicity
const DAY_IN_SECONDS = BigInt(60 * 60 * 24);
const HOUR_IN_SECONDS = BigInt(60 * 60);
const MINUTE_IN_SECONDS = BigInt(60);

/**
 * Converts seconds to a human-readable format based on the unit provided.
 *
 * Important: This function may lose precision when converting from seconds to a larger
 * unit (e.g. from seconds to years).
 */
export const toTimeUnit = (seconds: bigint, unit: TimeUnit): number => {
  if (seconds === BigInt(0)) {
    return 0;
  }

  switch (unit) {
    case TimeUnit.Years: {
      return Number(seconds) / Number(YEAR_IN_SECONDS);
    }
    case TimeUnit.Months: {
      return Number(seconds) / Number(MONTH_IN_SECONDS);
    }
    case TimeUnit.Days: {
      return Number(seconds) / Number(DAY_IN_SECONDS);
    }
    case TimeUnit.Hours: {
      return Number(seconds) / Number(HOUR_IN_SECONDS);
    }
    case TimeUnit.Minutes: {
      return Number(seconds) / Number(MINUTE_IN_SECONDS);
    }
    case TimeUnit.Seconds:
      return Number(seconds);
    default:
      return unreachable(unit);
  }
};

/**
 * Converts time in a specific unit to seconds based on the unit provided, if the provided unit has decimals,
 * the seconds amount will include the decimals.
 *
 * Important: This function may lose precision when converting from a floating number when using the smallest unit.
 */
export const fromTimeUnit = (units: number, unit: TimeUnit): bigint => {
  switch (unit) {
    case TimeUnit.Years: {
      return BigInt(units * Number(YEAR_IN_SECONDS));
    }
    case TimeUnit.Months: {
      return BigInt(units * Number(MONTH_IN_SECONDS));
    }
    case TimeUnit.Days: {
      return BigInt(units * Number(DAY_IN_SECONDS));
    }
    case TimeUnit.Hours: {
      return BigInt(units * Number(HOUR_IN_SECONDS));
    }
    case TimeUnit.Minutes: {
      return BigInt(units * Number(MINUTE_IN_SECONDS));
    }
    case TimeUnit.Seconds:
      return BigInt(Math.floor(units));
    default:
      return unreachable(unit);
  }
};
