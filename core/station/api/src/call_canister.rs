use candid::{CandidType, Deserialize, Principal};

use crate::Sha256HashDTO;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct CanisterMethodDTO {
    pub canister_id: Principal,
    pub method_name: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct CallCanisterOperationInput {
    pub validation_method: Option<CanisterMethodDTO>,
    pub execution_method: CanisterMethodDTO,
    #[serde(with = "serde_bytes")]
    pub arg: Vec<u8>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct CallCanisterOperationDTO {
    pub validation_method: Option<CanisterMethodDTO>,
    pub execution_method: CanisterMethodDTO,
    pub arg_checksum: Sha256HashDTO,
    pub arg_rendering: Option<String>,
    pub execution_method_reply: Option<Vec<u8>>,
}
