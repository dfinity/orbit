import { UserStatus } from '~/generated/station/station.did';
import { UserStatusType } from '~/types/station.types';
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
