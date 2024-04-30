import { describe, expect, it } from 'vitest';
import { validEmail } from './form.utils';

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
});
