use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum DefaultWalletInit {
    InitSharedWalletCanister,
    SpecifiedWalletCanister(Principal),
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CanisterInit {
    pub default_wallet: DefaultWalletInit,
}

impl Default for CanisterInit {
    fn default() -> Self {
        Self {
            default_wallet: DefaultWalletInit::InitSharedWalletCanister,
        }
    }
}
