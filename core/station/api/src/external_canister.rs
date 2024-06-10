use crate::Sha256HashDTO;
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct CreateExternalCanisterOperationInput {}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct CreateExternalCanisterOperationDTO {
    pub canister_id: Option<Principal>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct CanisterMethodDTO {
    pub canister_id: Principal,
    pub method_name: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct CallExternalCanisterOperationInput {
    pub validation_method: Option<CanisterMethodDTO>,
    pub execution_method: CanisterMethodDTO,
    #[serde(with = "serde_bytes")]
    pub arg: Vec<u8>,
    pub execution_method_cycles: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct CallExternalCanisterOperationDTO {
    pub validation_method: Option<CanisterMethodDTO>,
    pub execution_method: CanisterMethodDTO,
    pub arg_checksum: Sha256HashDTO,
    pub arg_rendering: Option<String>,
    pub execution_method_cycles: Option<u64>,
    pub execution_method_reply: Option<Vec<u8>>,
}
