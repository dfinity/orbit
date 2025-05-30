import { Principal } from '@dfinity/principal';
import isUUID from 'validator/es/lib/isUUID';
import { i18n } from '~/plugins/i18n.plugin';
import { detectAddressFormat } from './asset.utils';

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

export const isHexRule = (value: unknown): string | true => {
  const hasValue = !!value;
  if (!hasValue) {
    // this rule only applies if there is a value
    return true;
  }

  if (typeof value !== 'string') {
    throw new Error('isHexRule only applies to strings');
  }

  if (!/^([A-Fa-f0-9]{2})+$/.test(value)) {
    return i18n.global.t('forms.rules.isHex');
  }

  return true;
};

export const intNumberRangeRule = (
  field: string,
  min: number,
  max: number = Number.MAX_SAFE_INTEGER,
) => {
  return (value: unknown): string | boolean => {
    const hasValue = !!value;
    if (!hasValue) {
      // this rule only applies if there is a value
      return true;
    }

    const parsedValue = parseInt(`${value}`, 10);

    if (isNaN(parsedValue) || Number(value) % 1 !== 0) {
      return i18n.global.t('forms.rules.requiredIntNumber');
    }

    return parsedValue >= min && parsedValue <= max
      ? true
      : i18n.global.t('forms.rules.intNumberRange', { field, min, max });
  };
};

export const numberRangeRule = (opts: { decimals?: number; min?: number; max?: number } = {}) => {
  return (value: unknown): string | boolean => {
    const hasValue = !!value;
    if (!hasValue) {
      // this rule only applies if there is a value
      return true;
    }

    const min = opts.min ?? Number.MIN_SAFE_INTEGER;
    const max = opts.max ?? Number.MAX_SAFE_INTEGER;
    const allowedDecimalPlaces = opts.decimals ?? 0;

    const parsedValue = opts.decimals ? parseFloat(`${value}`) : parseInt(`${value}`, 10);
    const parsedDecimalsLength = `${parsedValue}`.split('.')[1]?.length ?? 0;

    if (isNaN(parsedValue)) {
      return i18n.global.t('forms.rules.requiredNumber');
    }

    if (allowedDecimalPlaces <= 0 && Number(value) % 1 !== 0) {
      return i18n.global.t('forms.rules.requiredIntNumber');
    }

    if (allowedDecimalPlaces > 0 && parsedDecimalsLength > allowedDecimalPlaces) {
      return i18n.global.t('forms.rules.invalidDecimalPlaces', { decimals: allowedDecimalPlaces });
    }

    return parsedValue >= min && parsedValue <= max
      ? true
      : i18n.global.t('forms.rules.numberRange', { min, max });
  };
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

export const validSymbolRule = (value: unknown): string | boolean => {
  const hasValue = !!value;
  if (!hasValue) {
    // this rule only applies if there is a value
    return true;
  }

  if (typeof value !== 'string') {
    throw new Error('validSymbolRule only applies to strings');
  }

  return /^[a-zA-Z0-9]{1,32}$/.test(value) ? true : i18n.global.t('forms.rules.validSymbol');
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
  } catch (_e) {
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
  } catch (_e) {
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
  } catch (_e) {
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
  } catch (_e) {
    return i18n.global.t('forms.rules.validTokenAmount');
  }
};

export const validEmail = (value: unknown): string | boolean => {
  const hasValue = !!value;
  if (!hasValue) {
    // this rule only applies if there is a value
    return true;
  }

  if (typeof value !== 'string') {
    throw new Error('requiredEmail only applies to strings');
  }

  if (!/^(?!.*\.\.)[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value)) {
    return i18n.global.t('forms.rules.validEmail');
  }

  return true;
};

export const validAddress =
  (blockchain: string) =>
  (value: unknown): string | boolean => {
    const hasValue = !!value;
    if (!hasValue) {
      // this rule only applies if there is a value
      return true;
    }

    if (typeof value !== 'string') {
      return i18n.global.t('forms.rules.validAddress');
    }

    try {
      if (detectAddressFormat(blockchain, value) !== undefined) {
        return true;
      }
      return i18n.global.t('forms.rules.validAddress');
    } catch {
      return i18n.global.t('forms.rules.validAddress');
    }
  };

export function compareMetadata<T extends { key: string; value: string }[]>(
  a: T | undefined,
  b: T,
): boolean {
  // Quick length check
  if (a?.length !== b.length) {
    return false;
  }

  // Sort both arrays by key then value to compare order-insensitively
  const sortEntries = (arr: { key: string; value: string }[]) =>
    [...arr].sort((x, y) => x.key.localeCompare(y.key) || x.value.localeCompare(y.value));

  const sortedA = sortEntries(a);
  const sortedB = sortEntries(b);

  // Compare each entry one-to-one
  return sortedA.every(
    (entry, index) => entry.key === sortedB[index].key && entry.value === sortedB[index].value,
  );
}

export function compareTruthy<T>(a: T | undefined, b: T): boolean {
  if (!a && !b) {
    return true;
  }

  if (!a || !b) {
    return false;
  }

  return JSON.stringify(a) === JSON.stringify(b);
}

export function focusText(el: HTMLTextAreaElement, term: string, offsetLines = 2) {
  const text = el.value;
  const index = text?.indexOf(term);

  if (index !== undefined && index >= 0) {
    setTimeout(() => {
      const lines = text.substr(0, index).split('\n');
      const lineHeight = parseFloat(getComputedStyle(el).lineHeight) || 18;
      const lineNumber = lines.length - 1;
      const scrollPosition = lineNumber * lineHeight;
      el.scrollTop = scrollPosition - lineHeight * offsetLines;
      el.setSelectionRange(index, index + term.length);
    }, 0);
  }
}
