import { UserStatus } from '~/generated/wallet/wallet.did';
import { UserStatusType } from '~/types/wallet.types';
import { unreachable, variantIs } from '~/utils/helper.utils';

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
