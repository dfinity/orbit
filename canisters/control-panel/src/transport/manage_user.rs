use super::{UserBankDTO, UserDTO, UserIdentityDTO};
use candid::{CandidType, Deserialize, Principal};

/// The input to manage an user.
#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ManageUserInput {
    /// The name to give the user.
    pub name: Option<String>,
    /// The main bank to use for the user.
    pub main_bank: Option<Principal>,
    /// The operation to perform on the banks.
    pub banks: Option<Vec<UserBankDTO>>,
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
pub struct RegisterUserBankSharedInput {
    pub is_main: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum RegisterUserBankInput {
    PrivateBank {
        id: Principal,
        use_shared_bank: Option<RegisterUserBankSharedInput>,
    },
    SharedBank,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RegisterUserInput {
    pub name: Option<String>,
    pub bank: RegisterUserBankInput,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RegisterUserResponse {
    pub user: UserDTO,
}
