import { describe, expect, it } from 'vitest';
import { endOfDay, startOfDay, convertDate } from './date.utils';

describe('Date utils', () => {
  describe('startOfDay', () => {
    it('moves the time to the beginning of the day', () => {
      const date = new Date('2020-01-01T12:00:00Z');

      expect(startOfDay(date).toISOString()).toEqual('2020-01-01T00:00:00.000Z');
    });
  });

  describe('endOfDay', () => {
    it('moves the time to the end of the day', () => {
      const date = new Date('2020-01-01T12:00:00Z');

      expect(endOfDay(date).toISOString()).toEqual('2020-01-01T23:59:59.999Z');
    });
  });

  describe('convertDate', () => {
    it('returns undefined for undefined date', () => {
      expect(convertDate(undefined)).toBeUndefined();
    });

    it('converts the date to start of day in UTC', () => {
      const testDate = new Date('2024-01-26T15:30:00Z');
      const convertedDate = convertDate(testDate, { time: 'start-of-day' });
      expect(convertedDate.toISOString()).toBe('2024-01-26T00:00:00.000Z');
    });

    it('converts the date to end of day in UTC', () => {
      const testDate = new Date('2024-01-26T15:30:00Z');
      const convertedDate = convertDate(testDate, { time: 'end-of-day' });
      expect(convertedDate.toISOString()).toBe('2024-01-26T23:59:59.999Z');
    });

    it('keeps the time and converts to local timezone', () => {
      const testDate = new Date('2024-01-26T15:30:00Z');
      const convertedDate = convertDate(testDate, { tz: 'local' });

      const convertedDateString = convertedDate.toISOString();
      const expectedDateString = testDate.toISOString();

      expect(convertedDateString).toBe(expectedDateString);
    });
  });
});
