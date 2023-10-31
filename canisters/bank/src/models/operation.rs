use super::{AccountId, OperationCode, OperationDecision, OperationStatus};
use crate::errors::OperationError;
use candid::{CandidType, Deserialize};
use ic_canister_core::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use ic_canister_macros::stable_object;
use std::collections::{HashMap, HashSet};

pub const OPERATION_METADATA_KEY_TRANSFER_ID: &str = "transfer_id";
pub const OPERATION_METADATA_KEY_WALLET_ID: &str = "wallet_id";

/// The operation id, which is a UUID.
pub type OperationId = UUID;
/// Represents an operation within the system.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Operation {
    /// The operation id, which is a UUID.
    pub id: OperationId,
    /// The account id that resulted in the operation creation.
    ///
    /// When the operation is created by the system, this field is `None`.
    pub originator_account_id: Option<AccountId>,
    /// The status of the operation.
    pub status: OperationStatus,
    /// An operation code that represents the operation type, e.g. "transfer".
    pub code: OperationCode,
    /// The decisions made by the accounts that this operation is assigned to.
    pub decisions: Vec<OperationDecision>,
    /// The operation metadata key-value pairs, where the key is unique and the first entry in the tuple.
    /// E.g. "transfer_id" => "1234".
    pub metadata: Vec<(String, String)>,
    /// The timestamp of the operation creation.
    pub created_timestamp: Timestamp,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperationKey {
    /// The operation id, which is a UUID.
    pub id: OperationId,
}

pub struct OperationValidator<'model> {
    operation: &'model Operation,
}

impl<'operation> OperationValidator<'operation> {
    pub const MAX_METADATA_KEY_LEN: u8 = 24;
    pub const MAX_METADATA_VALUE_LEN: u8 = 255;
    pub const MAX_METADATA_ENTRIES: u8 = 10;
    pub const MAX_DECISION_ENTRIES: u8 = 10;

    pub fn new(operation: &'operation Operation) -> OperationValidator {
        OperationValidator { operation }
    }

    pub fn validate_decisions(&self) -> ModelValidatorResult<OperationError> {
        if self.operation.decisions.len() > Self::MAX_DECISION_ENTRIES as usize {
            return Err(OperationError::ValidationError {
                info: format!(
                    "Operation decisions count exceeds the maximum allowed: {}",
                    Self::MAX_DECISION_ENTRIES
                ),
            });
        }

        self.operation
            .decisions
            .iter()
            .try_for_each(|decision| decision.validate())?;

        Ok(())
    }

    pub fn validate_metadata(&self) -> ModelValidatorResult<OperationError> {
        if self.operation.metadata.len() > Self::MAX_METADATA_ENTRIES as usize {
            return Err(OperationError::ValidationError {
                info: format!(
                    "Operation metadata count exceeds the maximum allowed: {}",
                    Self::MAX_METADATA_ENTRIES
                ),
            });
        }

        for (key, value) in self.operation.metadata.iter() {
            if key.len() > Self::MAX_METADATA_KEY_LEN as usize {
                return Err(OperationError::ValidationError {
                    info: format!(
                        "Operation metadata key length exceeds the maximum allowed: {}",
                        Self::MAX_METADATA_KEY_LEN
                    ),
                });
            }

            if value.len() > Self::MAX_METADATA_VALUE_LEN as usize {
                return Err(OperationError::ValidationError {
                    info: format!(
                        "Operation metadata value length exceeds the maximum allowed: {}",
                        Self::MAX_METADATA_VALUE_LEN
                    ),
                });
            }
        }

        Ok(())
    }

    pub fn validate(&self) -> ModelValidatorResult<OperationError> {
        self.validate_metadata()?;
        self.validate_decisions()?;

        Ok(())
    }
}

impl ModelValidator<OperationError> for Operation {
    fn validate(&self) -> ModelValidatorResult<OperationError> {
        OperationValidator::new(self).validate()
    }
}

impl Operation {
    /// Creates a new operation key from the given key components.
    pub fn key(operation_id: OperationId) -> OperationKey {
        OperationKey { id: operation_id }
    }

    pub fn to_key(&self) -> OperationKey {
        Operation::key(self.id.to_owned())
    }

    pub fn accounts(&self) -> HashSet<AccountId> {
        let mut accounts = HashSet::new();
        if let Some(originator_account_id) = self.originator_account_id.to_owned() {
            accounts.insert(originator_account_id);
        }

        self.decisions
            .iter()
            .map(|decision| decision.account_id.to_owned())
            .for_each(|account_id| {
                accounts.insert(account_id);
            });

        accounts
    }

