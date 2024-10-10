import {
  ExternalCanisterState,
  ValidationMethodResourceTarget,
} from '~/generated/station/station.did';
import { ExternalCanisterStateEnum } from '~/types/station.types';
import { unreachable, variantIs } from '~/utils/helper.utils';

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

export const mapExternalCanisterStateVariantToEnum = (
  state: ExternalCanisterState,
): ExternalCanisterStateEnum => {
  if ('Active' in state) {
    return ExternalCanisterStateEnum.Active;
  }

  if ('Archived' in state) {
    return ExternalCanisterStateEnum.Archived;
  }

  return unreachable(state);
};

export const mapMethodCallConfigurationToKey = (config: {
  executionMethod: string;
  validationMethod: ValidationMethodResourceTarget;
}): string => {
  let key = `${config.executionMethod}::`;

  if (variantIs(config.validationMethod, 'No')) {
    key += 'no_validation';
  }

  if (variantIs(config.validationMethod, 'ValidationMethod')) {
    key += `${config.validationMethod.ValidationMethod.method_name}::${config.validationMethod.ValidationMethod.canister_id.toText()}`;
  }

  return key;
};
