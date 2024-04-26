use super::{UserDTO, UserStationDTO};
use candid::{CandidType, Deserialize, Principal};

/// The input to manage an user.
#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ManageUserInput {
    /// The main station to use for the user.
    pub main_station: Option<Principal>,
    /// Set stations to use for the user.
    pub stations: Option<Vec<UserStationDTO>>,
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
pub struct RegisterUserStationSharedInput {
    pub is_main: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RegisterUserInput {
    pub station_id: Option<Principal>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RegisterUserResponse {
    pub user: UserDTO,
}
