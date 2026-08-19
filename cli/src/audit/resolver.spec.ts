import { describe, expect, it, beforeEach } from 'vitest';
import { minVotesForRule, resolveApprovers, walkQuorumRules } from './resolver';
import { makeGroup, makeNamedRule, makeUser, resetCounter } from './checks/fixtures';
import { NamedRule, RequestPolicyRule, User, UUID, UserSpecifier } from './types';

const namedMap = (rules: NamedRule[]): Map<UUID, NamedRule> => new Map(rules.map(r => [r.id, r]));

describe('resolveApprovers', () => {
  beforeEach(() => resetCounter());

  it('Any returns every active user, regardless of group', () => {
    const alice = makeUser({ id: 'u-alice' });
    const bob = makeUser({ id: 'u-bob' });
    const carol = makeUser({ id: 'u-carol', status: { Inactive: null } });
    expect(resolveApprovers({ Any: null }, [alice, bob, carol]).map(u => u.id)).toEqual([
      'u-alice',
      'u-bob',
    ]);
  });

  it('Id returns only the named users (active only)', () => {
    const alice = makeUser({ id: 'u-alice' });
    const bob = makeUser({ id: 'u-bob' });
    const carol = makeUser({ id: 'u-carol', status: { Inactive: null } });
    expect(
      resolveApprovers({ Id: ['u-alice', 'u-carol'] }, [alice, bob, carol]).map(u => u.id),
    ).toEqual(['u-alice']);
  });

  it('Id with no matching users returns empty', () => {
    const alice = makeUser({ id: 'u-alice' });
    expect(resolveApprovers({ Id: ['u-missing'] }, [alice])).toEqual([]);
  });

  it('Group matches active users in any of the named groups', () => {
    const finance = makeGroup({ id: 'g-finance' });
    const ops = makeGroup({ id: 'g-ops' });
    const alice = makeUser({ id: 'u-alice', groups: [finance] });
    const bob = makeUser({ id: 'u-bob', groups: [ops] });
    const carol = makeUser({ id: 'u-carol', groups: [], status: { Inactive: null } });
    expect(
      resolveApprovers({ Group: ['g-finance', 'g-ops'] }, [alice, bob, carol]).map(u => u.id),
    ).toEqual(['u-alice', 'u-bob']);
  });

  it('Group excludes users not in any of the named groups', () => {
    const finance = makeGroup({ id: 'g-finance' });
    const alice = makeUser({ id: 'u-alice', groups: [finance] });
    const bob = makeUser({ id: 'u-bob', groups: [] });
    expect(resolveApprovers({ Group: ['g-finance'] }, [alice, bob]).map(u => u.id)).toEqual([
      'u-alice',
    ]);
  });

  it('Group excludes inactive users even if they are in the group', () => {
    const finance = makeGroup({ id: 'g-finance' });
    const carol = makeUser({
      id: 'u-carol',
      groups: [finance],
      status: { Inactive: null },
    });
    expect(resolveApprovers({ Group: ['g-finance'] }, [carol])).toEqual([]);
  });
});

