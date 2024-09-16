import { describe, expect, it } from 'vitest';
import { intNumberRangeRule, numberRangeRule, validEmail } from './form.utils';

describe('FormUtils', () => {
  describe('validateEmail', () => {
    it.each([
      'john.doe@example.com',
      'jane.smith@workplace.net',
      'info@internet.org',
      'first.last@sub.domain.co',
      'contact@company.io',
      'user123@service.info',
      'üser@test.com',
      'my.email@multi-part.domain.com',
      'temp@international.co.uk',
      'special_char$@unique-domain.app',
      'number1@domainname.us',
    ])(`should return true for valid email %s`, email => {
      expect(validEmail(email)).toBe(true);
    });

    it.each([
      'plainaddress',
      'someone@web',
      'missingatsign.com',
      'missingusername.com',
      'invalid@domain..com',
      'email@domain space.com',
      'username@.com',
    ])(`should return error for invalid email %s`, email => {
      expect(validEmail(email)).not.toBe(true);
    });
  });

  describe('intNumberRangeRule', () => {
    it.each([
      { from: 0, to: 10, value: '5' },
      { from: -10, to: 10, value: '0' },
      { from: 0, to: 100, value: '100' },
      { from: -100, to: 100, value: '-100' },
      { from: 0, to: 100, value: '100' },
    ])(`$value is between $from and $to`, ({ from, to, value }) => {
      expect(intNumberRangeRule('number', from, to)(value)).toBe(true);
    });

    it.each([
      { from: 0, to: 4, value: '5' },
      { from: -10, to: -5, value: '0' },
    ])(`$value is not between $from and $to`, ({ from, to, value }) => {
      expect(intNumberRangeRule('number', from, to)(value)).not.toBe(true);
    });

    it.each(['string', '1.234'])(`%s is not a valid integer`, value => {
      expect(intNumberRangeRule('number', 0, 10)(value)).not.toBe(true);
    });
  });
});

describe('numberRangeRule', () => {
  it.each([
    { from: 0, to: 10, decimals: 0, value: '5' },
    { from: -10, to: 10, decimals: 0, value: '0' },
    { from: 0, to: 100, decimals: 2, value: '99.95' },
    { from: -100, to: 100, value: '-100' },
  ])(`$value is between $from and $to`, ({ from, to, decimals, value }) => {
    expect(
      numberRangeRule({
        min: from,
        max: to,
        decimals,
      })(value),
    ).toBe(true);
  });

  it.each([
    { from: 0, to: 4, decimals: 0, value: '5' },
    { from: -10, to: -5, decimals: 0, value: '0' },
    { from: 0, to: 100, decimals: 2, value: '100.01' },
  ])(`$value is not between $from and $to`, ({ from, to, decimals, value }) => {
    expect(
      numberRangeRule({
        min: from,
        max: to,
        decimals,
      })(value),
    ).not.toBe(true);
  });

  it.each(['string', 'a1.234'])(`%s is not a valid number`, value => {
    expect(numberRangeRule({ decimals: 10 })(value)).not.toBe(true);
  });
});
