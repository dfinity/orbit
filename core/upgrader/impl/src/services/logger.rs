use std::{cell::RefCell, sync::Arc};

use ic_stable_structures::{memory_manager::MemoryId, BTreeMap, Log};
use lazy_static::lazy_static;
use orbit_essentials::types::Timestamp;

use crate::{
    model::{LogEntry, LogEntryType},
    Memory, DEPRECATED_MEMORY_ID_LOG_DATA, DEPRECATED_MEMORY_ID_LOG_INDEX, MEMORY_ID_LOGS,
    MEMORY_MANAGER,
};

pub const MAX_GET_LOGS_LIMIT: u64 = 100;
pub const DEFAULT_GET_LOGS_LIMIT: u64 = 10;
pub const MAX_LOG_ENTRIES: u64 = 25000;

thread_local! {
    static DEPRECATED_STORAGE: RefCell<Log<LogEntry, Memory, Memory>> = RefCell::new(
        Log::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(DEPRECATED_MEMORY_ID_LOG_INDEX))),
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(DEPRECATED_MEMORY_ID_LOG_DATA))),
        ).expect("Failed to initialize deprecated log storage")
    );

    static STORAGE: RefCell<BTreeMap<Timestamp, LogEntry, Memory>> = RefCell::new(
        BTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(MEMORY_ID_LOGS))),
        )
    );
}

lazy_static! {
    pub static ref LOGGER_SERVICE: Arc<LoggerService> = Arc::new(LoggerService::default());
}

#[derive(Debug)]
pub struct GetLogsResult {
    pub logs: Vec<LogEntry>,
    pub total: u64,
    pub next_offset: Option<u64>,
}

#[derive(Clone, Default)]
pub struct LoggerService {}

impl LoggerService {
    /// Tries to log an entry to the storage.
    pub fn try_log(&self, entry_type: LogEntryType) -> Result<(), String> {
        let entry = LogEntry::try_from_entry_type(entry_type)?;
        STORAGE.with_borrow_mut(|storage| {
            if storage.len() >= MAX_LOG_ENTRIES {
                let _ = storage.pop_first();
            }
            storage.insert(entry.time, entry);
        });
        Ok(())
    }

    /// Logs an entry to the storage. If it cannot log the entry, it prints to the canister's logs.
    pub fn log(&self, entry_type: LogEntryType) {
        if let Err(err) = self.try_log(entry_type) {
            crate::upgrader_ic_cdk::api::print(format!("Failed to log entry: {}", err));
        }
    }

    /// Returns logs from the storage starting from the end of the log.
    pub fn get_logs(&self, offset: Option<u64>, limit: Option<u64>) -> GetLogsResult {
        STORAGE.with(|storage| {
            let borrowed = storage.borrow();

            let total = borrowed.len();

            if total == 0 {
                return GetLogsResult {
                    logs: vec![],
                    total,
                    next_offset: None,
                };
            }

            let offset = offset.unwrap_or(0);
            let limit = limit
                .unwrap_or(DEFAULT_GET_LOGS_LIMIT)
                .min(MAX_GET_LOGS_LIMIT);

            let logs = borrowed
                .iter()
                .rev()
                .skip(offset as usize)
                .take(limit as usize)
                .map(|(_, v)| v)
                .collect::<Vec<_>>();

            let next_offset = if total > offset + limit {
                Some(offset + limit)
            } else {
                None
            };
            GetLogsResult {
                logs,
                total,
                next_offset,
            }
        })
    }

