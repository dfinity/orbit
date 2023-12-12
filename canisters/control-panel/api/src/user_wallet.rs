use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct UserWalletDTO {
    pub canister_id: Principal,
    pub name: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ListWalletsResponse {
    pub wallets: Vec<UserWalletDTO>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GetMainWalletResponse {
    pub wallet: Option<UserWalletDTO>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DeployWalletResponse {
    pub canister_id: Principal,
}