describe('walkQuorumRules', () => {
  beforeEach(() => resetCounter());

  const collect = (rule: RequestPolicyRule, named: NamedRule[] = []) => {
    const seen: Array<{ kind: string; minApproved: number; path: string }> = [];
    walkQuorumRules(rule, namedMap(named), (kind, _approvers, minApproved, path) => {
      seen.push({ kind, minApproved, path });
    });
    return seen;
  };

  it('visits Quorum with the configured min_approved', () => {
    expect(collect({ Quorum: { approvers: { Any: null }, min_approved: 2 } })).toEqual([
      { kind: 'Quorum', minApproved: 2, path: 'Quorum' },
    ]);
  });

  it('visits QuorumPercentage with the configured percentage', () => {
    expect(collect({ QuorumPercentage: { approvers: { Any: null }, min_approved: 51 } })).toEqual([
      { kind: 'QuorumPercentage', minApproved: 51, path: 'QuorumPercentage' },
    ]);
  });

  it('skips AutoApproved and AllowListed', () => {
    expect(collect({ AutoApproved: null })).toEqual([]);
    expect(collect({ AllowListed: null })).toEqual([]);
  });

  // The path reports the breadcrumb up to but not including the rule itself;
  // the rule kind is delivered separately via the visitor signature.
  it('descends into AnyOf with indexed path breadcrumbs', () => {
    const seen = collect({
      AnyOf: [
        { Quorum: { approvers: { Any: null }, min_approved: 1 } },
        { Quorum: { approvers: { Any: null }, min_approved: 2 } },
      ],
    });
    expect(seen.map(s => s.path)).toEqual(['AnyOf[0]', 'AnyOf[1]']);
  });

  it('descends into AllOf with indexed path breadcrumbs', () => {
    const seen = collect({
      AllOf: [{ Quorum: { approvers: { Any: null }, min_approved: 3 } }],
    });
    expect(seen[0].path).toBe('AllOf[0]');
  });

  it('descends into Not', () => {
    const seen = collect({
      Not: { Quorum: { approvers: { Any: null }, min_approved: 1 } },
    });
    expect(seen[0].path).toBe('Not');
  });

  it('resolves NamedRule and includes its name in the path', () => {
    const admin = makeNamedRule({
      id: 'nr-admin',
      name: 'Admin approval',
      rule: { Quorum: { approvers: { Any: null }, min_approved: 2 } },
    });
    const seen = collect({ NamedRule: 'nr-admin' }, [admin]);
    expect(seen[0].path).toBe('NamedRule("Admin approval")');
  });

  it('joins nested combinators with arrows', () => {
    const seen = collect({
      AnyOf: [
        {
          AllOf: [{ Quorum: { approvers: { Any: null }, min_approved: 1 } }],
        },
      ],
    });
    expect(seen[0].path).toBe('AnyOf[0] → AllOf[0]');
  });

  it('silently skips a missing NamedRule', () => {
    expect(collect({ NamedRule: 'nr-does-not-exist' }, [])).toEqual([]);
  });

  it('breaks cycles in NamedRule references', () => {
    const a = makeNamedRule({ id: 'nr-a', name: 'A', rule: { NamedRule: 'nr-b' } });
    const b = makeNamedRule({ id: 'nr-b', name: 'B', rule: { NamedRule: 'nr-a' } });
    expect(collect({ NamedRule: 'nr-a' }, [a, b])).toEqual([]);
  });
});