    /// Returns logs from the deprecated storage starting from the end of the log.
    pub fn deprecated_get_logs(&self, offset: Option<u64>, limit: Option<u64>) -> GetLogsResult {
        DEPRECATED_STORAGE.with(|storage| {
            let borrowed = storage.borrow();

            let total = borrowed.len();

            if total == 0 {
                return GetLogsResult {
                    logs: vec![],
                    total,
                    next_offset: None,
                };
            }

            let offset = offset.unwrap_or(0);
            let limit = limit
                .unwrap_or(DEFAULT_GET_LOGS_LIMIT)
                .min(MAX_GET_LOGS_LIMIT);

            let first_inclusive = u64::saturating_sub(total, offset + 1);
            let last_inclusive = u64::saturating_sub(total, offset + limit);

            let logs = (last_inclusive..=first_inclusive)
                .rev()
                .filter_map(|i| borrowed.get(i))
                .collect::<Vec<_>>();

            let next_offset = if last_inclusive > 0 {
                Some(offset + logs.len() as u64)
            } else {
                None
            };
            GetLogsResult {
                logs,
                total,
                next_offset,
            }
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::model::{
        tests::{mock_assets, mock_committee, mock_multi_asset_accounts},
        DisasterRecoveryResultLog, RecoveryResult, SetAccountsAndAssetsLog, SetCommitteeLog,
        UpgradeResultLog,
    };

    use super::*;

    #[test]
    fn test_logger_service() {
        let logger_service = LoggerService::default();
        logger_service.log(LogEntryType::SetCommittee(SetCommitteeLog {
            committee: mock_committee(),
        }));
        logger_service.log(LogEntryType::UpgradeResult(UpgradeResultLog::Success));
        logger_service.log(LogEntryType::DisasterRecoveryResult(
            DisasterRecoveryResultLog {
                result: RecoveryResult::Success,
            },
        ));
        logger_service.log(LogEntryType::SetAccountsAndAssets(
            SetAccountsAndAssetsLog {
                multi_asset_accounts: mock_multi_asset_accounts(),
                assets: mock_assets(),
            },
        ));
        let result = logger_service.get_logs(None, None);

        assert_eq!(result.logs.len(), 4);
        assert_eq!(result.total, 4);
        assert_eq!(result.logs[3].entry_type, "set_committee".to_owned());
        assert_eq!(
            result.logs[0].entry_type,
            "set_accounts_and_assets".to_owned()
        );

        let result = logger_service.get_logs(Some(1), Some(2));
        assert_eq!(result.logs.len(), 2);
        assert_eq!(result.total, 4);
        assert_eq!(result.next_offset, Some(3));
        assert_eq!(
            result.logs[0].entry_type,
            "disaster_recovery_result".to_owned()
        );
        assert_eq!(result.logs[1].entry_type, "upgrade_result".to_owned());

        let result = logger_service.get_logs(Some(3), Some(10));
        assert_eq!(result.logs.len(), 1);
        assert_eq!(result.total, 4);
        assert_eq!(result.next_offset, None);
        assert_eq!(result.logs[0].entry_type, "set_committee".to_owned());
    }

    #[test]
    fn test_log_trimming() {
        for _ in 0..MAX_LOG_ENTRIES {
            LOGGER_SERVICE.log(LogEntryType::SetCommittee(SetCommitteeLog {
                committee: mock_committee(),
            }));
        }

        let result = LOGGER_SERVICE.get_logs(None, None);
        assert_eq!(result.total, MAX_LOG_ENTRIES);

        let latest_log_time = result.logs.last().unwrap().time;

        LOGGER_SERVICE.log(LogEntryType::SetCommittee(SetCommitteeLog {
            committee: mock_committee(),
        }));

        let result = LOGGER_SERVICE.get_logs(None, None);

        assert_eq!(result.total, MAX_LOG_ENTRIES);
        assert_ne!(result.logs.last().unwrap().time, latest_log_time);
    }

    #[test]
    fn test_deprecated_storage() {
        let logger_service = LoggerService::default();
        logger_service.log(LogEntryType::SetCommittee(SetCommitteeLog {
            committee: mock_committee(),
        }));

        // new logs should be in the new storage
        let result = logger_service.get_logs(None, None);
        assert_eq!(result.total, 1);

        // deprecated logs should not get new logs
        let result = logger_service.deprecated_get_logs(None, None);
        assert_eq!(result.total, 0);
    }
}
