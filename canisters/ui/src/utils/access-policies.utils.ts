import {
  AccessPolicyResourceAction,
  AccountResourceAction,
  ChangeCanisterResourceAction,
  ProposalResourceAction,
  ResourceAction,
  ResourceId,
  ResourceTypeId,
  SettingsResourceAction,
  UserResourceAction,
} from '~/generated/wallet/wallet.did';
import { variantIs } from '~/utils/helper.utils';

/**
 * Checks if `a` is contained in `b`
 *
 * @param a Specifier to check if it is contained in b
 * @param b Specifier to check if it contains a
 * @returns true if a is contained in b
 */
export const isResourceIdContained = (a: ResourceId, b: ResourceId) => {
  if (variantIs(a, 'Id') && variantIs(b, 'Id')) {
    return a.Id === b.Id;
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
export const isResourceActionContained = (a: ResourceAction, b: ResourceAction) => {
  if (variantIs(a, 'List') && variantIs(b, 'List')) {
    return true;
  }

  if (variantIs(a, 'Create') && variantIs(b, 'Create')) {
    return true;
  }

  if (variantIs(a, 'Read') && variantIs(b, 'Read')) {
    return isResourceIdContained(a.Read, b.Read);
  }

  if (variantIs(a, 'Update') && variantIs(b, 'Update')) {
    return isResourceIdContained(a.Update, b.Update);
  }

  if (variantIs(a, 'Delete') && variantIs(b, 'Delete')) {
    return isResourceIdContained(a.Delete, b.Delete);
  }

  return false;
};

export const isSettingsResourceActionContained = (
  a: SettingsResourceAction,
  b: SettingsResourceAction,
) => {
  if (variantIs(a, 'Read') && variantIs(b, 'Read')) {
    return true;
  }

  if (variantIs(a, 'ReadConfig') && variantIs(b, 'ReadConfig')) {
    return true;
  }

  return false;
};

export const isChangeCanisterResourceActionContained = (
  a: ChangeCanisterResourceAction,
  b: ChangeCanisterResourceAction,
) => {
  if (variantIs(a, 'Create') && variantIs(b, 'Create')) {
    return true;
  }

  return false;
};

export const isProposalResourceActionContained = (
  a: ProposalResourceAction,
  b: ProposalResourceAction,
) => {
  if (variantIs(a, 'List') && variantIs(b, 'List')) {
    return true;
  }

  if (variantIs(a, 'Read') && variantIs(b, 'Read')) {
    return isResourceIdContained(a.Read, b.Read);
  }

  return false;
};

export const isUserResourceActionContained = (a: UserResourceAction, b: UserResourceAction) => {
  if (variantIs(a, 'List') && variantIs(b, 'List')) {
    return true;
  }

  if (variantIs(a, 'Create') && variantIs(b, 'Create')) {
    return true;
  }

  if (variantIs(a, 'Read') && variantIs(b, 'Read')) {
    return isResourceIdContained(a.Read, b.Read);
  }

  if (variantIs(a, 'Update') && variantIs(b, 'Update')) {
    return isResourceIdContained(a.Update, b.Update);
  }

  return false;
};

export const isAccountResourceActionContained = (
  a: AccountResourceAction,
  b: AccountResourceAction,
) => {
  if (variantIs(a, 'List') && variantIs(b, 'List')) {
    return true;
  }

  if (variantIs(a, 'Read') && variantIs(b, 'Read')) {
    return isResourceIdContained(a.Read, b.Read);
  }

  if (variantIs(a, 'Update') && variantIs(b, 'Update')) {
    return isResourceIdContained(a.Update, b.Update);
  }

  if (variantIs(a, 'Transfer') && variantIs(b, 'Transfer')) {
    return isResourceIdContained(a.Transfer, b.Transfer);
  }

  return false;
};

export const isAccessPolicyResourceActionContained = (
  a: AccessPolicyResourceAction,
  b: AccessPolicyResourceAction,
) => {
  if (variantIs(a, 'List') && variantIs(b, 'List')) {
    return true;
  }

  if (variantIs(a, 'Edit') && variantIs(b, 'Edit')) {
    return isResourceTypeIdContained(a.Edit, b.Edit);
  }

  if (variantIs(a, 'Read') && variantIs(b, 'Read')) {
    return isResourceTypeIdContained(a.Read, b.Read);
  }

  return false;
};

export const isResourceTypeIdContained = (a: ResourceTypeId, b: ResourceTypeId) => {
  if (variantIs(a, 'Any') && variantIs(b, 'Any')) {
    return true;
  }

  if (variantIs(a, 'Resource') && variantIs(b, 'Resource')) {
    return a.Resource === b.Resource;
  }

  return false;
};
