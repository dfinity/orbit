import { variantIs } from '~/core';
import { CommonActionSpecifier, CommonSpecifier } from '~/generated/wallet/wallet.did';

/**
 * Checks if `a` is contained in `b`
 *
 * @param a Specifier to check if it is contained in b
 * @param b Specifier to check if it contains a
 * @returns true if a is contained in b
 */
export const isCommonSpecifierContained = (a: CommonSpecifier, b: CommonSpecifier) => {
  if (variantIs(a, 'Id') && variantIs(b, 'Id')) {
    return a.Id.every(b.Id.includes);
  }

  if (variantIs(a, 'Group') && variantIs(b, 'Group')) {
    return a.Group.every(b.Group.includes);
  }

  return variantIs(a, 'Any') && variantIs(b, 'Any');
};

/**
 * Checks if `a` is contained in `b`
 *
 * @param a Specifier to check if it is contained in b
 * @param b Specifier to check if it contains a
 * @returns true if a is contained in b
 */
export const isCommonActionSpecifierContained = (
  a: CommonActionSpecifier,
  b: CommonActionSpecifier,
) => {
  if (variantIs(a, 'List') && variantIs(b, 'List')) {
    return true;
  }

  if (variantIs(a, 'Create') && variantIs(b, 'Create')) {
    return true;
  }

  if (variantIs(a, 'Read') && variantIs(b, 'Read')) {
    return isCommonSpecifierContained(a.Read, b.Read);
  }

  if (variantIs(a, 'Update') && variantIs(b, 'Update')) {
    return isCommonSpecifierContained(a.Update, b.Update);
  }

  if (variantIs(a, 'Delete') && variantIs(b, 'Delete')) {
    return isCommonSpecifierContained(a.Delete, b.Delete);
  }

  return false;
};
