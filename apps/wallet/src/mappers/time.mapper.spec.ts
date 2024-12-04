import { describe, expect, it } from 'vitest';
import { TimeUnit } from '~/types/app.types';
import { fromTimeUnit, toTimeUnit } from '~/mappers/time.mapper.ts';

describe('toTimeUnit', () => {
  it('should convert seconds to years', () => {
    expect(toTimeUnit(BigInt(31_536_000), TimeUnit.Years)).toBeCloseTo(1, 3);
  });

  it('should convert seconds to months', () => {
    expect(toTimeUnit(BigInt(2_678_400), TimeUnit.Months)).toBeCloseTo(1, 3);
  });

  it('should convert seconds to days', () => {
    expect(toTimeUnit(BigInt(86_400), TimeUnit.Days)).toBeCloseTo(1, 3);
  });

  it('should convert seconds to hours', () => {
    expect(toTimeUnit(BigInt(3_600), TimeUnit.Hours)).toBeCloseTo(1, 3);
  });

  it('should convert seconds to minutes', () => {
    expect(toTimeUnit(BigInt(60), TimeUnit.Minutes)).toBeCloseTo(1, 3);
  });

  it('should return the same number of seconds', () => {
    expect(toTimeUnit(BigInt(1), TimeUnit.Seconds)).toBe(1);
  });

  it('should return 0 if seconds is 0', () => {
    expect(toTimeUnit(BigInt(0), TimeUnit.Years)).toBe(0);
    expect(toTimeUnit(BigInt(0), TimeUnit.Months)).toBe(0);
    expect(toTimeUnit(BigInt(0), TimeUnit.Days)).toBe(0);
    expect(toTimeUnit(BigInt(0), TimeUnit.Hours)).toBe(0);
    expect(toTimeUnit(BigInt(0), TimeUnit.Minutes)).toBe(0);
    expect(toTimeUnit(BigInt(0), TimeUnit.Seconds)).toBe(0);
  });

  it('should throw if the unit is not recognized', () => {
    expect(() => toTimeUnit(BigInt(1), 'unknown' as TimeUnit)).toThrow();
  });
});
describe('fromTimeUnit', () => {
  it('should convert years to seconds', () => {
    expect(fromTimeUnit(1, TimeUnit.Years)).toBe(BigInt(31_536_000));
  });

  it('should convert months to seconds', () => {
    expect(fromTimeUnit(1, TimeUnit.Months)).toBe(BigInt(2_678_400));
  });

  it('should convert days to seconds', () => {
    expect(fromTimeUnit(1, TimeUnit.Days)).toBe(BigInt(86_400));
  });

  it('should convert hours to seconds', () => {
    expect(fromTimeUnit(1, TimeUnit.Hours)).toBe(BigInt(3_600));
  });

  it('should convert minutes to seconds', () => {
    expect(fromTimeUnit(1, TimeUnit.Minutes)).toBe(BigInt(60));
  });

  it('should return the same number of seconds', () => {
    expect(fromTimeUnit(1, TimeUnit.Seconds)).toBe(BigInt(1));
  });

  it('should drop decimals if seconds unit is used', () => {
    expect(fromTimeUnit(1.5, TimeUnit.Seconds)).toBe(BigInt(1));
  });

  it('should return 0 if units is 0', () => {
    expect(fromTimeUnit(0, TimeUnit.Years)).toBe(BigInt(0));
    expect(fromTimeUnit(0, TimeUnit.Months)).toBe(BigInt(0));
    expect(fromTimeUnit(0, TimeUnit.Days)).toBe(BigInt(0));
    expect(fromTimeUnit(0, TimeUnit.Hours)).toBe(BigInt(0));
    expect(fromTimeUnit(0, TimeUnit.Minutes)).toBe(BigInt(0));
    expect(fromTimeUnit(0, TimeUnit.Seconds)).toBe(BigInt(0));
  });

  it('should throw if the unit is not recognized', () => {
    expect(() => fromTimeUnit(1, 'unknown' as TimeUnit)).toThrow();
  });
});
