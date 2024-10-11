import { CanisterConfiguredMethodCall } from '~/components/external-canisters/external-canisters.types';
import {
  ExternalCanisterPermissions,
  ExternalCanisterRequestPolicies,
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

export const mapConfiguredMethodCalls = (opts: {
  requestPolicies: ExternalCanisterRequestPolicies['calls'];
  permissions: ExternalCanisterPermissions['calls'];
}) => {
  const configuredMethodCalls: Map<string, CanisterConfiguredMethodCall> = new Map();

  const getOrDefault = (
    methodName: string,
    validationTarget: ValidationMethodResourceTarget,
  ): CanisterConfiguredMethodCall =>
    configuredMethodCalls.get(
      mapMethodCallConfigurationToKey({
        executionMethod: methodName,
        validationMethod: validationTarget,
      }),
    ) ?? {
      methodName,
      requestPolicies: [],
      permission: undefined,
      validationTarget,
    };

  for (const policy of opts.requestPolicies) {
    const methodCallEntry = getOrDefault(policy.execution_method, policy.validation_method);

    methodCallEntry.requestPolicies.push({
      rule: policy.rule,
      policy_id: [policy.policy_id],
    });

    configuredMethodCalls.set(
      mapMethodCallConfigurationToKey({
        executionMethod: policy.execution_method,
        validationMethod: policy.validation_method,
      }),
      methodCallEntry,
    );
  }

  for (const permission of opts.permissions) {
    const methodCallEntry = getOrDefault(permission.execution_method, permission.validation_method);

    methodCallEntry.permission = permission.allow;

    configuredMethodCalls.set(
      mapMethodCallConfigurationToKey({
        executionMethod: permission.execution_method,
        validationMethod: permission.validation_method,
      }),
      methodCallEntry,
    );
  }

  return Array.from(configuredMethodCalls.values()).sort((a, b) => {
    if (
      a.methodName === b.methodName &&
      variantIs(a.validationTarget, 'ValidationMethod') &&
      variantIs(b.validationTarget, 'ValidationMethod')
    ) {
      return a.validationTarget.ValidationMethod.method_name.localeCompare(
        b.validationTarget.ValidationMethod.method_name,
      );
    }

    return a.methodName.localeCompare(b.methodName);
  });
};
