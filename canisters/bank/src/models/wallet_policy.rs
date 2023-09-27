use candid::{CandidType, Deserialize};
use ic_canister_macros::stable_object;

/// Represents a wallet policy within the system.
///
/// Policies are used to define the rules for a wallet, including approval thresholds for
/// operations and others.
#[stable_object(size = 256)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum WalletPolicy {
    ApprovalThreshold(ApprovalThresholdPolicy),
}

/// Represents an approval threshold policy.
///
/// This policy defines the number of approvals required for operations to be executed.
/// It can be either a fixed number or percentage, based on the number of owners.
#[stable_object(size = 64)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ApprovalThresholdPolicy {
    VariableThreshold(u8),
    FixedThreshold(u8),
}
