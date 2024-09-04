import { ExternalCanisterState } from '~/generated/station/station.did';
import { ExternalCanisterStateEnum } from '~/types/station.types';
import { unreachable } from '~/utils/helper.utils';

export const mapExternalCanisterStateEnumToVariant = (
  state: ExternalCanisterStateEnum,
): ExternalCanisterState => {
  switch (state) {
    case ExternalCanisterStateEnum.Active:
      return { Active: null };
    case ExternalCanisterStateEnum.Archived:
      return { Archived: null };
  }

  return unreachable(state);
};
