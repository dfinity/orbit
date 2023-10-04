import { Principal } from '@dfinity/principal';
import { i18n } from '~/ui/modules';
import isUUID from 'validator//es/lib/isUUID';

export const requiredRule = (value: unknown): string | boolean => {
  if (value === null || value === undefined || value === '') {
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

export const uniqueRule = (existing: unknown[]) => {
  return (value: unknown): string | boolean => {
    const hasValue = !!value;
    if (!hasValue) {
      // this rule only applies if there is a value
      return true;
    }

    return !existing.includes(value) ? true : i18n.global.t('forms.rules.duplicate');
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