describe('minVotesForRule', () => {
  beforeEach(() => resetCounter());

  const noUsers: User[] = [];
  const noNamed = new Map<UUID, NamedRule>();
  const fiveActive = () => Array.from({ length: 5 }, (_, i) => makeUser({ id: `u-${i}` }));

  it('AutoApproved returns 0', () => {
    expect(minVotesForRule({ AutoApproved: null }, noUsers, noNamed)).toBe(0);
  });

  it('AllowListed and AllowListedByMetadata return 0', () => {
    expect(minVotesForRule({ AllowListed: null }, noUsers, noNamed)).toBe(0);
    expect(
      minVotesForRule({ AllowListedByMetadata: { key: 'k', value: 'v' } }, noUsers, noNamed),
    ).toBe(0);
  });

  it('Quorum returns min_approved when enough active approvers exist', () => {
    const users = fiveActive();
    const rule: RequestPolicyRule = { Quorum: { approvers: { Any: null }, min_approved: 3 } };
    expect(minVotesForRule(rule, users, noNamed)).toBe(3);
  });

  it('Quorum clamps to the number of active approvers', () => {
    const users = [makeUser({ id: 'u-1' })];
    const rule: RequestPolicyRule = { Quorum: { approvers: { Any: null }, min_approved: 5 } };
    expect(minVotesForRule(rule, users, noNamed)).toBe(1);
  });

  it('Quorum returns 0 when the approver set is empty', () => {
    const rule: RequestPolicyRule = {
      Quorum: { approvers: { Id: ['u-missing'] }, min_approved: 2 },
    };
    expect(minVotesForRule(rule, noUsers, noNamed)).toBe(0);
  });

  it('QuorumPercentage ceilings the percentage of the approver pool', () => {
    const users = fiveActive();
    // 51% of 5 = 2.55 → ceiling = 3
    const rule: RequestPolicyRule = {
      QuorumPercentage: { approvers: { Any: null }, min_approved: 51 },
    };
    expect(minVotesForRule(rule, users, noNamed)).toBe(3);
  });

  it('QuorumPercentage returns 0 when the approver pool is empty', () => {
    const rule: RequestPolicyRule = {
      QuorumPercentage: { approvers: { Any: null }, min_approved: 100 },
    };
    expect(minVotesForRule(rule, noUsers, noNamed)).toBe(0);
  });

  it('AnyOf returns the minimum over children (easiest path)', () => {
    const users = fiveActive();
    const rule: RequestPolicyRule = {
      AnyOf: [
        { Quorum: { approvers: { Any: null }, min_approved: 3 } },
        { Quorum: { approvers: { Any: null }, min_approved: 1 } },
      ],
    };
    expect(minVotesForRule(rule, users, noNamed)).toBe(1);
  });

  it('AnyOf with no children returns Infinity', () => {
    expect(minVotesForRule({ AnyOf: [] }, noUsers, noNamed)).toBe(Number.POSITIVE_INFINITY);
  });

  it('AllOf returns the maximum over children (lower-bound estimate)', () => {
    const users = fiveActive();
    const rule: RequestPolicyRule = {
      AllOf: [
        { Quorum: { approvers: { Any: null }, min_approved: 2 } },
        { Quorum: { approvers: { Any: null }, min_approved: 4 } },
      ],
    };
    expect(minVotesForRule(rule, users, noNamed)).toBe(4);
  });

  it('AllOf with no children returns 0', () => {
    expect(minVotesForRule({ AllOf: [] }, noUsers, noNamed)).toBe(0);
  });

  it('Not returns Infinity (opaque)', () => {
    const rule: RequestPolicyRule = {
      Not: { Quorum: { approvers: { Any: null }, min_approved: 1 } },
    };
    expect(minVotesForRule(rule, noUsers, noNamed)).toBe(Number.POSITIVE_INFINITY);
  });

  it('NamedRule resolves and recurses into the referenced rule', () => {
    const users = fiveActive();
    const admin = makeNamedRule({
      id: 'nr-admin',
      rule: { Quorum: { approvers: { Any: null }, min_approved: 2 } },
    });
    expect(minVotesForRule({ NamedRule: 'nr-admin' }, users, namedMap([admin]))).toBe(2);
  });

  it('NamedRule pointing at a missing id returns Infinity', () => {
    expect(minVotesForRule({ NamedRule: 'nr-missing' }, noUsers, noNamed)).toBe(
      Number.POSITIVE_INFINITY,
    );
  });

  it('NamedRule cycle returns Infinity, never infinite-loops', () => {
    const a = makeNamedRule({ id: 'nr-a', rule: { NamedRule: 'nr-b' } });
    const b = makeNamedRule({ id: 'nr-b', rule: { NamedRule: 'nr-a' } });
    expect(minVotesForRule({ NamedRule: 'nr-a' }, noUsers, namedMap([a, b]))).toBe(
      Number.POSITIVE_INFINITY,
    );
  });

  it('does not consult the UserSpecifier for AutoApproved / AllowListed', () => {
    // UserSpecifier is unused for these variants; sanity check.
    const _unused: UserSpecifier = { Id: [] };
    expect(minVotesForRule({ AutoApproved: null }, fiveActive(), noNamed)).toBe(0);
    void _unused;
  });
});
