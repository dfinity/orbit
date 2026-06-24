import { describe, expect, it, beforeEach } from 'vitest';
import { assetEditPolicyWeakerThanTransfer } from './asset-edit-policy-weaker-than-transfer';
import { makePolicy, makeUser, resetCounter } from './fixtures';

describe('asset.edit-policy-weaker-than-transfer', () => {
  beforeEach(() => resetCounter());

  const fiveActiveUsers = () => Array.from({ length: 5 }, (_, i) => makeUser({ id: `u-${i}` }));

  it('does not fire when there are no EditAsset policies', () => {
    const users = fiveActiveUsers();
    const policy = makePolicy(
      { Transfer: { Any: null } },
      { Quorum: { approvers: { Any: null }, min_approved: 2 } },
    );
    const findings = assetEditPolicyWeakerThanTransfer([policy], users, []);
    expect(findings).toHaveLength(0);
  });

  it('does not fire when there are no Transfer policies', () => {
    const users = fiveActiveUsers();
    const policy = makePolicy(
      { EditAsset: { Any: null } },
      { Quorum: { approvers: { Any: null }, min_approved: 2 } },
    );
    const findings = assetEditPolicyWeakerThanTransfer([policy], users, []);
    expect(findings).toHaveLength(0);
  });

  it('does not fire when EditAsset is gated as strict as the strictest Transfer', () => {
    const users = fiveActiveUsers();
    const editPolicy = makePolicy(
      { EditAsset: { Any: null } },
      { Quorum: { approvers: { Any: null }, min_approved: 3 } },
    );
    const transferPolicy = makePolicy(
      { Transfer: { Any: null } },
      { Quorum: { approvers: { Any: null }, min_approved: 3 } },
    );
    const findings = assetEditPolicyWeakerThanTransfer([editPolicy, transferPolicy], users, []);
    expect(findings).toHaveLength(0);
  });

  it('fires when EditAsset is gated more loosely than Transfer', () => {
    const users = fiveActiveUsers();
    const editPolicy = makePolicy(
      { EditAsset: { Any: null } },
      { Quorum: { approvers: { Any: null }, min_approved: 1 } },
    );
    const transferPolicy = makePolicy(
      { Transfer: { Any: null } },
      { Quorum: { approvers: { Any: null }, min_approved: 3 } },
    );
    const findings = assetEditPolicyWeakerThanTransfer([editPolicy, transferPolicy], users, []);
    expect(findings).toHaveLength(1);
    expect(findings[0].severity).toBe('warning');
    expect(findings[0].message).toMatch(/Easiest EditAsset path requires 1/);
    expect(findings[0].message).toMatch(/strictest Transfer path requires 3/);
  });

  it('uses minVotes including AutoApproved (treated as 0)', () => {
    const users = fiveActiveUsers();
    const editPolicy = makePolicy({ EditAsset: { Any: null } }, { AutoApproved: null });
    const transferPolicy = makePolicy(
      { Transfer: { Any: null } },
      { Quorum: { approvers: { Any: null }, min_approved: 2 } },
    );
    const findings = assetEditPolicyWeakerThanTransfer([editPolicy, transferPolicy], users, []);
    expect(findings).toHaveLength(1);
    expect(findings[0].message).toMatch(/Easiest EditAsset path requires 0/);
  });
});
