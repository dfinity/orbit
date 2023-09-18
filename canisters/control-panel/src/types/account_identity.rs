use super::AccountIdentityStatus;
use candid::{CandidType, Deserialize, Principal};

/// The identity of an account.
#[derive(CandidType, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AccountIdentity {
    /// The principal ID of the identity.
    id: Principal,
    /// The name of the identity (if any).
    name: Option<String>,
    /// The status of the identity.
    status: AccountIdentityStatus,
}

impl Default for AccountIdentity {
    fn default() -> Self {
        Self {
            id: Principal::anonymous(),
            name: None,
            status: AccountIdentityStatus::PendingActivation,
        }
    }
}
