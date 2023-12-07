use candid::{CandidType, Deserialize};

use crate::UuidDTO;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum UpgradeTargetDTO {
    Wallet,
    Upgrader,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct UpgradeDTO {
    pub id: UuidDTO,
    pub target: UpgradeTargetDTO,
    pub module: Vec<u8>,
    pub checksum: Vec<u8>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct UpgradeOperationInput {
    pub target: UpgradeTargetDTO,
    pub module: Vec<u8>,
    pub checksum: Vec<u8>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct UpgradeOperationDTO {
    pub upgrade: Option<UpgradeDTO>,
    pub input: UpgradeOperationInput,
}
