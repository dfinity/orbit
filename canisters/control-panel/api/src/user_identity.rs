use super::UserDTO;
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct UserIdentityDTO {
    pub identity: Principal,
    pub name: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AssociateIdentityWithUserInput {
    pub user_id: String,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AssociateIdentityWithUserResponse {
    pub user: UserDTO,
}
