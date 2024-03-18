import { Resource, UserAuthentication } from '~/generated/wallet/wallet.did';
import { AccessPolicyForAllUsers, ResourceTypeEnum } from '~/types/access-policies.types';
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

  if (variantIs(resource, 'Settings')) {
    return ResourceTypeEnum.Settings;
  }

  if (variantIs(resource, 'Proposal')) {
    return ResourceTypeEnum.Proposal;
  }

  if (variantIs(resource, 'AddressBook')) {
    return ResourceTypeEnum.AddressBook;
  }

  return unreachable(resource);
};

export const toUserAuthentication = (
  everyone: AccessPolicyForAllUsers,
): UserAuthentication | null => {
  if (everyone === AccessPolicyForAllUsers.Public) {
    return { None: null };
  }

  if (everyone === AccessPolicyForAllUsers.AuthenticationRequired) {
    return { Required: null };
  }

  return null;
};

export const fromUserAuthentication = (
  userAuthentication: UserAuthentication | null,
): AccessPolicyForAllUsers => {
  if (userAuthentication === null) {
    return AccessPolicyForAllUsers.NotSet;
  }

  if (variantIs(userAuthentication, 'None')) {
    return AccessPolicyForAllUsers.Public;
  }

  if (variantIs(userAuthentication, 'Required')) {
    return AccessPolicyForAllUsers.AuthenticationRequired;
  }

  return unreachable(userAuthentication);
};
