import { Finding } from '../report';
import { resolveApprovers, walkQuorumRules } from '../resolver';
import { NamedRule, RequestPolicy, User, UUID } from '../types';
import { describeSpecifier, describeRequestSpecifier } from './describe';

/**
 * Flags Quorum / QuorumPercentage rules whose approver set resolves to zero
 * active users today.
 *
 * Background: `RequestApprovalSummary::evaluate` clamps `min_approved` down to
 * `total_possible_approvers`. When the approver set is empty the clamp drives
 * the threshold to 0, and `approved (0) >= min_approved (0)` evaluates to
 * Approved without any votes cast. The next matching request auto-approves.
 */
export const quorumEmptyApproverSet = (
  policies: RequestPolicy[],
  users: User[],
  namedRules: NamedRule[],
): Finding[] => {
  const findings: Finding[] = [];
  const namedByUUID = new Map<UUID, NamedRule>(namedRules.map(r => [r.id, r]));

  for (const policy of policies) {
    walkQuorumRules(policy.rule, namedByUUID, (kind, approvers, minApproved, path) => {
      const resolved = resolveApprovers(approvers, users);
      if (resolved.length === 0 && minApproved > 0) {
        findings.push({
          checkId: 'quorum.empty-approver-set',
          severity: 'blocker',
          location: `policy ${policy.id} (${describeRequestSpecifier(policy.specifier)}) — ${path}`,
          message: `${kind} rule asks for ${minApproved} approval(s) but ${describeSpecifier(approvers)} currently resolves to 0 active users. Next matching request will auto-approve.`,
          fix: 'Add eligible approvers to this specifier, or wrap in AnyOf with an admin-group fallback before the next matching request is submitted.',
        });
      }
    });
  }

  return findings;
};
