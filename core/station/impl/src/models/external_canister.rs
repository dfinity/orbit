use super::permission::Allow;
use super::request_specifier::RequestSpecifier;
use super::resource::{
    CallExternalCanisterResourceTarget, ExecutionMethodResourceTarget, ExternalCanisterId,
    ValidationMethodResourceTarget,
};
use super::{
    CanisterMethod, ConfigureExternalCanisterSettingsInput, CreateExternalCanisterOperationInput,
    CreateExternalCanisterOperationKind, ExternalCanisterChangeCallRequestPoliciesInput,
    ExternalCanisterRequestPoliciesCreateInput, ExternalCanisterRequestPoliciesUpdateInput,
    Metadata, MonitorExternalCanisterStartInput, RequestPolicy, RequestPolicyRule,
};
use crate::core::validation::EnsureExternalCanister;
use crate::core::validation::{
    StringFieldValidator, StringFieldValidatorBuilder, ValidateField, VecFieldValidator,
    VecFieldValidatorBuilder,
};
use crate::errors::{ExternalCanisterError, ExternalCanisterValidationError};
use crate::repositories::REQUEST_POLICY_REPOSITORY;
use candid::Principal;
use lazy_static::lazy_static;
use orbit_essentials::model::{ContextualModel, ModelKey};
use orbit_essentials::repository::Repository;
use orbit_essentials::storable;
use orbit_essentials::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use station_api::GetExternalCanisterFiltersResponse;
use std::collections::BTreeSet;
use std::hash::Hash;
use std::sync::Arc;
use uuid::Uuid;

/// The external canister id, which is a UUID.
pub type ExternalCanisterEntryId = UUID;

lazy_static! {
    pub static ref EXTERNAL_CANISTER_NAME_VALIDATOR: StringFieldValidator = {
        StringFieldValidatorBuilder::new("name".to_string())
            .min_length(1)
            .max_length(ExternalCanister::MAX_NAME_LENGTH)
            .build()
    };
    pub static ref EXTERNAL_CANISTER_DESCRIPTION_VALIDATOR: StringFieldValidator = {
        StringFieldValidatorBuilder::new("description".to_string())
            .min_length(0)
            .max_length(ExternalCanister::MAX_DESCRIPTION_LENGTH)
            .build()
    };
    pub static ref EXTERNAL_CANISTER_LABEL_VALIDATOR: StringFieldValidator = {
        StringFieldValidatorBuilder::new("label".to_string())
            .min_length(1)
            .max_length(ExternalCanister::MAX_LABEL_LENGTH)
            .build()
    };
    pub static ref EXTERNAL_CANISTER_LABELS_VALIDATOR: VecFieldValidator<String> = {
        VecFieldValidatorBuilder::new(
            "labels".to_string(),
            Arc::new(EXTERNAL_CANISTER_LABEL_VALIDATOR.clone()),
        )
        .max_length(ExternalCanister::MAX_LABELS)
        .build()
    };
}

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
    /// The canister metadata.
    ///
    /// Can be used for storing additional information such as a group_id,
    /// logo, group_name, etc.
    #[serde(default)]
    pub metadata: Metadata,
    /// The state of the canister (e.g. active, archived, etc.)
    pub state: ExternalCanisterState,
    /// When the canister was added to the station.
    pub created_at: Timestamp,
    /// The last time the record was updated.
    pub modified_at: Option<Timestamp>,
    /// Is the canister monitored by the station.
    pub monitoring: Option<ExternalCanisterMonitoring>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalCanisterKey {
    pub id: ExternalCanisterEntryId,
}

impl ModelKey<ExternalCanisterKey> for ExternalCanister {
    fn key(&self) -> ExternalCanisterKey {
        ExternalCanisterKey { id: self.id }
    }
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

pub type ExternalCanisterMonitoring = MonitorExternalCanisterStartInput;

impl ExternalCanister {
    pub const MAX_NAME_LENGTH: usize = 100;
    pub const MAX_LABEL_LENGTH: usize = 50;
    pub const MAX_LABELS: usize = 10;
    pub const MAX_DESCRIPTION_LENGTH: usize = 1000;

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

        if let Some(change_metadata) = changes.change_metadata {
            self.metadata.change(change_metadata);
        }
    }
}

