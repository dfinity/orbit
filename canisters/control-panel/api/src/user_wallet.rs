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

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum UserAuthorizationStatusDTO {
    Unauthorized,
    Pending,
    Authorized,
    Blacklisted,
}

impl std::fmt::Display for UserAuthorizationStatusDTO {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UserAuthorizationStatusDTO::Unauthorized => write!(f, "unauthorized"),
            UserAuthorizationStatusDTO::Pending => write!(f, "pending"),
            UserAuthorizationStatusDTO::Authorized => write!(f, "authorized"),
            UserAuthorizationStatusDTO::Blacklisted => write!(f, "blacklisted"),
        }
    }
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum CanDeployWalletResponse {
    NotAllowed(UserAuthorizationStatusDTO),
    Allowed(usize),
    QuotaExceeded,
}
