import { User, UserStatus } from '~/generated/wallet/wallet.did';
import { UserDTO, UserStatusType } from '~/types/wallet.types';
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

export const toPartialUserDTO = (user: Partial<User>): Partial<UserDTO> => {
  const dto: Partial<UserDTO> = {};
  if (user.id) {
    dto.id = user.id;
  }
  if (user.name) {
    dto.name = user.name;
  }
  if (user.groups) {
    dto.groups = user.groups?.map(group => group.id);
  }
  if (user.identities) {
    dto.identities = user.identities.map(identity => identity.toText());
  }
  if (user.status) {
    dto.status = user.status;
  }
  if (user.last_modification_timestamp) {
    dto.last_modification_timestamp = user.last_modification_timestamp;
  }

  return dto;
};
