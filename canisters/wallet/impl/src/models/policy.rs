use candid::{CandidType, Deserialize};
use ic_canister_macros::stable_object;

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PolicyStatus {
    Pending,
    Failed,
    Fulfilled,
}

/// Represents a policy within the system.
///
/// Policies are used to define the rules of operating within the wallet, including approval thresholds for
/// operations and others.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Policy {
    ApprovalThreshold(ApprovalThresholdPolicy),
}

/// Represents an approval threshold policy.
///
/// This policy defines the number of approvals required for operations to be executed.
/// It can be either a fixed number or percentage, based on the number of owners.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ApprovalThresholdPolicy {
    VariableThreshold(u8),
    FixedThreshold(u8),
}
