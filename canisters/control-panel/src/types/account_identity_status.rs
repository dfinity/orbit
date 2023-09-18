use candid::{CandidType, Deserialize};
use std::fmt::Display;

/// Represents the possible states of an account identity.
#[derive(CandidType, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum AccountIdentityStatus {
    /// The identity is active within the account.
    Active,
    /// The identity is pending activation, and is waiting for the user with access to the identity to activate it.
    PendingActivation,
}

impl Display for AccountIdentityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountIdentityStatus::Active => write!(f, "active"),
            AccountIdentityStatus::PendingActivation => write!(f, "pending-activation"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_types_match_string_representation() {
        assert_eq!(AccountIdentityStatus::Active.to_string(), "active");
        assert_eq!(
            AccountIdentityStatus::PendingActivation.to_string(),
            "pending-activation"
        );
    }
}