impl ModelValidator<ExternalCanisterError> for ExternalCanister {
    fn validate(&self) -> ModelValidatorResult<ExternalCanisterError> {
        EXTERNAL_CANISTER_NAME_VALIDATOR.validate_field(&self.name)?;

        if let Some(description) = &self.description {
            EXTERNAL_CANISTER_DESCRIPTION_VALIDATOR.validate_field(description)?;
        }

        EXTERNAL_CANISTER_LABELS_VALIDATOR.validate_field(&self.labels)?;

        let labels_set: BTreeSet<&String> = self.labels.iter().collect();
        if labels_set.len() != self.labels.len() {
            return Err(ExternalCanisterError::ValidationError {
                info: "The labels cannot be duplicated.".to_string(),
            });
        }

        self.metadata
            .validate()
            .map_err(|e| ExternalCanisterError::ValidationError {
                info: e.to_string(),
            })?;

        Ok(())
    }
}

fn assert_policy_is_associated_with_target_canister<F>(
    policy_id: &UUID,
    is_associated: F,
) -> ModelValidatorResult<ExternalCanisterValidationError>
where
    F: Fn(&RequestPolicy) -> bool,
{
    let policy = REQUEST_POLICY_REPOSITORY.get(policy_id).ok_or_else(|| {
        ExternalCanisterValidationError::ValidationError {
            info: format!(
                "The request policy with id '{}' does not exist.",
                Uuid::from_bytes(*policy_id).hyphenated()
            ),
        }
    })?;

    // validates if the policy matches the expected variant and canister_id or throws an error
    if !is_associated(&policy) {
        Err(ExternalCanisterValidationError::ValidationError {
            info: format!(
                "The request policy with id '{}' is not associated with the external canister.",
                Uuid::from_bytes(*policy_id).hyphenated()
            ),
        })?;
    }

    Ok(())
}

fn validate_change_policies_are_associated_with_target_canister(
    canister_id: &Principal,
    policy_ids: &[UUID],
) -> ModelValidatorResult<ExternalCanisterValidationError> {
    for policy_id in policy_ids {
        // validates if the policy matches the expected change variant and canister_id or throws an error
        assert_policy_is_associated_with_target_canister(policy_id, |policy| {
            matches!(
                policy.specifier,
                RequestSpecifier::ChangeExternalCanister(ExternalCanisterId::Canister(id)) if id == *canister_id
            )
        })?;
    }

    Ok(())
}

fn validate_calls_policies_are_associated_with_target_canister(
    canister_id: &Principal,
    policy_ids: &[UUID],
) -> ModelValidatorResult<ExternalCanisterValidationError> {
    for policy_id in policy_ids {
        // validates if the policy matches the expected call variant and canister_id or throws an error
        assert_policy_is_associated_with_target_canister(policy_id, |policy| {
            matches!(
                policy.specifier,
                RequestSpecifier::CallExternalCanister(CallExternalCanisterResourceTarget {
                    execution_method: ExecutionMethodResourceTarget::ExecutionMethod(
                        CanisterMethod {
                            canister_id: id,
                            method_name: _,
                        }
                    ),
                    ..
                }) if id == *canister_id
            )
        })?;
    }

    Ok(())
}

impl ModelValidator<ExternalCanisterValidationError>
    for ContextualModel<ExternalCanisterRequestPoliciesUpdateInput, Principal>
{
    fn validate(&self) -> ModelValidatorResult<ExternalCanisterValidationError> {
        let canister_id = &self.context;

        if let Some(change_policies) = &self.model.change {
            let policy_ids = change_policies
                .iter()
                .filter_map(|p| p.policy_id)
                .collect::<Vec<_>>();

            validate_change_policies_are_associated_with_target_canister(canister_id, &policy_ids)?;
        }

        if let Some(change_calls_policies_operation) = &self.model.calls {
            let policy_ids = match &change_calls_policies_operation {
                ExternalCanisterChangeCallRequestPoliciesInput::ReplaceAllBy(policies) => policies
                    .iter()
                    .filter_map(|p| p.policy_id)
                    .collect::<Vec<_>>(),
                ExternalCanisterChangeCallRequestPoliciesInput::RemoveByPolicyIds(policy_ids) => {
                    policy_ids.clone()
                }
                ExternalCanisterChangeCallRequestPoliciesInput::OverrideSpecifiedByExecutionMethods(
                    policies,
                ) => policies
                    .iter()
                    .flat_map(|p| p.policies.iter().filter_map(|p| p.policy_id))
                    .collect::<Vec<_>>(),
                ExternalCanisterChangeCallRequestPoliciesInput::OverrideSpecifiedByExecutionValidationMethodPairs(
                    operations,
                ) => operations
                    .iter()
                    .flat_map(|entry| entry
                        .policies
                        .iter()
                        .flat_map(|policy| policy.policy_id)
                    )
                    .collect::<Vec<_>>(),
            };

            validate_calls_policies_are_associated_with_target_canister(canister_id, &policy_ids)?;
        }

        Ok(())
    }
}

