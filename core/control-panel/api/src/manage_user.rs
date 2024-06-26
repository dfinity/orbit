use super::{UserDTO, UserStationDTO};
use candid::{CandidType, Deserialize};

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DeleteUserResponse {
    pub user: UserDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RegisterUserInput {
    pub station: Option<UserStationDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct RegisterUserResponse {
    pub user: UserDTO,
}
