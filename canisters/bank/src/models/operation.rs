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
#[stable_object(size = 4096)]
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

#[stable_object(size = 48)]
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
