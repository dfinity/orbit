use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct BankCanisterInit {
    pub approval_threshold: Option<u8>,
}

impl Default for BankCanisterInit {
    fn default() -> Self {
        Self {
            // By default, the bank canister requires 100% of the votes to approve operations.
            approval_threshold: Some(100u8),
        }
    }
}
