use candid::{CandidType, Deserialize, Principal};

use crate::Sha256HashDTO;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ChangeCanisterTargetDTO {
    UpgradeStation,
    UpgradeUpgrader,
    UpgradeCanister(Principal),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ChangeCanisterOperationInput {
    pub target: ChangeCanisterTargetDTO,
    #[serde(with = "serde_bytes")]
    pub module: Vec<u8>,
    #[serde(deserialize_with = "orbit_essentials::deserialize::deserialize_option_blob")]
    pub arg: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ChangeCanisterOperationDTO {
    pub target: ChangeCanisterTargetDTO,
    pub module_checksum: Sha256HashDTO,
    pub arg_checksum: Option<Sha256HashDTO>,
}