impl ModelValidator<ExternalCanisterValidationError>
    for ContextualModel<ExternalCanisterRequestPoliciesCreateInput, Principal>
{
    fn validate(&self) -> ModelValidatorResult<ExternalCanisterValidationError> {
        let canister_id = &self.context;

        validate_change_policies_are_associated_with_target_canister(
            canister_id,
            &self
                .model
                .change
                .iter()
                .filter_map(|p| p.policy_id)
                .collect::<Vec<_>>(),
        )?;

        validate_calls_policies_are_associated_with_target_canister(
            canister_id,
            &self
                .model
                .calls
                .iter()
                .filter_map(|p| p.policy_id)
                .collect::<Vec<_>>(),
        )?;

        Ok(())
    }
}

impl ModelValidator<ExternalCanisterValidationError> for CreateExternalCanisterOperationInput {
    fn validate(&self) -> ModelValidatorResult<ExternalCanisterValidationError> {
        match &self.kind {
            CreateExternalCanisterOperationKind::AddExisting(existing) => {
                let canister_id = existing.canister_id;
                EnsureExternalCanister::ensure_external_canister(canister_id)?;
                ContextualModel::new(self.request_policies.clone(), canister_id).validate()?;
            }
            CreateExternalCanisterOperationKind::CreateNew(_) => {
                if self
                    .request_policies
                    .change
                    .iter()
                    .any(|p| p.policy_id.is_some())
                    || self
                        .request_policies
                        .calls
                        .iter()
                        .any(|p| p.policy_id.is_some())
                {
                    return Err(ExternalCanisterValidationError::ValidationError {
                        info: "The request policies cannot have policy ids when creating a new external canister.".to_string(),
                    });
                }
            }
        }

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
            metadata: Metadata::default(),
            labels: vec!["test".to_string()],
            state: ExternalCanisterState::Active,
            created_at: next_time(),
            modified_at: None,
            monitoring: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::models::{
        request_policy_test_utils::mock_request_policy,
        CreateExternalCanisterOperationKindCreateNew, ExternalCanisterCallRequestPolicyRuleInput,
        ExternalCanisterChangeRequestPolicyRuleInput, ExternalCanisterPermissionsCreateInput,
        MetadataItem,
    };

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
        assert_eq!(model.monitoring, deserialized_model.monitoring);
    }

    #[test]
    fn valid_external_canister_validation() {
        let mut external_canister = mock_external_canister();
        external_canister.name = "Test canister".to_string();
        external_canister.description = Some("Test canister description".to_string());
        external_canister.labels = vec!["test".to_string()];
        external_canister.metadata =
            Metadata::new(BTreeMap::from([("key".to_string(), "value".to_string())]));

        assert!(external_canister.validate().is_ok());
    }

    #[test]
    fn invalid_external_canister_validation() {
        let mut external_canister = mock_external_canister();
        external_canister.name = "".to_string();
        assert!(external_canister.validate().is_err());

        external_canister.name = "Test canister".to_string();
        external_canister.metadata = Metadata::new(BTreeMap::from([(
            "key".to_string(),
            "a".repeat(Metadata::MAX_METADATA_VALUE_LEN as usize + 1),
        )]));

        assert!(external_canister.validate().is_err());
    }

    #[test]
    fn invalid_external_canister_validation_with_long_name() {
        let mut external_canister = mock_external_canister();
        external_canister.name = "a".repeat(ExternalCanister::MAX_NAME_LENGTH + 1);

        let result = external_canister.validate();

        assert!(result.is_err());
        let error = result.unwrap_err();
        if let ExternalCanisterError::ValidationError { info } = error {
            assert!(info.contains("name"));
            assert!(info.contains("100"));
        } else {
            panic!("Expected ValidationError");
        }
    }

