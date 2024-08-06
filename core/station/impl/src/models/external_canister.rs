use super::indexes::external_canister_index::{ExternalCanisterIndex, ExternalCanisterIndexKind};
use super::resource::ValidationMethodResourceTarget;
use super::ConfigureExternalCanisterSettingsInput;
use crate::core::utils::format_unique_string;
use crate::errors::ExternalCanisterError;
use candid::Principal;
use orbit_essentials::storable;
use orbit_essentials::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use std::collections::BTreeSet;
use std::hash::Hash;

/// The external canister id, which is a UUID.
pub type ExternalCanisterId = UUID;

/// Represents an external canister that the station can interact with.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalCanister {
    /// The external canister id, which is a UUID.
    pub id: ExternalCanisterId,
    /// The canister id, which is a Principal.
    pub canister_id: Principal,
    /// The canister name.
    pub name: String,
    /// The canister description.
    pub description: Option<String>,
    /// The canister labels.
    ///
    /// This is a list of strings that can be used to categorize the canister
    /// and make it easier to search for.
    pub labels: Vec<String>,
    /// The state of the canister (e.g. active, archived, etc.)
    pub state: ExternalCanisterState,
    /// When the canister was added to the station.
    pub created_at: Timestamp,
    /// The last time the record was updated.
    pub modified_at: Option<Timestamp>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalCanisterKey {
    pub id: ExternalCanisterId,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExternalCanisterState {
    Active,
    Archived,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalCanisterCallerMethodsPrivileges {
    pub validation_method: ValidationMethodResourceTarget,
    pub execution_method: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalCanisterCallerPrivileges {
    pub id: UUID,
    pub can_change: bool,
    pub can_call: Vec<ExternalCanisterCallerMethodsPrivileges>,
}

impl ExternalCanister {
    pub const MAX_NAME_LENGTH: usize = 100;
    pub const MAX_LABEL_LENGTH: usize = 50;
    pub const MAX_LABELS: usize = 10;
    pub const MAX_DESCRIPTION_LENGTH: usize = 1000;

    /// Creates a new external canister key from the given key components.
    pub fn key(id: ExternalCanisterId) -> ExternalCanisterKey {
        ExternalCanisterKey { id }
    }

    /// Extracts the lookup key of the external canister.
    pub fn to_key(&self) -> ExternalCanisterKey {
        Self::key(self.id)
    }

    /// Converts the external canister to an index by its name.
    pub fn to_index_by_name(&self) -> ExternalCanisterIndex {
        ExternalCanisterIndex {
            index: ExternalCanisterIndexKind::Name(format_unique_string(self.name.as_str())),
            external_canister_id: self.id,
        }
    }

    /// Converts the external canister to indexes by its labels.
    pub fn to_index_by_labels(&self) -> Vec<ExternalCanisterIndex> {
        self.labels
            .iter()
            .map(|label| ExternalCanisterIndex {
                index: ExternalCanisterIndexKind::Label(format_unique_string(label.as_str())),
                external_canister_id: self.id,
            })
            .collect()
    }

    /// Converts the external canister to an index by its canister id.
    pub fn to_index_by_canister_id(&self) -> ExternalCanisterIndex {
        ExternalCanisterIndex {
            index: ExternalCanisterIndexKind::CanisterId(self.canister_id),
            external_canister_id: self.id,
        }
    }

    /// Converts the external canister to indexes to facilitate searching.
    pub fn indexes(&self) -> Vec<ExternalCanisterIndex> {
        let mut indexes = vec![self.to_index_by_name(), self.to_index_by_canister_id()];
        indexes.extend(self.to_index_by_labels());

        indexes
    }

    pub fn update_with(&mut self, changes: ConfigureExternalCanisterSettingsInput) {
        if let Some(name) = changes.name {
            self.name = name;
        }

        if let Some(description) = changes.description {
            self.description = Some(description);
        }

        if let Some(labels) = changes.labels {
            self.labels = labels;
        }
    }
}

fn validate_name(name: &str) -> ModelValidatorResult<ExternalCanisterError> {
    if name.is_empty() {
        return Err(ExternalCanisterError::ValidationError {
            info: "The name of the external canister cannot be empty.".to_string(),
        });
    }

    if name.len() > ExternalCanister::MAX_NAME_LENGTH {
        return Err(ExternalCanisterError::ValidationError {
            info: format!(
                "The name of the external canister cannot be longer than {} characters.",
                ExternalCanister::MAX_NAME_LENGTH
            ),
        });
    }

    Ok(())
}

fn validate_description(
    description: &Option<String>,
) -> ModelValidatorResult<ExternalCanisterError> {
    if let Some(description) = description {
        if description.len() > ExternalCanister::MAX_DESCRIPTION_LENGTH {
            return Err(ExternalCanisterError::ValidationError {
                info: format!(
                    "The description of the external canister cannot be longer than {} characters.",
                    ExternalCanister::MAX_DESCRIPTION_LENGTH
                ),
            });
        }
    }

    Ok(())
}

fn validate_labels(labels: &[String]) -> ModelValidatorResult<ExternalCanisterError> {
    if labels.len() > ExternalCanister::MAX_LABELS {
        return Err(ExternalCanisterError::ValidationError {
            info: format!(
                "The external canister cannot have more than {} labels.",
                ExternalCanister::MAX_LABELS
            ),
        });
    }

    for label in labels {
        if label.len() > ExternalCanister::MAX_LABEL_LENGTH {
            return Err(ExternalCanisterError::ValidationError {
                info: format!(
                    "The label '{}' cannot be longer than {} characters.",
                    label,
                    ExternalCanister::MAX_LABEL_LENGTH
                ),
            });
        }
    }

    let labels_set: BTreeSet<&String> = labels.iter().collect();
    if labels_set.len() != labels.len() {
        return Err(ExternalCanisterError::ValidationError {
            info: "The labels cannot be duplicated.".to_string(),
        });
    }

    Ok(())
}

impl ModelValidator<ExternalCanisterError> for ExternalCanister {
    fn validate(&self) -> ModelValidatorResult<ExternalCanisterError> {
        validate_name(&self.name)?;
        validate_description(&self.description)?;
        validate_labels(&self.labels)?;

        Ok(())
    }
}
