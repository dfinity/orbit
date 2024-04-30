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
    pub module: Vec<u8>,
    pub arg: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ChangeCanisterOperationDTO {
    pub target: ChangeCanisterTargetDTO,
    pub module_checksum: Sha256HashDTO,
    pub arg_checksum: Option<Sha256HashDTO>,
}
