import { AuthScope, Resource } from '~/generated/wallet/wallet.did';
import { AuthScopeEnum, ResourceTypeEnum } from '~/types/access-policies.types';
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

  if (variantIs(resource, 'AccessPolicy')) {
    return ResourceTypeEnum.AccessPolicy;
  }

  if (variantIs(resource, 'ProposalPolicy')) {
    return ResourceTypeEnum.ProposalPolicy;
  }

  if (variantIs(resource, 'ChangeCanister')) {
    return ResourceTypeEnum.ChangeCanister;
  }

  if (variantIs(resource, 'System')) {
    return ResourceTypeEnum.System;
  }

  if (variantIs(resource, 'Proposal')) {
    return ResourceTypeEnum.Proposal;
  }

  if (variantIs(resource, 'AddressBook')) {
    return ResourceTypeEnum.AddressBook;
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
