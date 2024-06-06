use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct CreateManagedCanisterOperationInput {}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct CreateManagedCanisterOperationDTO {
    pub canister_id: Option<Principal>,
}
