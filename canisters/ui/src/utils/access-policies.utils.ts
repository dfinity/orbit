import { variantIs } from '~/core/utils.core';
import {
  CanisterSettingsActionSpecifier,
  ChangeCanisterActionSpecifier,
  CommonActionSpecifier,
  CommonSpecifier,
  ProposalActionSpecifier,
  TransferActionSpecifier,
} from '~/generated/wallet/wallet.did';

/**
 * Checks if `a` is contained in `b`
 *
 * @param a Specifier to check if it is contained in b
 * @param b Specifier to check if it contains a
 * @returns true if a is contained in b
 */
export const isCommonSpecifierContained = (a: CommonSpecifier, b: CommonSpecifier) => {
  if (variantIs(a, 'Id') && variantIs(b, 'Id')) {
    return a.Id.every(id => b.Id.includes(id));
  }

  if (variantIs(a, 'Group') && variantIs(b, 'Group')) {
    return a.Group.every(id => b.Group.includes(id));
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

export const isCanisterSettingsActionSpecifier = (
  a: CanisterSettingsActionSpecifier,
  b: CanisterSettingsActionSpecifier,
) => {
  if (variantIs(a, 'Read') && variantIs(b, 'Read')) {
    return true;
  }

  if (variantIs(a, 'ReadConfig') && variantIs(b, 'ReadConfig')) {
    return true;
  }

  return false;
};

export const isChangeCanisterActionSpecifier = (
  a: ChangeCanisterActionSpecifier,
  b: ChangeCanisterActionSpecifier,
) => {
  if (variantIs(a, 'Create') && variantIs(b, 'Create')) {
    return true;
  }

  return false;
};

export const isTransferActionSpecifier = (
  a: TransferActionSpecifier,
  b: TransferActionSpecifier,
) => {
  if (variantIs(a, 'Create') && variantIs(b, 'Create')) {
    return isCommonSpecifierContained(a.Create.account, b.Create.account);
  }

  if (variantIs(a, 'Read') && variantIs(b, 'Read')) {
    return isCommonSpecifierContained(a.Read.account, b.Read.account);
  }

  if (variantIs(a, 'Delete') && variantIs(b, 'Delete')) {
    return isCommonSpecifierContained(a.Delete.account, b.Delete.account);
  }

  return false;
};

export const isProposalActionSpecifier = (
  a: ProposalActionSpecifier,
  b: ProposalActionSpecifier,
) => {
  if (variantIs(a, 'List') && variantIs(b, 'List')) {
    return true;
  }

  if (variantIs(a, 'Read') && variantIs(b, 'Read')) {
    return isCommonSpecifierContained(a.Read, b.Read);
  }

  return false;
};
