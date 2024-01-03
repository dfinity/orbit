use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ChangeCanisterTargetDTO {
    UpgradeWallet,
    UpgradeUpgrader,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ChangeCanisterOperationInput {
    pub target: ChangeCanisterTargetDTO,
    pub module: Vec<u8>,
    pub checksum: Vec<u8>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ChangeCanisterOperationDTO {
    pub input: ChangeCanisterOperationInput,
}
