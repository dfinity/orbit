use super::{UserDTO, UserWalletDTO};
use candid::{CandidType, Deserialize, Principal};

/// The input to manage an user.
#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ManageUserInput {
    /// Set e-mail to use for the user.
    pub email: Option<String>,
    /// The main wallet to use for the user.
    pub main_wallet: Option<Principal>,
    /// Set wallets to use for the user.
    pub wallets: Option<Vec<UserWalletDTO>>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ManageUserResponse {
    pub user: UserDTO,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DeleteUserResponse {
    pub user: UserDTO,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RegisterUserWalletSharedInput {
    pub is_main: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RegisterUserInput {
    pub wallet_id: Option<Principal>,
    pub email: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RegisterUserResponse {
    pub user: UserDTO,
}
