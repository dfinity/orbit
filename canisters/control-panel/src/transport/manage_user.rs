use super::{UserDTO, UserIdentityDTO, UserWalletDTO};
use candid::{CandidType, Deserialize, Principal};

/// The input to manage an user.
#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ManageUserInput {
    /// The name to give the user.
    pub name: Option<String>,
    /// The main wallet to use for the user.
    pub main_wallet: Option<Principal>,
    /// The operation to perform on the wallets.
    pub wallets: Option<Vec<UserWalletDTO>>,
    /// The identities to associate with the user.
    pub identities: Option<Vec<UserIdentityDTO>>,
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
    pub name: Option<String>,
    pub wallet: RegisterUserWalletInput,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RegisterUserResponse {
    pub user: UserDTO,
}
