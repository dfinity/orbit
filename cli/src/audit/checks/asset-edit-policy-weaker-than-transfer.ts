import { Finding } from '../report';
import { minVotesForRule } from '../resolver';
import { NamedRule, RequestPolicy, User, UUID } from '../types';

/**
 * Warns when the *easiest path* to passing an `EditAsset` request requires
 * fewer approvals than the *strictest* `Transfer` policy in the station.
 *
 * Background: an approved transfer's destination ledger is resolved live from
 * `asset.metadata["ledger_canister_id"]` at execute time. A successful
 * `EditAsset` between approval and execution can therefore redirect funds. If
 * `EditAsset` is gated more loosely than `Transfer`, an attacker who can pass
 * the lower bar can subvert the higher one.
 *
 * Per-asset / per-account scoping would be more precise; the MVP compares the
 * easiest EditAsset path against the strictest Transfer path across the entire
 * station, which catches the typical misconfiguration without requiring an
 * account-to-asset cross-reference.
 */
export const assetEditPolicyWeakerThanTransfer = (
  policies: RequestPolicy[],
  users: User[],
  namedRules: NamedRule[],
): Finding[] => {
  const namedByUUID = new Map<UUID, NamedRule>(namedRules.map(r => [r.id, r]));
  const editPolicies = policies.filter(p => 'EditAsset' in p.specifier);
  const transferPolicies = policies.filter(p => 'Transfer' in p.specifier);

  if (editPolicies.length === 0 || transferPolicies.length === 0) return [];

  const editVotes = editPolicies.map(p => minVotesForRule(p.rule, users, namedByUUID));
  const transferVotes = transferPolicies.map(p => minVotesForRule(p.rule, users, namedByUUID));

  const easiestEdit = Math.min(...editVotes);
  const strictestTransfer = Math.max(...transferVotes);

  if (!Number.isFinite(easiestEdit) || !Number.isFinite(strictestTransfer)) return [];
  if (easiestEdit >= strictestTransfer) return [];

  return [
    {
      checkId: 'asset.edit-policy-weaker-than-transfer',
      severity: 'warning',
      location: `station-wide (${editPolicies.length} EditAsset policy/policies vs ${transferPolicies.length} Transfer policy/policies)`,
      message: `Easiest EditAsset path requires at least ${easiestEdit} approval(s); strictest Transfer path requires at least ${strictestTransfer} (estimates — AllOf combinators are reduced via max-of-children, giving a lower bound). An actor who can pass the EditAsset bar can mutate asset.metadata["ledger_canister_id"] and redirect an approved Transfer between approval and execution.`,
      fix: 'Gate EditAsset at the same approval level as Transfer for the affected assets. Until v0.2 supports per-asset scoping, treat this as a station-wide signal to tighten the loosest EditAsset policy.',
    },
  ];
};
