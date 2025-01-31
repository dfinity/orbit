import { AuthScope, Resource } from '~/generated/station/station.did';
import { AuthScopeEnum, ResourceTypeEnum } from '~/types/permissions.types';
import { unreachable, variantIs } from '~/utils/helper.utils';

export const fromResourceToResourceEnum = (resource: Resource): ResourceTypeEnum => {
  if (variantIs(resource, 'User')) {
    return ResourceTypeEnum.User;
  }

  if (variantIs(resource, 'UserGroup')) {
    return ResourceTypeEnum.UserGroup;
  }

  if (variantIs(resource, 'Account')) {
    return ResourceTypeEnum.Account;
  }

  if (variantIs(resource, 'Permission')) {
    return ResourceTypeEnum.Permission;
  }

  if (variantIs(resource, 'RequestPolicy')) {
    return ResourceTypeEnum.RequestPolicy;
  }

  if (variantIs(resource, 'System')) {
    return ResourceTypeEnum.System;
  }

  if (variantIs(resource, 'Request')) {
    return ResourceTypeEnum.Request;
  }

  if (variantIs(resource, 'AddressBook')) {
    return ResourceTypeEnum.AddressBook;
  }

  if (variantIs(resource, 'ExternalCanister')) {
    return ResourceTypeEnum.ExternalCanister;
  }

  if (variantIs(resource, 'Notification')) {
    return ResourceTypeEnum.Notification;
  }

  if (variantIs(resource, 'Asset')) {
    return ResourceTypeEnum.Asset;
  }

  return unreachable(resource);
};

export const toAuthScope = (authScope: AuthScopeEnum): AuthScope => {
  switch (authScope) {
    case AuthScopeEnum.Public:
      return { Public: null };
    case AuthScopeEnum.Authenticated:
      return { Authenticated: null };
    case AuthScopeEnum.Restrictred:
      return { Restricted: null };
    default:
      return unreachable(authScope);
  }
};

export const toAuthScopeEnum = (authScope: AuthScope): AuthScopeEnum => {
  if (variantIs(authScope, 'Public')) {
    return AuthScopeEnum.Public;
  }

  if (variantIs(authScope, 'Authenticated')) {
    return AuthScopeEnum.Authenticated;
  }

  if (variantIs(authScope, 'Restricted')) {
    return AuthScopeEnum.Restrictred;
  }

  return unreachable(authScope);
};

/**
 * Creates a text representation of a resource action for display purposes.
 *
 * e.g. For the resource object { User: { List: null } }, the text representation would be "user_list".
 *
 * @param resourceAction The resource action to convert to text.
 * @returns The text representation of the resource action.
 */
export const fromResourceToDisplayText = (resource: Resource, separator = '_'): string => {
  const keys = Object.keys(resource);
  if (keys.length === 0) {
    return '';
  }

  let displayText = '';

  for (const key of keys) {
    displayText += displayText.length ? separator + key : key;
    const resourceKey = key as keyof Resource;

    // also check if resource is an object and not null
    if (
      typeof resource[resourceKey] === 'object' &&
      resource[resourceKey] !== null &&
      !Array.isArray(resource[resourceKey])
    ) {
      const actionValue = fromResourceToDisplayText(resource[resourceKey]);

      if (actionValue?.length) {
        displayText += separator + actionValue;
      }
    }
  }

  return displayText.toLowerCase();
};