    #[test]
    fn invalid_external_canister_validation_with_long_description() {
        let mut external_canister = mock_external_canister();
        external_canister.description =
            Some("a".repeat(ExternalCanister::MAX_DESCRIPTION_LENGTH + 1));

        let result = external_canister.validate();

        assert!(result.is_err());
        let error = result.unwrap_err();
        if let ExternalCanisterError::ValidationError { info } = error {
            assert!(info.contains("description"));
            assert!(info.contains("1000"));
        } else {
            panic!("Expected ValidationError");
        }
    }

    #[test]
    fn invalid_external_canister_validation_with_long_label() {
        let mut external_canister = mock_external_canister();
        external_canister.labels = vec!["a".repeat(ExternalCanister::MAX_LABEL_LENGTH + 1)];

        let result = external_canister.validate();

        assert!(result.is_err());
        let error = result.unwrap_err();
        if let ExternalCanisterError::ValidationError { info } = error {
            assert!(info.contains("label"));
            assert!(info.contains("50"));
        } else {
            panic!("Expected ValidationError");
        }
    }

    #[test]
    fn invalid_external_canister_validation_with_too_many_labels() {
        let mut external_canister = mock_external_canister();
        external_canister.labels = vec!["a".to_string(); ExternalCanister::MAX_LABELS + 1];

        let result = external_canister.validate();

        assert!(result.is_err());
        let error = result.unwrap_err();
        if let ExternalCanisterError::ValidationError { info } = error {
            assert!(info.contains("labels"));
            assert!(info.contains("10"));
        } else {
            panic!("Expected ValidationError");
        }
    }

    #[test]
    fn invalid_external_canister_validation_with_duplicate_labels() {
        let mut external_canister = mock_external_canister();
        external_canister.labels = vec!["a".to_string(), "a".to_string()];

        let result = external_canister.validate();

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
            change_metadata: Some(crate::models::ChangeMetadata::ReplaceAllBy(
                [("key".to_string(), "value".to_string())]
                    .into_iter()
                    .collect(),
            )),
            permissions: None,
            request_policies: None,
            state: Some(ExternalCanisterState::Archived),
        };

        model.update_with(changes);

