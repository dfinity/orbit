import { describe, expect, it, beforeEach } from 'vitest';
import { externalCallValidationEqualsExecution } from './external-call-validation-equals-execution';
import { makePermission, makePolicy, method, resetCounter } from './fixtures';

describe('external-call.validation-equals-execution', () => {
  beforeEach(() => resetCounter());

  it('does not fire when validation and execution methods differ', () => {
    const policy = makePolicy(
      {
        CallExternalCanister: {
          validation_method: {
            ValidationMethod: method('canister-1', 'validate_inc'),
          },
          execution_method: { ExecutionMethod: method('canister-1', 'inc') },
        },
      },
      { AutoApproved: null },
    );
    const findings = externalCallValidationEqualsExecution([policy], []);
    expect(findings).toHaveLength(0);
  });

  it('does not fire when validation_method is No (no hook configured)', () => {
    const policy = makePolicy(
      {
        CallExternalCanister: {
          validation_method: { No: null },
          execution_method: { ExecutionMethod: method('canister-1', 'inc') },
        },
      },
      { AutoApproved: null },
    );
    const findings = externalCallValidationEqualsExecution([policy], []);
    expect(findings).toHaveLength(0);
  });

  it('fires on a request policy with matching validation and execution', () => {
    const policy = makePolicy(
      {
        CallExternalCanister: {
          validation_method: {
            ValidationMethod: method('canister-1', 'inc'),
          },
          execution_method: { ExecutionMethod: method('canister-1', 'inc') },
        },
      },
      { AutoApproved: null },
    );
    const findings = externalCallValidationEqualsExecution([policy], []);
    expect(findings).toHaveLength(1);
    expect(findings[0].severity).toBe('blocker');
    expect(findings[0].message).toMatch(/canister-1::inc/);
  });

  it('fires on a permission with matching validation and execution', () => {
    const permission = makePermission({
      ExternalCanister: {
        Call: {
          validation_method: {
            ValidationMethod: method('canister-7', 'transfer'),
          },
          execution_method: {
            ExecutionMethod: method('canister-7', 'transfer'),
          },
        },
      },
    });
    const findings = externalCallValidationEqualsExecution([], [permission]);
    expect(findings).toHaveLength(1);
    expect(findings[0].severity).toBe('blocker');
    expect(findings[0].message).toMatch(/canister-7::transfer/);
  });

  it('does not flag a permission targeting different methods on same canister', () => {
    const permission = makePermission({
      ExternalCanister: {
        Call: {
          validation_method: {
            ValidationMethod: method('canister-1', 'render_args'),
          },
          execution_method: { ExecutionMethod: method('canister-1', 'inc') },
        },
      },
    });
    const findings = externalCallValidationEqualsExecution([], [permission]);
    expect(findings).toHaveLength(0);
  });

  it('does not flag a permission targeting same method on different canisters', () => {
    const permission = makePermission({
      ExternalCanister: {
        Call: {
          validation_method: {
            ValidationMethod: method('canister-A', 'inc'),
          },
          execution_method: { ExecutionMethod: method('canister-B', 'inc') },
        },
      },
    });
    const findings = externalCallValidationEqualsExecution([], [permission]);
    expect(findings).toHaveLength(0);
  });
});
