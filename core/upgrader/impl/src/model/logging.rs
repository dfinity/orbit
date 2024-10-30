use crate::upgrader_ic_cdk::api::time;
use orbit_essentials::{storable, types::Timestamp, utils::timestamp_to_rfc3339};
use serde::Serialize;

use super::{
    Account, AdminUser, Asset, DisasterRecoveryCommittee, MultiAssetAccount, RecoveryResult,
};

#[derive(Serialize)]
pub enum UpgradeResultLog {
    Success,
    Failure(String),
}

#[derive(Serialize)]
pub struct SetCommitteeLog {
    pub committee: DisasterRecoveryCommittee,
}

#[derive(Serialize)]
pub struct SetAccountsLog {
    pub accounts: Vec<Account>,
}

#[derive(Serialize)]
pub struct SetAccountsAndAssetsLog {
    pub multi_asset_accounts: Vec<MultiAssetAccount>,
    pub assets: Vec<Asset>,
}

#[derive(Serialize)]
pub struct RequestDisasterRecoveryLog {
    pub user: AdminUser,
    pub wasm_sha256: String,
    pub arg_sha256: String,
    pub install_mode: String,
}

#[derive(Serialize)]
pub struct DisasterRecoveryStartLog {
    pub wasm_sha256: String,
    pub arg_sha256: String,
    pub install_mode: String,
}

#[derive(Serialize)]
pub struct DisasterRecoveryResultLog {
    pub result: RecoveryResult,
}

#[derive(Serialize)]
pub struct DisasterRecoveryInProgressLog {
    pub operation: String,
}

pub enum LogEntryType {
    SetCommittee(SetCommitteeLog),
    SetAccounts(SetAccountsLog),
    SetAccountsAndAssets(SetAccountsAndAssetsLog),
    RequestDisasterRecovery(RequestDisasterRecoveryLog),
    DisasterRecoveryStart(DisasterRecoveryStartLog),
    DisasterRecoveryResult(DisasterRecoveryResultLog),
    UpgradeResult(UpgradeResultLog),
    DisasterRecoveryInProgress(DisasterRecoveryInProgressLog),
    DisasterRecoveryInProgressExpired(DisasterRecoveryInProgressLog),
}

#[derive(Debug)]
#[storable]
pub struct LogEntry {
    pub time: Timestamp,
    pub entry_type: String,
    pub message: String,
    pub data_json: String,
}

impl LogEntryType {
    pub fn to_type_string(&self) -> String {
        match self {
            LogEntryType::SetCommittee(_) => "set_committee".to_owned(),
            LogEntryType::SetAccounts(_) => "set_accounts".to_owned(),
            LogEntryType::RequestDisasterRecovery(_) => "request_disaster_recovery".to_owned(),
            LogEntryType::DisasterRecoveryStart(_) => "disaster_recovery_start".to_owned(),
            LogEntryType::DisasterRecoveryResult(_) => "disaster_recovery_result".to_owned(),
            LogEntryType::UpgradeResult(_) => "upgrade_result".to_owned(),
            LogEntryType::DisasterRecoveryInProgress(_) => {
                "disaster_recovery_in_progress".to_owned()
            }
            LogEntryType::DisasterRecoveryInProgressExpired(_) => {
                "disaster_recovery_in_progress_expired".to_owned()
            }
            LogEntryType::SetAccountsAndAssets(_) => "set_accounts_and_assets".to_owned(),
        }
    }

    pub fn to_message(&self) -> String {
        match self {
            LogEntryType::SetCommittee(data) => format!(
                "Set committee of {}, with quorum of {}",
                data.committee
                    .users
                    .iter()
                    .map(|u| u.to_summary())
                    .collect::<Vec<_>>()
                    .join(", "),
                data.committee.quorum
            ),
            LogEntryType::SetAccounts(data) => {
                format!("Set {} disaster recovery account(s)", data.accounts.len(),)
            }
            LogEntryType::RequestDisasterRecovery(data) => format!(
                "{} requested disaster recovery with wasm hash {} and arg hash {}",
                data.user.to_summary(),
                hex::encode(&data.wasm_sha256),
                hex::encode(&data.arg_sha256)
            ),

            LogEntryType::DisasterRecoveryStart(data) => format!(
                "Disaster recovery successfully initiated to {} station with wasm {}",
                data.install_mode,
                hex::encode(&data.wasm_sha256)
            ),
            LogEntryType::DisasterRecoveryResult(data) => match data.result {
                RecoveryResult::Success => "Disaster recovery succeeded".to_owned(),
                RecoveryResult::Failure(ref failure) => {
                    format!("Disaster recovery failed: {}", failure.reason)
                }
            },
            LogEntryType::UpgradeResult(data) => match data {
                UpgradeResultLog::Success => "Upgrade succeeded".to_owned(),
                UpgradeResultLog::Failure(ref reason) => format!("Upgrade failed: {}", reason),
            },
            LogEntryType::DisasterRecoveryInProgress(data) => {
                format!(
                    "Disaster recovery in progress, rejecting operation {}",
                    data.operation
                )
            }
            LogEntryType::DisasterRecoveryInProgressExpired(data) => {
                format!(
                    "Disaster recovery in-progress expired before operation {}",
                    data.operation
                )
            }
            LogEntryType::SetAccountsAndAssets(data) => {
                format!(
                    "Set {} multi-asset account(s) and {} asset(s)",
                    data.multi_asset_accounts.len(),
                    data.assets.len()
                )
            }
        }
    }

    pub fn to_json_string(&self) -> Result<String, String> {
        match self {
            LogEntryType::SetCommittee(data) => serde_json::to_string(data),
            LogEntryType::SetAccounts(data) => serde_json::to_string(data),
            LogEntryType::RequestDisasterRecovery(data) => serde_json::to_string(data),
            LogEntryType::DisasterRecoveryStart(data) => serde_json::to_string(data),
            LogEntryType::DisasterRecoveryResult(data) => serde_json::to_string(data),
            LogEntryType::UpgradeResult(data) => serde_json::to_string(data),
            LogEntryType::DisasterRecoveryInProgress(data) => serde_json::to_string(data),
            LogEntryType::DisasterRecoveryInProgressExpired(data) => serde_json::to_string(data),
            LogEntryType::SetAccountsAndAssets(data) => serde_json::to_string(data),
        }
        .map_err(|err| format!("Failed to serialize log entry: {}", err))
    }
}

impl LogEntry {
    pub fn try_from_entry_type(entry_type: LogEntryType) -> Result<Self, String> {
        Ok(LogEntry {
            time: time(),
            entry_type: entry_type.to_type_string(),
            message: entry_type.to_message(),
            data_json: entry_type.to_json_string()?,
        })
    }
}

impl From<LogEntry> for upgrader_api::LogEntry {
    fn from(value: LogEntry) -> Self {
        upgrader_api::LogEntry {
            time: timestamp_to_rfc3339(&value.time),
            entry_type: value.entry_type,
            message: value.message,
            data_json: value.data_json,
        }
    }
}