    pub fn metadata_map(&self) -> HashMap<String, String> {
        self.metadata
            .iter()
            .map(|(key, value)| (key.to_owned(), value.to_owned()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fail_operation_metadata_too_many_entries() {
        let operation = Operation {
            id: [0; 16],
            originator_account_id: None,
            status: OperationStatus::Pending,
            code: OperationCode::ApproveTransfer,
            decisions: vec![],
            metadata: vec![
                ("a".repeat(25), "b".repeat(25)),
                ("c".repeat(25), "d".repeat(25)),
                ("e".repeat(25), "f".repeat(25)),
                ("g".repeat(25), "h".repeat(25)),
                ("i".repeat(25), "j".repeat(25)),
                ("k".repeat(25), "l".repeat(25)),
                ("m".repeat(25), "n".repeat(25)),
                ("o".repeat(25), "p".repeat(25)),
                ("q".repeat(25), "r".repeat(25)),
                ("s".repeat(25), "t".repeat(25)),
                ("u".repeat(25), "v".repeat(25)),
            ],
            created_timestamp: 0,
            last_modification_timestamp: 0,
        };

        let result = OperationValidator::new(&operation).validate_metadata();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            OperationError::ValidationError {
                info: "Operation metadata count exceeds the maximum allowed: 10".to_string()
            }
        );
    }

    #[test]
    fn test_operation_metadata_validation() {
        let operation = Operation {
            id: [0; 16],
            originator_account_id: None,
            status: OperationStatus::Pending,
            code: OperationCode::ApproveTransfer,
            decisions: vec![],
            metadata: vec![
                ("a".repeat(24), "b".repeat(24)),
                ("c".repeat(24), "d".repeat(24)),
                ("e".repeat(24), "f".repeat(24)),
                ("g".repeat(24), "h".repeat(24)),
                ("i".repeat(24), "j".repeat(24)),
                ("k".repeat(24), "l".repeat(24)),
                ("m".repeat(24), "n".repeat(24)),
                ("o".repeat(24), "p".repeat(24)),
                ("q".repeat(24), "r".repeat(24)),
                ("s".repeat(24), "t".repeat(24)),
            ],
            created_timestamp: 0,
            last_modification_timestamp: 0,
        };

        let result = OperationValidator::new(&operation).validate_metadata();

        assert!(result.is_ok());
    }

    #[test]
    fn fail_operation_metadata_key_too_long() {
        let operation = Operation {
            id: [0; 16],
            originator_account_id: None,
            status: OperationStatus::Pending,
            code: OperationCode::ApproveTransfer,
            decisions: vec![],
            metadata: vec![("a".repeat(25), "b".repeat(24))],
            created_timestamp: 0,
            last_modification_timestamp: 0,
        };

        let result = OperationValidator::new(&operation).validate_metadata();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            OperationError::ValidationError {
                info: "Operation metadata key length exceeds the maximum allowed: 24".to_string()
            }
        );
    }

    #[test]
    fn fail_operaton_decisions_too_many_entries() {
        let operation = Operation {
            id: [0; 16],
            originator_account_id: None,
            status: OperationStatus::Pending,
            code: OperationCode::ApproveTransfer,
            decisions: vec![
                OperationDecision {
                    account_id: [0; 16],
                    read: false,
                    status: OperationStatus::Rejected,
                    status_reason: None,
                    decided_dt: None,
                    last_modification_timestamp: 0,
                };
                OperationValidator::MAX_DECISION_ENTRIES as usize + 1
            ],
            metadata: vec![],
            created_timestamp: 0,
            last_modification_timestamp: 0,
        };

        let result = OperationValidator::new(&operation).validate_decisions();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            OperationError::ValidationError {
                info: "Operation decisions count exceeds the maximum allowed: 10".to_string()
            }
        );
    }

    #[test]
    fn test_operation_decisions_validation() {
        let operation = Operation {
            id: [0; 16],
            originator_account_id: None,
            status: OperationStatus::Pending,
            code: OperationCode::ApproveTransfer,
            decisions: vec![
                OperationDecision {
                    account_id: [0; 16],
                    read: false,
                    status: OperationStatus::Rejected,
                    status_reason: None,
                    decided_dt: None,
                    last_modification_timestamp: 0,
                };
                OperationValidator::MAX_DECISION_ENTRIES as usize - 1
            ],
            metadata: vec![],
            created_timestamp: 0,
            last_modification_timestamp: 0,
        };

        let result = OperationValidator::new(&operation).validate_decisions();

        assert!(result.is_ok());
    }
}
