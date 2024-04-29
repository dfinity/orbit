use crate::UserSubscriptionStatusDTO;
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct DeployStationInput {
    pub name: String,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct UserStationDTO {
    pub canister_id: Principal,
    pub name: String,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ListStationsResponse {
    pub stations: Vec<UserStationDTO>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GetMainStationResponse {
    pub station: Option<UserStationDTO>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DeployStationResponse {
    pub canister_id: Principal,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum CanDeployStationResponse {
    NotAllowed(UserSubscriptionStatusDTO),
    Allowed(usize),
    QuotaExceeded,
}
