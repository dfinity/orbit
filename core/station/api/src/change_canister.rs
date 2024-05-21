use candid::{CandidType, Deserialize, Principal};
use orbit_essentials::cdk::api::management_canister::main::{self as mgmt};
use orbit_essentials::storable;

use crate::Sha256HashDTO;

#[storable]
#[derive(Copy, Clone, Debug, CandidType, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanisterInstallMode {
    #[serde(rename = "install")]
    Install = 1,
    #[serde(rename = "reinstall")]
    Reinstall = 2,
    #[serde(rename = "upgrade")]
    Upgrade = 3,
}

impl From<CanisterInstallMode> for mgmt::CanisterInstallMode {
    fn from(mode: CanisterInstallMode) -> Self {
        match mode {
            CanisterInstallMode::Install => mgmt::CanisterInstallMode::Install,
            CanisterInstallMode::Reinstall => mgmt::CanisterInstallMode::Reinstall,
            CanisterInstallMode::Upgrade => mgmt::CanisterInstallMode::Upgrade(None),
        }
    }
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct InstallCanisterInputDTO {
    pub canister_id: Principal,
    pub mode: CanisterInstallMode,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ChangeCanisterTargetDTO {
    UpgradeStation,
    UpgradeUpgrader,
    InstallCanister(InstallCanisterInputDTO),
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
