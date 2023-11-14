use candid::{CandidType, Deserialize};

use super::TimestampRfc3339;

pub type UpgradeIdDTO = String;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum UpgradeExecutionScheduleDTO {
    Immediate,
    Scheduled { execution_time: TimestampRfc3339 },
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum UpgradeStatusDTO {
    Cancelled {
        reason: Option<String>,
    },
    Processing {
        started_at: TimestampRfc3339,
    },
    Submitted,
    Pending,
    Completed {
        signature: Option<String>,
        hash: Option<String>,
        completed_at: TimestampRfc3339,
    },
    Approved,
    Rejected {
        reason: String,
    },
    Failed {
        reason: String,
    },
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct UpgradeMetadataDTO {
    pub key: String,
    pub value: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct UpgradeDTO {
    pub id: UpgradeIdDTO,
    pub status: UpgradeStatusDTO,
    pub expiration_dt: TimestampRfc3339,
    pub execution_plan: UpgradeExecutionScheduleDTO,
    pub metadata: Vec<UpgradeMetadataDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct UpgradeInput {
    pub expiration_dt: Option<TimestampRfc3339>,
    pub execution_plan: Option<UpgradeExecutionScheduleDTO>,
    pub metadata: Option<Vec<UpgradeMetadataDTO>>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct UpgradeResponse {
    pub upgrade: UpgradeDTO,
}
