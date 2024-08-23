import {
  PermissionResourceAction,
  AccountResourceAction,
  RequestResourceAction,
  ResourceAction,
  ResourceId,
  SystemResourceAction,
  UserResourceAction,
} from '~/generated/station/station.did';
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

export const isSystemResourceActionContained = (
  a: SystemResourceAction,
  b: SystemResourceAction,
) => {
  if (variantIs(a, 'Capabilities') && variantIs(b, 'Capabilities')) {
    return true;
  }

  if (variantIs(a, 'SystemInfo') && variantIs(b, 'SystemInfo')) {
    return true;
  }

  if (variantIs(a, 'ManageSystemInfo') && variantIs(b, 'ManageSystemInfo')) {
    return true;
  }

  return false;
};

export const isRequestResourceActionContained = (
  a: RequestResourceAction,
  b: RequestResourceAction,
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

  if (variantIs(a, 'Create') && variantIs(b, 'Create')) {
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

export const isPermissionResourceActionContained = (
  a: PermissionResourceAction,
  b: PermissionResourceAction,
) => {
  if (variantIs(a, 'Read') && variantIs(b, 'Read')) {
    return true;
  }

  if (variantIs(a, 'Update') && variantIs(b, 'Update')) {
    return true;
  }

  return false;
};
