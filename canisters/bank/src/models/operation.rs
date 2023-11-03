use super::{OperationCode, OperationDecision, OperationStatus, UserId};
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
    /// The user id that resulted in the operation creation.
    ///
    /// When the operation is created by the system, this field is `None`.
    pub proposed_by: Option<UserId>,
    /// The status of the operation.
    pub status: OperationStatus,
    /// An operation code that represents the operation type, e.g. "transfer".
    pub code: OperationCode,
    /// The decisions made by the users that this operation is assigned to.
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

    pub fn users(&self) -> HashSet<UserId> {
        let mut users = HashSet::new();
        if let Some(proposed_by) = self.proposed_by.to_owned() {
            users.insert(proposed_by);
        }

        self.decisions
            .iter()
            .map(|decision| decision.user_id.to_owned())
            .for_each(|user_id| {
                users.insert(user_id);
            });

        users
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
    use super::operation_test_utils::mock_operation;
    use super::*;

    #[test]
    fn fail_operation_metadata_too_many_entries() {
        let mut operation = mock_operation();
        operation.metadata = vec![
            ("foo".to_string(), "bar".to_string());
            OperationValidator::MAX_METADATA_ENTRIES as usize + 1
        ];

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
        let mut operation = mock_operation();
        operation.metadata = vec![("a".repeat(24), "b".repeat(24)); 10];

        let result = OperationValidator::new(&operation).validate_metadata();

        assert!(result.is_ok());
    }

    #[test]
    fn fail_operation_metadata_key_too_long() {
        let mut operation = mock_operation();
        operation.metadata = vec![("a".repeat(25), "b".repeat(24))];

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
        let mut operation = mock_operation();
        operation.decisions = vec![
            OperationDecision {
                user_id: [0; 16],
                read: false,
                status: OperationStatus::Rejected,
                status_reason: None,
                decided_dt: None,
                last_modification_timestamp: 0,
            };
            OperationValidator::MAX_DECISION_ENTRIES as usize + 1
        ];

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
        let mut operation = mock_operation();
        operation.decisions = vec![
            OperationDecision {
                user_id: [0; 16],
                read: false,
                status: OperationStatus::Rejected,
                status_reason: None,
                decided_dt: None,
                last_modification_timestamp: 0,
            };
            OperationValidator::MAX_DECISION_ENTRIES as usize - 1
        ];

        let result = OperationValidator::new(&operation).validate_decisions();

        assert!(result.is_ok());
    }
}

#[cfg(test)]
pub mod operation_test_utils {
    use super::*;

    pub fn mock_operation() -> Operation {
        Operation {
            id: [0; 16],
            proposed_by: Some([1; 16]),
            status: OperationStatus::Adopted,
            code: OperationCode::ApproveTransfer,
            decisions: vec![OperationDecision {
                user_id: [1; 16],
                read: true,
                status: OperationStatus::Adopted,
                status_reason: None,
                decided_dt: Some(0),
                last_modification_timestamp: 0,
            }],
            metadata: vec![("a".repeat(25), "b".repeat(24))],
            created_timestamp: 0,
            last_modification_timestamp: 0,
        }
    }
}
