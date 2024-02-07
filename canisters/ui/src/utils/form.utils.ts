import { Principal } from '@dfinity/principal';
import isUUID from 'validator/es/lib/isUUID';
import { i18n } from '~/modules/i18n.module';

export const requiredRule = (value: unknown): string | boolean => {
  if (value === null || value === undefined || value === '') {
    return i18n.global.t('forms.rules.required');
  }

  if (typeof value === 'string' && value.trim() === '') {
    return i18n.global.t('forms.rules.required');
  }

  if (Array.isArray(value) && value.length === 0) {
    return i18n.global.t('forms.rules.required');
  }

  return true;
};

export const maxLengthRule = (max: number, field: string) => {
  return (value: unknown): string | boolean => {
    const hasValue = !!value;
    if (!hasValue) {
      // this rule only applies if there is a value
      return true;
    }

    if (typeof value !== 'string') {
      throw new Error('maxLengthRule only applies to strings');
    }

    return value.length <= max ? true : i18n.global.t('forms.rules.maxLength', { field, max });
  };
};

export const uniqueRule = (
  existing: unknown[],
  errorMessage: string = i18n.global.t('forms.rules.duplicate'),
) => {
  return (value: unknown): string | boolean => {
    const hasValue = !!value;
    if (!hasValue) {
      // this rule only applies if there is a value
      return true;
    }

    return !existing.includes(value) ? true : errorMessage;
  };
};

export const validPrincipalRule = (value: unknown): string | boolean => {
  const hasValue = !!value;
  if (!hasValue) {
    // this rule only applies if there is a value
    return true;
  }

  try {
    // parsing the principal will throw if it is invalid
    Principal.fromText(value as string);
    return true;
  } catch (e) {
    return i18n.global.t('forms.rules.validPrincipal');
  }
};

export const validCanisterId = (value: unknown): string | boolean => {
  const hasValue = !!value;
  if (!hasValue) {
    // this rule only applies if there is a value
    return true;
  }

  if (typeof value !== 'string') {
    return i18n.global.t('forms.rules.validCanisterId');
  }

  if (!/^[a-zA-Z0-9]{5}-[a-zA-Z0-9]{5}-[a-zA-Z0-9]{5}-[a-zA-Z0-9]{5}-[a-zA-Z0-9]{3}$/.test(value)) {
    return i18n.global.t('forms.rules.validCanisterId');
  }

  try {
    // parsing the principal will throw if it is invalid
    Principal.fromText(value as string);
    return true;
  } catch (e) {
    return i18n.global.t('forms.rules.validCanisterId');
  }
};

export const validUuidV4Rule = (value: unknown): string | boolean => {
  const hasValue = !!value;
  if (!hasValue) {
    // this rule only applies if there is a value
    return true;
  }

  try {
    // parsing the principal will throw if it is invalid
    if (isUUID(value as string, 4)) {
      return true;
    } else {
      return i18n.global.t('forms.rules.validUuidV4');
    }
  } catch (e) {
    return i18n.global.t('forms.rules.validUuidV4');
  }
};

export const validTokenAmount = (value: unknown, decimals: number): string | boolean => {
  const hasValue = !!value;
  if (!hasValue) {
    // this rule only applies if there is a value
    return true;
  }

  try {
    if (typeof value !== 'string') {
      throw new Error('validTokenAmount only applies to strings');
    }

    if (!value.includes('.')) {
      // if there is no decimal point, we assume the number is an integer
      if (BigInt(`${value}`) < 0) {
        throw new Error('Invalid format, amount must be greater than 0');
      }

      return true;
    }

    if (value.split('.').length !== 2) {
      throw new Error('Invalid format, amounts can only have one decimal point');
    }

    const [integer, decimal] = value.split('.');

    if (decimal.trim().length > decimals) {
      throw new Error(`Invalid format, amounts can only have ${decimals} decimals`);
    }

    if (BigInt(`${integer}${decimal}`) < 0) {
      throw new Error('Invalid format, amount must be greater than 0');
    }

    return true;
  } catch (e) {
    return i18n.global.t('forms.rules.validTokenAmount');
  }
};
