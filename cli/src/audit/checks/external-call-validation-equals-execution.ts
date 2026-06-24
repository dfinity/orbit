import { Finding } from '../report';
import { CallExternalCanisterResourceTarget, Permission, RequestPolicy } from '../types';

/**
 * Flags `CallExternalCanister` resources whose `validation_method` and
 * `execution_method` resolve to the same canister + method pair.
 *
 * Background: when a `CallExternalCanister` request is submitted, the station
 * invokes the configured `validation_method` immediately to render/validate the
 * argument blob — before the approval policy completes. If that method is also
 * the `execution_method`, its side effect runs at submission time, bypassing
 * the approval gate. There is no benign reason for the pair to match.
 *
 * Sweeps both surfaces: request-policy specifiers and permission resources.
 */
export const externalCallValidationEqualsExecution = (
  policies: RequestPolicy[],
  permissions: Permission[],
): Finding[] => {
  const findings: Finding[] = [];

  for (const policy of policies) {
    if ('CallExternalCanister' in policy.specifier) {
      const conflict = pairConflict(policy.specifier.CallExternalCanister);
      if (conflict) {
        findings.push({
          checkId: 'external-call.validation-equals-execution',
          severity: 'blocker',
          location: `policy ${policy.id}`,
          message: `CallExternalCanister policy has validation_method == execution_method (${conflict}). Validation hook runs before approval; if the method has side effects, the approval gate is bypassed.`,
          fix: 'Remove the policy, or configure the validation hook as a separate read-only method.',
        });
      }
    }
  }

  for (const permission of permissions) {
    if (
      'ExternalCanister' in permission.resource &&
      'Call' in permission.resource.ExternalCanister
    ) {
      const conflict = pairConflict(permission.resource.ExternalCanister.Call);
      if (conflict) {
        findings.push({
          checkId: 'external-call.validation-equals-execution',
          severity: 'blocker',
          location: `permission on CallExternalCanister`,
          message: `Permission grants a CallExternalCanister resource with validation_method == execution_method (${conflict}). Validation hook runs before approval; if the method has side effects, the approval gate is bypassed.`,
          fix: 'Remove or restructure the permission so the validation hook is a separate read-only method.',
        });
      }
    }
  }

  return findings;
};

const pairConflict = (target: CallExternalCanisterResourceTarget): string | null => {
  if (!('ValidationMethod' in target.validation_method)) return null;
  if (!('ExecutionMethod' in target.execution_method)) return null;
  const v = target.validation_method.ValidationMethod;
  const e = target.execution_method.ExecutionMethod;
  if (v.canister_id === e.canister_id && v.method_name === e.method_name) {
    return `${v.canister_id}::${v.method_name}`;
  }
  return null;
};
