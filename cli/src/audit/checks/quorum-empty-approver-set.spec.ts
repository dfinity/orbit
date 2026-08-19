import { describe, expect, it, beforeEach } from 'vitest';
import { quorumEmptyApproverSet } from './quorum-empty-approver-set';
import { makeGroup, makeNamedRule, makePolicy, makeUser, resetCounter } from './fixtures';

describe('quorum.empty-approver-set', () => {
  beforeEach(() => resetCounter());

  it('does not fire when the approver set has active users', () => {
    const alice = makeUser({ id: 'u-alice', name: 'Alice' });
    const policy = makePolicy(
      { Transfer: { Any: null } },
      { Quorum: { approvers: { Id: ['u-alice'] }, min_approved: 1 } },
    );
    const findings = quorumEmptyApproverSet([policy], [alice], []);
    expect(findings).toHaveLength(0);
  });

  it('fires when a named user is inactive', () => {
    const carol = makeUser({
      id: 'u-carol',
      name: 'Carol',
      status: { Inactive: null },
    });
    const policy = makePolicy(
      { Transfer: { Any: null } },
      { Quorum: { approvers: { Id: ['u-carol'] }, min_approved: 1 } },
    );
    const findings = quorumEmptyApproverSet([policy], [carol], []);
    expect(findings).toHaveLength(1);
    expect(findings[0].severity).toBe('blocker');
    expect(findings[0].checkId).toBe('quorum.empty-approver-set');
    expect(findings[0].message).toMatch(/0 active users/);
  });

  it('fires when a group has zero members', () => {
    const emptyGroup = makeGroup({ id: 'g-finance', name: 'Finance' });
    const someoneElse = makeUser({ id: 'u-other', name: 'Other' });
    // No users are in g-finance.
    const policy = makePolicy(
      { Transfer: { Any: null } },
      { Quorum: { approvers: { Group: ['g-finance'] }, min_approved: 2 } },
    );
    const findings = quorumEmptyApproverSet([policy], [someoneElse], []);
    expect(findings).toHaveLength(1);
    expect(findings[0].severity).toBe('blocker');
    void emptyGroup;
  });

  it('does not fire on UserSpecifier::Any when active users exist', () => {
    const alice = makeUser({ id: 'u-alice' });
    const policy = makePolicy(
      { Transfer: { Any: null } },
      { Quorum: { approvers: { Any: null }, min_approved: 1 } },
    );
    const findings = quorumEmptyApproverSet([policy], [alice], []);
    expect(findings).toHaveLength(0);
  });

  it('does not fire when min_approved is 0 (rule asks for nothing)', () => {
    const policy = makePolicy(
      { Transfer: { Any: null } },
      { Quorum: { approvers: { Id: ['u-missing'] }, min_approved: 0 } },
    );
    const findings = quorumEmptyApproverSet([policy], [], []);
    expect(findings).toHaveLength(0);
  });

  it('descends into AnyOf to flag a buried empty-approver rule', () => {
    const carol = makeUser({
      id: 'u-carol',
      status: { Inactive: null },
    });
    const policy = makePolicy(
      { Transfer: { Any: null } },
      {
        AnyOf: [
          { Quorum: { approvers: { Id: ['u-carol'] }, min_approved: 1 } },
          { AutoApproved: null },
        ],
      },
    );
    const findings = quorumEmptyApproverSet([policy], [carol], []);
    expect(findings).toHaveLength(1);
    expect(findings[0].location).toMatch(/AnyOf\[0\]/);
  });

  it('resolves NamedRule references', () => {
    const carol = makeUser({
      id: 'u-carol',
      status: { Inactive: null },
    });
    const namedRule = makeNamedRule({
      id: 'nr-admin',
      name: 'Admin approval',
      rule: { Quorum: { approvers: { Id: ['u-carol'] }, min_approved: 1 } },
    });
    const policy = makePolicy({ Transfer: { Any: null } }, { NamedRule: 'nr-admin' });
    const findings = quorumEmptyApproverSet([policy], [carol], [namedRule]);
    expect(findings).toHaveLength(1);
    expect(findings[0].location).toMatch(/Admin approval/);
  });

  it('renders QuorumPercentage findings as a percentage, not a vote count', () => {
    const carol = makeUser({ id: 'u-carol', status: { Inactive: null } });
    const policy = makePolicy(
      { Transfer: { Any: null } },
      { QuorumPercentage: { approvers: { Id: ['u-carol'] }, min_approved: 50 } },
    );
    const findings = quorumEmptyApproverSet([policy], [carol], []);
    expect(findings).toHaveLength(1);
    expect(findings[0].message).toMatch(/50% of approvers/);
    expect(findings[0].message).not.toMatch(/50 approval/);
  });

  it('does not loop infinitely on cyclic NamedRule references', () => {
    const ruleA = makeNamedRule({
      id: 'nr-a',
      name: 'A',
      rule: { NamedRule: 'nr-b' },
    });
    const ruleB = makeNamedRule({
      id: 'nr-b',
      name: 'B',
      rule: { NamedRule: 'nr-a' },
    });
    const policy = makePolicy({ AddUser: null }, { NamedRule: 'nr-a' });
    const findings = quorumEmptyApproverSet([policy], [], [ruleA, ruleB]);
    expect(findings).toHaveLength(0);
  });
});
