use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ChangeCanisterTargetDTO {
    UpgradeWallet,
    UpgradeUpgrader,
    UpgradeCanister(Principal),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ChangeCanisterOperationInput {
    pub target: ChangeCanisterTargetDTO,
    pub module: Vec<u8>,
    pub arg: Option<Vec<u8>>,
    pub checksum: Vec<u8>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ChangeCanisterOperationDTO {
    pub input: ChangeCanisterOperationInput,
}