        assert_eq!(model.name, "New name".to_string());
        assert_eq!(model.description, Some("New description".to_string()));
        assert_eq!(model.labels, vec!["new".to_string()]);
        assert!(model.metadata.contains(&MetadataItem {
            key: "key".to_string(),
            value: "value".to_string()
        }));
        assert_eq!(model.state, ExternalCanisterState::Archived);
    }

    #[test]
    fn fail_validation_adding_policies_associated_with_another_external_canister() {
        let mut request_policy = mock_request_policy();
        request_policy.specifier = RequestSpecifier::ChangeExternalCanister(
            ExternalCanisterId::Canister(Principal::from_slice(&[1; 29])),
        );

        REQUEST_POLICY_REPOSITORY.insert(request_policy.id, request_policy.clone());

        let validation_change_policies =
            validate_change_policies_are_associated_with_target_canister(
                &Principal::from_slice(&[2; 29]),
                &[request_policy.id],
            );

        assert!(validation_change_policies.is_err());
        assert_eq!(
            validation_change_policies.unwrap_err(),
            ExternalCanisterValidationError::ValidationError {
                info: format!(
                    "The request policy with id '{}' is not associated with the external canister.",
                    Uuid::from_bytes(request_policy.id).hyphenated()
                )
            }
        );

        let validation_calls_policies = validate_calls_policies_are_associated_with_target_canister(
            &Principal::from_slice(&[2; 29]),
            &[request_policy.id],
        );

        assert!(validation_calls_policies.is_err());
        assert_eq!(
            validation_calls_policies.unwrap_err(),
            ExternalCanisterValidationError::ValidationError {
                info: format!(
                    "The request policy with id '{}' is not associated with the external canister.",
                    Uuid::from_bytes(request_policy.id).hyphenated()
                )
            }
        );
    }

    #[test]
    fn fail_validation_adding_policies_associated_with_another_specifier() {
        let mut request_policy = mock_request_policy();
        request_policy.specifier = RequestSpecifier::AddAccount;

        REQUEST_POLICY_REPOSITORY.insert(request_policy.id, request_policy.clone());

        let validation_change_policies =
            validate_change_policies_are_associated_with_target_canister(
                &Principal::from_slice(&[1; 29]),
                &[request_policy.id],
            );

        assert!(validation_change_policies.is_err());
        assert_eq!(
            validation_change_policies.unwrap_err(),
            ExternalCanisterValidationError::ValidationError {
                info: format!(
                    "The request policy with id '{}' is not associated with the external canister.",
                    Uuid::from_bytes(request_policy.id).hyphenated()
                )
            }
        );

        let validation_calls_policies = validate_calls_policies_are_associated_with_target_canister(
            &Principal::from_slice(&[1; 29]),
            &[request_policy.id],
        );

        assert!(validation_calls_policies.is_err());
        assert_eq!(
            validation_calls_policies.unwrap_err(),
            ExternalCanisterValidationError::ValidationError {
                info: format!(
                    "The request policy with id '{}' is not associated with the external canister.",
                    Uuid::from_bytes(request_policy.id).hyphenated()
                )
            }
        );
    }

    #[test]
    fn passes_validation_adding_policies_for_the_target_external_canister() {
        let mut request_policy = mock_request_policy();
        request_policy.specifier = RequestSpecifier::ChangeExternalCanister(
            ExternalCanisterId::Canister(Principal::from_slice(&[1; 29])),
        );

        REQUEST_POLICY_REPOSITORY.insert(request_policy.id, request_policy.clone());

        let validation_change_policies =
            validate_change_policies_are_associated_with_target_canister(
                &Principal::from_slice(&[1; 29]),
                &[request_policy.id],
            );

        assert!(validation_change_policies.is_ok());

        let mut request_policy = mock_request_policy();

        request_policy.specifier =
            RequestSpecifier::CallExternalCanister(CallExternalCanisterResourceTarget {
                execution_method: ExecutionMethodResourceTarget::ExecutionMethod(CanisterMethod {
                    canister_id: Principal::from_slice(&[2; 29]),
                    method_name: "test".to_string(),
                }),
                validation_method: ValidationMethodResourceTarget::No,
            });

        REQUEST_POLICY_REPOSITORY.insert(request_policy.id, request_policy.clone());

        let validation_calls_policies = validate_calls_policies_are_associated_with_target_canister(
            &Principal::from_slice(&[2; 29]),
            &[request_policy.id],
        );

        assert!(validation_calls_policies.is_ok());
    }

    #[test]
    fn fail_creating_new_canister_with_existing_policy_ids() {
        let mut input = CreateExternalCanisterOperationInput {
            kind: CreateExternalCanisterOperationKind::CreateNew(
                CreateExternalCanisterOperationKindCreateNew {
                    initial_cycles: None,
                    subnet_selection: None,
                },
            ),
            request_policies: ExternalCanisterRequestPoliciesCreateInput {
                change: vec![ExternalCanisterChangeRequestPolicyRuleInput {
                    policy_id: Some([1; 16]),
                    rule: RequestPolicyRule::AutoApproved,
                }],
                calls: vec![],
            },
            name: "Test canister".to_string(),
            description: None,
            labels: None,
            metadata: None,
            permissions: ExternalCanisterPermissionsCreateInput {
                read: Allow::authenticated(),
                change: Allow::authenticated(),
                calls: vec![],
            },
        };

        let validation = input.validate();

        assert!(validation.is_err());
        assert_eq!(
            validation.unwrap_err(),
            ExternalCanisterValidationError::ValidationError {
                info: "The request policies cannot have policy ids when creating a new external canister.".to_string()
            }
        );

        input.request_policies.change = vec![];
        input.request_policies.calls = vec![ExternalCanisterCallRequestPolicyRuleInput {
            policy_id: Some([1; 16]),
            rule: RequestPolicyRule::AutoApproved,
            validation_method: ValidationMethodResourceTarget::No,
            execution_method: "test".to_string(),
        }];

        let validation = input.validate();

        assert!(validation.is_err());
        assert_eq!(
            validation.unwrap_err(),
            ExternalCanisterValidationError::ValidationError {
                info: "The request policies cannot have policy ids when creating a new external canister.".to_string()
            }
        );
    }
}
