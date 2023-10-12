use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum DefaultBankInit {
    InitSharedBankCanister,
    SpecifiedBankCanister(Principal),
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CanisterInit {
    pub default_bank: DefaultBankInit,
}

impl Default for CanisterInit {
    fn default() -> Self {
        Self {
            default_bank: DefaultBankInit::InitSharedBankCanister,
        }
    }
}
