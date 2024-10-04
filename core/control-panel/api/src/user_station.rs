use crate::UserSubscriptionStatusDTO;
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct DeployStationAdminUserInput {
    pub username: String,
    pub identity: Principal,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct AssociateWithCallerInput {
    pub labels: Vec<String>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct SubnetFilter {
    pub subnet_type: Option<String>,
}

/// Options to select subnets when creating a canister
#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub enum SubnetSelection {
    /// Choose a random subnet that satisfies the specified properties
    Filter(SubnetFilter),
    /// Choose a specific subnet
    Subnet { subnet: Principal },
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct DeployStationInput {
    pub name: String,
    pub admins: Vec<DeployStationAdminUserInput>,
    pub associate_with_caller: Option<AssociateWithCallerInput>,
    pub subnet_selection: Option<SubnetSelection>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct UserStationDTO {
    pub canister_id: Principal,
    pub name: String,
    pub labels: Vec<String>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct UpdateUserStationInput {
    pub index: Option<u64>,
    pub station: UserStationDTO,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub enum ManageUserStationsInput {
    Add(Vec<UserStationDTO>),
    Remove(Vec<Principal>),
    Update(Vec<UpdateUserStationInput>),
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct ListUserStationsInput {
    pub filter_by_labels: Option<Vec<String>>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ListUserStationsResponse {
    pub stations: Vec<UserStationDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DeployStationResponse {
    pub canister_id: Principal,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum CanDeployStationResponse {
    NotAllowed(UserSubscriptionStatusDTO),
    Allowed(usize),
    QuotaExceeded,
}
