import { describe, expect, it } from 'vitest';
import { fromCyclesUnit, toCyclesUnit } from '~/mappers/cycles.mapper';
import { CyclesUnit } from '~/types/app.types';

describe('toCyclesUnit', () => {
  it('should convert cycles to a number based on the unit provided', () => {
    expect(toCyclesUnit(BigInt(1_000_000_000_000), CyclesUnit.Trillion)).toBe(1);
    expect(toCyclesUnit(BigInt(1_000_000_000), CyclesUnit.Billion)).toBe(1);
    expect(toCyclesUnit(BigInt(1_000_000), CyclesUnit.Million)).toBe(1);
    expect(toCyclesUnit(BigInt(1), CyclesUnit.Smallest)).toBe(1);
  });

  it('should convert cycles to a number based on the unit provided with decimals', () => {
    expect(toCyclesUnit(BigInt(1_502_000_000_000), CyclesUnit.Trillion)).toBe(1.502);
    expect(toCyclesUnit(BigInt(1_500_000_000), CyclesUnit.Billion)).toBe(1.5);
    expect(toCyclesUnit(BigInt(1_500_000), CyclesUnit.Million)).toBe(1.5);
    expect(toCyclesUnit(BigInt(1), CyclesUnit.Smallest)).toBe(1);
  });

  it('should return 0 if cycles is 0', () => {
    expect(toCyclesUnit(BigInt(0), CyclesUnit.Trillion)).toBe(0);
    expect(toCyclesUnit(BigInt(0), CyclesUnit.Billion)).toBe(0);
    expect(toCyclesUnit(BigInt(0), CyclesUnit.Million)).toBe(0);
    expect(toCyclesUnit(BigInt(0), CyclesUnit.Smallest)).toBe(0);
  });

  it('expected to have less precision when converting from e8s to a larger unit', () => {
    expect(toCyclesUnit(BigInt(1_999_999_999_999), CyclesUnit.Trillion)).toBe(1.999);
    expect(toCyclesUnit(BigInt(1_999_999_999_999), CyclesUnit.Billion)).toBe(1999.999);
    expect(toCyclesUnit(BigInt(1_999_999_999_999), CyclesUnit.Million)).toBe(1999999.999);
    expect(toCyclesUnit(BigInt(1_999_999_999_999), CyclesUnit.Smallest)).toBe(1_999_999_999_999);
  });

  it('should throw if the unit is not recognized', () => {
    expect(() => toCyclesUnit(BigInt(1), 'unknown' as CyclesUnit)).toThrow();
  });
});

describe('fromCyclesUnit', () => {
  it('should convert a number to cycles based on the unit provided', () => {
    expect(fromCyclesUnit(1, CyclesUnit.Trillion)).toBe(BigInt(1_000_000_000_000));
    expect(fromCyclesUnit(1, CyclesUnit.Billion)).toBe(BigInt(1_000_000_000));
    expect(fromCyclesUnit(1, CyclesUnit.Million)).toBe(BigInt(1_000_000));
    expect(fromCyclesUnit(1, CyclesUnit.Smallest)).toBe(BigInt(1));
  });

  it('should convert a number to cycles based on the unit provided with decimals', () => {
    expect(fromCyclesUnit(1.502, CyclesUnit.Trillion)).toBe(BigInt(1_502_000_000_000));
    expect(fromCyclesUnit(1.5, CyclesUnit.Billion)).toBe(BigInt(1_500_000_000));
    expect(fromCyclesUnit(1.5, CyclesUnit.Million)).toBe(BigInt(1_500_000));
    expect(fromCyclesUnit(1, CyclesUnit.Smallest)).toBe(BigInt(1));
  });

  it('should return 0 if cycles is 0', () => {
    expect(fromCyclesUnit(0, CyclesUnit.Trillion)).toBe(BigInt(0));
    expect(fromCyclesUnit(0, CyclesUnit.Billion)).toBe(BigInt(0));
    expect(fromCyclesUnit(0, CyclesUnit.Million)).toBe(BigInt(0));
    expect(fromCyclesUnit(0, CyclesUnit.Smallest)).toBe(BigInt(0));
  });

  it('expected to have less precision when converting from a larger unit to cycles', () => {
    expect(fromCyclesUnit(1.999999, CyclesUnit.Trillion)).toBe(BigInt(1_999_000_000_000));
    expect(fromCyclesUnit(1999.999999, CyclesUnit.Billion)).toBe(BigInt(1_999_999_000_000));
    expect(fromCyclesUnit(1999999.999999, CyclesUnit.Million)).toBe(BigInt(1_999_999_999_000));
    expect(fromCyclesUnit(1_999_999_999_999, CyclesUnit.Smallest)).toBe(BigInt(1_999_999_999_999));
  });

  it('should throw if the unit is not recognized', () => {
    expect(() => fromCyclesUnit(1, 'unknown' as CyclesUnit)).toThrow();
  });
});
