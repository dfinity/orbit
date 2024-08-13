use super::permission::Allow;
use super::resource::ValidationMethodResourceTarget;
use super::{ConfigureExternalCanisterSettingsInput, RequestPolicyRule};
use crate::errors::ExternalCanisterError;
use candid::Principal;
use orbit_essentials::storable;
use orbit_essentials::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use station_api::GetExternalCanisterFiltersResponse;
use std::collections::BTreeSet;
use std::hash::Hash;

/// The external canister id, which is a UUID.
pub type ExternalCanisterEntryId = UUID;

/// Represents an external canister that the station can interact with.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalCanister {
    /// The external canister id, which is a UUID.
    pub id: ExternalCanisterEntryId,
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
pub struct ExternalCanisterCallPermission {
    pub allow: Allow,
    pub validation_method: ValidationMethodResourceTarget,
    pub execution_method: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalCanisterPermissions {
    pub read: Allow,
    pub change: Allow,
    pub calls: Vec<ExternalCanisterCallPermission>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalCanisterChangeRequestPolicyRule {
    pub policy_id: UUID,
    pub rule: RequestPolicyRule,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalCanisterCallRequestPolicyRule {
    pub policy_id: UUID,
    pub rule: RequestPolicyRule,
    pub validation_method: ValidationMethodResourceTarget,
    pub execution_method: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalCanisterRequestPolicies {
    pub change: Vec<ExternalCanisterChangeRequestPolicyRule>,
    pub calls: Vec<ExternalCanisterCallRequestPolicyRule>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalCanisterKey {
    pub id: ExternalCanisterEntryId,
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
    pub canister_id: Principal,
    pub can_change: bool,
    pub can_fund: bool,
    pub can_call: Vec<ExternalCanisterCallerMethodsPrivileges>,
}

pub type ExternalCanisterAvailableFilters = GetExternalCanisterFiltersResponse;

impl ExternalCanister {
    pub const MAX_NAME_LENGTH: usize = 100;
    pub const MAX_LABEL_LENGTH: usize = 50;
    pub const MAX_LABELS: usize = 10;
    pub const MAX_DESCRIPTION_LENGTH: usize = 1000;

    /// Creates a new external canister key from the given key components.
    pub fn key(id: ExternalCanisterEntryId) -> ExternalCanisterKey {
        ExternalCanisterKey { id }
    }

    /// Extracts the lookup key of the external canister.
    pub fn to_key(&self) -> ExternalCanisterKey {
        Self::key(self.id)
    }

    /// Checks if the external canister is archived.
    pub fn is_archived(&self) -> bool {
        self.state == ExternalCanisterState::Archived
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

        if let Some(state) = changes.state {
            self.state = state;
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

#[cfg(any(test, feature = "canbench"))]
pub mod external_canister_test_utils {
    use super::*;
    use crate::core::ic_cdk::next_time;
    use candid::Principal;
    use uuid::Uuid;

    pub fn mock_external_canister() -> ExternalCanister {
        let resource_id = *Uuid::new_v4().as_bytes();
        let canister_id = Principal::from_slice(&resource_id);

        ExternalCanister {
            id: resource_id,
            canister_id,
            name: canister_id.to_string(),
            description: Some("Test canister description".to_string()),
            labels: vec!["test".to_string()],
            state: ExternalCanisterState::Active,
            created_at: next_time(),
            modified_at: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use external_canister_test_utils::mock_external_canister;
    use ic_stable_structures::Storable;
    use orbit_essentials::model::ModelValidator;

    #[test]
    fn valid_model_serialization() {
        let model = mock_external_canister();

        let serialized_model = model.to_bytes();
        let deserialized_model = ExternalCanister::from_bytes(serialized_model);

        assert_eq!(model.id, deserialized_model.id);
        assert_eq!(model.canister_id, deserialized_model.canister_id);
        assert_eq!(model.name, deserialized_model.name);
        assert_eq!(model.description, deserialized_model.description);
        assert_eq!(model.labels, deserialized_model.labels);
        assert_eq!(model.state, deserialized_model.state);
        assert_eq!(model.created_at, deserialized_model.created_at);
        assert_eq!(model.modified_at, deserialized_model.modified_at);
    }

    #[test]
    fn valid_external_canister_validation() {
        let mut external_canister = mock_external_canister();
        external_canister.name = "Test canister".to_string();
        external_canister.description = Some("Test canister description".to_string());
        external_canister.labels = vec!["test".to_string()];

        assert!(external_canister.validate().is_ok());
    }

    #[test]
    fn invalid_external_canister_validation() {
        let mut external_canister = mock_external_canister();
        external_canister.name = "".to_string();

        assert!(external_canister.validate().is_err());
    }

    #[test]
    fn invalid_external_canister_validation_with_long_name() {
        let result = validate_name(&"a".repeat(ExternalCanister::MAX_NAME_LENGTH + 1));

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ExternalCanisterError::ValidationError {
                info: format!(
                    "The name of the external canister cannot be longer than {} characters.",
                    ExternalCanister::MAX_NAME_LENGTH
                )
            }
        );
    }

    #[test]
    fn invalid_external_canister_validation_with_long_description() {
        let result = validate_description(&Some(
            "a".repeat(ExternalCanister::MAX_DESCRIPTION_LENGTH + 1),
        ));

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ExternalCanisterError::ValidationError {
                info: format!(
                    "The description of the external canister cannot be longer than {} characters.",
                    ExternalCanister::MAX_DESCRIPTION_LENGTH
                )
            }
        );
    }

    #[test]
    fn invalid_external_canister_validation_with_long_label() {
        let result = validate_labels(&["a".repeat(ExternalCanister::MAX_LABEL_LENGTH + 1)]);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ExternalCanisterError::ValidationError {
                info: format!(
                    "The label '{}' cannot be longer than {} characters.",
                    "a".repeat(ExternalCanister::MAX_LABEL_LENGTH + 1),
                    ExternalCanister::MAX_LABEL_LENGTH
                )
            }
        );
    }

    #[test]
    fn invalid_external_canister_validation_with_too_many_labels() {
        let result = validate_labels(&vec!["a".to_string(); ExternalCanister::MAX_LABELS + 1]);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ExternalCanisterError::ValidationError {
                info: format!(
                    "The external canister cannot have more than {} labels.",
                    ExternalCanister::MAX_LABELS
                )
            }
        );
    }

    #[test]
    fn invalid_external_canister_validation_with_duplicate_labels() {
        let result = validate_labels(&["a".to_string(), "a".to_string()]);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ExternalCanisterError::ValidationError {
                info: "The labels cannot be duplicated.".to_string()
            }
        );
    }

    #[test]
    fn update_existing_model_with_changes() {
        let mut model = mock_external_canister();
        let changes = ConfigureExternalCanisterSettingsInput {
            name: Some("New name".to_string()),
            description: Some("New description".to_string()),
            labels: Some(vec!["new".to_string()]),
            permissions: None,
            request_policies: None,
            state: Some(ExternalCanisterState::Archived),
        };

        model.update_with(changes);

        assert_eq!(model.name, "New name".to_string());
        assert_eq!(model.description, Some("New description".to_string()));
        assert_eq!(model.labels, vec!["new".to_string()]);
        assert_eq!(model.state, ExternalCanisterState::Archived);
    }
}
