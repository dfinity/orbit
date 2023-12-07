use super::{UserDTO, UserWalletDTO};
use candid::{CandidType, Deserialize, Principal};

/// The input to manage an user.
#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ManageUserInput {
    /// The main wallet to use for the user.
    pub main_wallet: Option<Principal>,
    /// The operation to perform on the wallets.
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
pub enum RegisterUserWalletInput {
    PrivateWallet {
        id: Principal,
        use_shared_wallet: Option<RegisterUserWalletSharedInput>,
    },
    SharedWallet,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RegisterUserInput {
    pub wallet: RegisterUserWalletInput,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RegisterUserResponse {
    pub user: UserDTO,
}
