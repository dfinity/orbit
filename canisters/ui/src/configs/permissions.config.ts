import { variantIs } from '~/core';
import { AccessPolicy, ResourceSpecifier } from '~/generated/wallet/wallet.did';
import { ResourceActionEnum, ResourceTypeEnum } from '~/types/permissions.types';
import { isCommonActionSpecifierContained } from '~/utils/permissions.utils';

export interface ResourcePermissions {
  resourceType: ResourceTypeEnum;
  specifiers: { action: ResourceActionEnum; specifier: ResourceSpecifier }[];
  match(specifier: ResourceSpecifier, policy: AccessPolicy): boolean;
}

export const resourcePermissions: ResourcePermissions[] = [
  {
    resourceType: ResourceTypeEnum.User,
    specifiers: [
      { action: ResourceActionEnum.List, specifier: { User: { List: null } } },
      { action: ResourceActionEnum.Create, specifier: { User: { Create: null } } },
      { action: ResourceActionEnum.Read, specifier: { User: { Read: { Any: null } } } },
      { action: ResourceActionEnum.Update, specifier: { User: { Update: { Any: null } } } },
      { action: ResourceActionEnum.Delete, specifier: { User: { Delete: { Any: null } } } },
    ],
    match(specifier: ResourceSpecifier, policy: AccessPolicy): boolean {
      if (variantIs(specifier, 'User') && variantIs(policy.resource, 'User')) {
        return isCommonActionSpecifierContained(specifier.User, policy.resource.User);
      }

      return false;
    },
  },
  {
    resourceType: ResourceTypeEnum.UserGroup,
    specifiers: [
      { action: ResourceActionEnum.List, specifier: { UserGroup: { List: null } } },
      { action: ResourceActionEnum.Create, specifier: { UserGroup: { Create: null } } },
      { action: ResourceActionEnum.Read, specifier: { UserGroup: { Read: { Any: null } } } },
      { action: ResourceActionEnum.Update, specifier: { UserGroup: { Update: { Any: null } } } },
      { action: ResourceActionEnum.Delete, specifier: { UserGroup: { Delete: { Any: null } } } },
    ],
    match(specifier: ResourceSpecifier, policy: AccessPolicy): boolean {
      if (variantIs(specifier, 'UserGroup') && variantIs(policy.resource, 'UserGroup')) {
        return isCommonActionSpecifierContained(specifier.UserGroup, policy.resource.UserGroup);
      }

      return false;
    },
  },
  {
    resourceType: ResourceTypeEnum.Account,
    specifiers: [
      { action: ResourceActionEnum.List, specifier: { Account: { List: null } } },
      { action: ResourceActionEnum.Create, specifier: { Account: { Create: null } } },
      { action: ResourceActionEnum.Read, specifier: { Account: { Read: { Any: null } } } },
      { action: ResourceActionEnum.Update, specifier: { Account: { Update: { Any: null } } } },
      { action: ResourceActionEnum.Delete, specifier: { Account: { Delete: { Any: null } } } },
    ],
    match(specifier: ResourceSpecifier, policy: AccessPolicy): boolean {
      if (variantIs(specifier, 'Account') && variantIs(policy.resource, 'Account')) {
        return isCommonActionSpecifierContained(specifier.Account, policy.resource.Account);
      }

      return false;
    },
  },
];
