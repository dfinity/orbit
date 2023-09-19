use super::AccountIdentity;
use candid::{CandidType, Deserialize};
use uuid::Uuid;

/// A UUID that identifies an account within the system.
pub type AccountID = String;

/// The identity of an account.
#[derive(CandidType, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Account {
    /// The UUID that identifies the account.
    id: AccountID,
    /// The name of the account (if any).
    name: Option<String>,
    /// The status of the identity.
    main_bank: Option<Vec<u8>>,
    /// The identifies associated with the account.
    identities: Vec<AccountIdentity>,
}

impl Default for Account {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: None,
            main_bank: None,
            identities: Vec::new(),
        }
    }
}
