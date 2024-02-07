import { unreachable, variantIs } from '~/core/utils.core';
import { User, UserStatus } from '~/generated/wallet/wallet.did';
import { UserInput, UserStatusType } from '~/types/wallet.types';

export const fromUserToUserInput = (user: Partial<User> = {}): UserInput => {
  return {
    id: user.id,
    name: user.name?.[0],
    status: user.status ? user.status : { Inactive: null },
    groups: user.groups?.map(g => g.id) ?? [],
    identities: user.identities?.map(i => i.toText()) ?? [],
    prefilledGroups: user.groups ?? [],
  };
};

export const fromUserStatusVariantToEnum = (status: UserStatus): UserStatusType => {
  if (variantIs(status, UserStatusType.Active)) {
    return UserStatusType.Active;
  }

  if (variantIs(status, UserStatusType.Inactive)) {
    return UserStatusType.Inactive;
  }

  return unreachable(status);
};

export const fromUserStatusEnumToVariant = (status: UserStatusType): UserStatus => {
  switch (status) {
    case UserStatusType.Active:
      return { Active: null };
    case UserStatusType.Inactive:
      return { Inactive: null };
  }

  unreachable(status);
};
