import { NamedRule, RequestPolicyRule, User, UserSpecifier, UUID } from './types';

/**
 * The set of users that *could* approve a rule, mirroring the station's
 * `find_matching_users` semantics in `core/station/impl/src/models/request_policy_rule.rs`.
 * Only the request-agnostic `UserSpecifier` variants (`Any`, `Id`, `Group`) are
 * resolvable statically without a concrete request to evaluate against.
 */
export const resolveApprovers = (specifier: UserSpecifier, users: User[]): User[] => {
  const active = users.filter(u => 'Active' in u.status);
  if ('Any' in specifier) return active;
  if ('Id' in specifier) {
    const wanted = new Set(specifier.Id);
    return active.filter(u => wanted.has(u.id));
  }
  if ('Group' in specifier) {
    const wanted = new Set(specifier.Group);
    return active.filter(u => u.groups.some(g => wanted.has(g.id)));
  }
  return [];
};

/**
 * Walks the rule graph, calling `visit` on every Quorum / QuorumPercentage
 * encountered. Resolves `NamedRule` references on the way; cycles are detected
 * via a visited set so a self-referential rule cannot loop the walker.
 *
 * `path` accumulates the human-readable breadcrumb to the rule for reporting.
 */
export const walkQuorumRules = (
  rule: RequestPolicyRule,
  namedRules: Map<UUID, NamedRule>,
  visit: (
    kind: 'Quorum' | 'QuorumPercentage',
    approvers: UserSpecifier,
    minApproved: number,
    path: string,
  ) => void,
  path: string = '',
  visited: Set<UUID> = new Set(),
): void => {
  if ('AutoApproved' in rule || 'AllowListed' in rule || 'AllowListedByMetadata' in rule) {
    return;
  }
  if ('Quorum' in rule) {
    visit('Quorum', rule.Quorum.approvers, rule.Quorum.min_approved, path || 'Quorum');
    return;
  }
  if ('QuorumPercentage' in rule) {
    visit(
      'QuorumPercentage',
      rule.QuorumPercentage.approvers,
      rule.QuorumPercentage.min_approved,
      path || 'QuorumPercentage',
    );
    return;
  }
  if ('AnyOf' in rule) {
    rule.AnyOf.forEach((child, idx) =>
      walkQuorumRules(child, namedRules, visit, joinPath(path, `AnyOf[${idx}]`), visited),
    );
    return;
  }
  if ('AllOf' in rule) {
    rule.AllOf.forEach((child, idx) =>
      walkQuorumRules(child, namedRules, visit, joinPath(path, `AllOf[${idx}]`), visited),
    );
    return;
  }
  if ('Not' in rule) {
    walkQuorumRules(rule.Not, namedRules, visit, joinPath(path, 'Not'), visited);
    return;
  }
  if ('NamedRule' in rule) {
    const id = rule.NamedRule;
    if (visited.has(id)) return;
    const named = namedRules.get(id);
    if (!named) return;
    walkQuorumRules(
      named.rule,
      namedRules,
      visit,
      joinPath(path, `NamedRule("${named.name}")`),
      new Set([...visited, id]),
    );
  }
};

const joinPath = (parent: string, segment: string): string =>
  parent ? `${parent} → ${segment}` : segment;

/**
 * Returns the minimum number of approval votes required to satisfy a rule, given
 * the current set of active users. Mirrors the station evaluator's clamp:
 * `min(min_approved, total_possible_approvers)`.
 *
 * Combinators reduce as:
 *   AnyOf  → min over children (easiest path)
 *   AllOf  → max over children (lower-bound; semantically each child needs its own quorum)
 *   Not    → opaque, treated as Infinity (cannot reason statically about negation)
 *
 * `AutoApproved` and `AllowListed*` return 0 — rules that bypass vote counting.
 * Unresolvable cases (cycle, missing NamedRule) return Infinity to avoid false positives.
 */
export const minVotesForRule = (
  rule: RequestPolicyRule,
  users: User[],
  namedRules: Map<UUID, NamedRule>,
  visited: Set<UUID> = new Set(),
): number => {
  if ('AutoApproved' in rule) return 0;
  if ('AllowListed' in rule || 'AllowListedByMetadata' in rule) return 0;
  if ('Quorum' in rule) {
    const resolved = resolveApprovers(rule.Quorum.approvers, users);
    return Math.min(rule.Quorum.min_approved, resolved.length);
  }
  if ('QuorumPercentage' in rule) {
    const resolved = resolveApprovers(rule.QuorumPercentage.approvers, users);
    const scaled = rule.QuorumPercentage.min_approved * resolved.length;
    return scaled === 0 ? 0 : Math.ceil(scaled / 100);
  }
  if ('AnyOf' in rule) {
    return rule.AnyOf.reduce(
      (acc, child) => Math.min(acc, minVotesForRule(child, users, namedRules, visited)),
      Number.POSITIVE_INFINITY,
    );
  }
  if ('AllOf' in rule) {
    return rule.AllOf.reduce(
      (acc, child) => Math.max(acc, minVotesForRule(child, users, namedRules, visited)),
      0,
    );
  }
  if ('Not' in rule) return Number.POSITIVE_INFINITY;
  if ('NamedRule' in rule) {
    if (visited.has(rule.NamedRule)) return Number.POSITIVE_INFINITY;
    const named = namedRules.get(rule.NamedRule);
    if (!named) return Number.POSITIVE_INFINITY;
    return minVotesForRule(named.rule, users, namedRules, new Set([...visited, rule.NamedRule]));
  }
  return Number.POSITIVE_INFINITY;
};
