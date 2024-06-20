use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::UuidDTO;

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct DisasterRecoveryCommitteeDTO {
    pub user_group_id: UuidDTO,
    pub quorum_percentage: u16,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct SetDisasterRecoveryOperationInput {
    pub committee: Option<DisasterRecoveryCommitteeDTO>,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct SetDisasterRecoveryOperationDTO {
    pub committee: Option<DisasterRecoveryCommitteeDTO>,
}
