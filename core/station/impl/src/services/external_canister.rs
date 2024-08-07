use super::permission::{PermissionService, PERMISSION_SERVICE};
use super::request_policy::{RequestPolicyService, REQUEST_POLICY_SERVICE};
use crate::core::ic_cdk::api::print;
use crate::core::validation::EnsureExternalCanister;
use crate::errors::ExternalCanisterError;
use crate::mappers::ExternalCanisterMapper;
use crate::models::request_specifier::RequestSpecifier;
use crate::models::resource::{
    CallExternalCanisterResourceTarget, ChangeExternalCanisterResourceTarget,
    ExecutionMethodResourceTarget, ExternalCanisterResourceAction,
    ReadExternalCanisterResourceTarget, Resource, ValidationMethodResourceTarget,
};
use crate::models::{
    AddRequestPolicyOperationInput, CanisterMethod, ConfigureExternalCanisterSettingsInput,
    CreateExternalCanisterOperationInput, CreateExternalCanisterOperationKind,
    DefiniteCanisterSettingsInput, EditPermissionOperationInput, EditRequestPolicyOperationInput,
    ExternalCanister, ExternalCanisterId, ExternalCanisterPermissionsInput,
    ExternalCanisterRequestPoliciesInput,
};
use crate::repositories::permission::{PermissionRepository, PERMISSION_REPOSITORY};
use crate::repositories::{
    ExternalCanisterRepository, RequestPolicyRepository, EXTERNAL_CANISTER_REPOSITORY,
    REQUEST_POLICY_REPOSITORY,
};
use candid::{Encode, Principal};
use ic_cdk::api::call::call_raw;
use ic_cdk::api::management_canister::main::{
    self as mgmt, delete_canister, deposit_cycles, stop_canister, update_settings,
    CanisterIdRecord, CanisterStatusResponse, CreateCanisterArgument, UpdateSettingsArgument,
};
use lazy_static::lazy_static;
use orbit_essentials::api::ServiceResult;
use orbit_essentials::model::ModelValidator;
use orbit_essentials::repository::Repository;
use orbit_essentials::types::UUID;
use std::collections::HashSet;
use std::sync::Arc;
use uuid::Uuid;

lazy_static! {
    pub static ref EXTERNAL_CANISTER_SERVICE: Arc<ExternalCanisterService> =
        Arc::new(ExternalCanisterService::new(
            Arc::clone(&EXTERNAL_CANISTER_REPOSITORY),
            Arc::clone(&PERMISSION_SERVICE),
            Arc::clone(&PERMISSION_REPOSITORY),
            Arc::clone(&REQUEST_POLICY_SERVICE),
            Arc::clone(&REQUEST_POLICY_REPOSITORY),
        ));
}

const CREATE_CANISTER_CYCLES: u128 = 100_000_000_000; // the default fee of 100 B cycles

#[derive(Default, Debug)]
pub struct ExternalCanisterService {
    external_canister_repository: Arc<ExternalCanisterRepository>,
    permission_service: Arc<PermissionService>,
    permission_repository: Arc<PermissionRepository>,
    request_policy_service: Arc<RequestPolicyService>,
    request_policy_repository: Arc<RequestPolicyRepository>,
}

impl ExternalCanisterService {
    pub fn new(
        external_canister_repository: Arc<ExternalCanisterRepository>,
        permission_service: Arc<PermissionService>,
        permission_repository: Arc<PermissionRepository>,
        request_policy_service: Arc<RequestPolicyService>,
        request_policy_repository: Arc<RequestPolicyRepository>,
    ) -> Self {
        Self {
            external_canister_repository,
            permission_service,
            permission_repository,
            request_policy_service,
            request_policy_repository,
        }
    }

    // Returns the external canister if found, otherwise an error.
    pub fn get_external_canister(
        &self,
        id: &ExternalCanisterId,
    ) -> ServiceResult<ExternalCanister> {
        let resource = self
            .external_canister_repository
            .get(&ExternalCanister::key(*id))
            .ok_or(ExternalCanisterError::NotFound {
                id: Uuid::from_bytes(*id).hyphenated().to_string(),
            })?;

        Ok(resource)
    }

    // Returns the external canister by its canister id if found, otherwise an error.
    pub fn get_external_canister_by_canister_id(
        &self,
        canister_id: &Principal,
    ) -> ServiceResult<ExternalCanister> {
        let recource_id = self
            .external_canister_repository
            .find_by_canister_id(canister_id)
            .ok_or(ExternalCanisterError::InvalidExternalCanister {
                principal: *canister_id,
            })?;

        self.get_external_canister(&recource_id)
    }

    pub async fn create_canister(
        &self,
        cycles: Option<u128>,
    ) -> ServiceResult<Principal, ExternalCanisterError> {
        let create_canister_arg = CreateCanisterArgument { settings: None };

        let canister_id = mgmt::create_canister(
            create_canister_arg,
            cycles.unwrap_or(CREATE_CANISTER_CYCLES),
        )
        .await
        .map_err(|(_, err)| ExternalCanisterError::Failed {
            reason: err.to_string(),
        })?
        .0
        .canister_id;

        Ok(canister_id)
    }

    pub async fn canister_status(
        &self,
        input: CanisterIdRecord,
    ) -> ServiceResult<CanisterStatusResponse> {
        let canister_status_arg = CanisterIdRecord {
            canister_id: input.canister_id,
        };

        let canister_status_response = mgmt::canister_status(canister_status_arg)
            .await
            .map_err(|(_, err)| ExternalCanisterError::Failed {
                reason: err.to_string(),
            })?
            .0;

        Ok(canister_status_response)
    }

    pub async fn call_external_canister(
        &self,
        canister_id: Principal,
        method_name: String,
        arg: Option<Vec<u8>>,
        cycles: Option<u64>,
    ) -> ServiceResult<Vec<u8>, ExternalCanisterError> {
        EnsureExternalCanister::is_external_canister(canister_id)?;

        call_raw(
            canister_id,
            &method_name,
            arg.unwrap_or(Encode!(&()).unwrap()),
            cycles.unwrap_or_default(),
        )
        .await
        .map_err(|(_, err)| ExternalCanisterError::Failed {
            reason: err.to_string(),
        })
    }

    /// Adds a new external canister to the system.
    ///
    /// Can be used to create another canister to a subnet or add an existing canister.
    pub async fn add_external_canister(
        &self,
        input: CreateExternalCanisterOperationInput,
    ) -> ServiceResult<ExternalCanister> {
        self.check_unique_name(input.name.clone().as_str(), None)?;
        let external_canister = match &input.kind {
            CreateExternalCanisterOperationKind::CreateNew(opts) => {
                let mut external_canister = ExternalCanisterMapper::from_create_input(
                    // The canister will be created below, but this makes sure that we can validate the
                    // model ahead of time without the canister id that will be generated.
                    Principal::anonymous(),
                    input.clone(),
                );

                external_canister.validate()?;

                // Create the canister in the subnet and update the external canister with the correct id.
                external_canister.canister_id = self
                    .create_canister(opts.initial_cycles.map(|cycles| cycles as u128))
                    .await
                    .map_err(|err| ExternalCanisterError::Failed {
                        reason: format!("failed to create external canister: {}", err),
                    })?;

                external_canister
            }
            CreateExternalCanisterOperationKind::AddExisting(opts) => {
                EnsureExternalCanister::is_external_canister(opts.canister_id)?;
                self.check_unique_canister_id(&opts.canister_id, None)?;

                let external_canister =
                    ExternalCanisterMapper::from_create_input(opts.canister_id, input.clone());

                external_canister.validate()?;

                external_canister
            }
        };

        self.external_canister_repository
            .insert(external_canister.to_key(), external_canister.clone());

        self.configure_external_canister_permissions(&external_canister, input.permissions)?;
        self.configure_external_canister_request_policies(
            &external_canister,
            input.request_policies,
        )?;

        Ok(external_canister)
    }

    /// Updates the permissions of the external canister.
    fn configure_external_canister_permissions(
        &self,
        external_canister: &ExternalCanister,
        input: ExternalCanisterPermissionsInput,
    ) -> ServiceResult<()> {
        // read permission
        self.permission_service
            .edit_permission(EditPermissionOperationInput {
                auth_scope: Some(input.read.auth_scope),
                users: Some(input.read.users),
                user_groups: Some(input.read.user_groups),
                resource: Resource::ExternalCanister(ExternalCanisterResourceAction::Read(
                    ReadExternalCanisterResourceTarget::Canister(external_canister.canister_id),
                )),
            })?;

        // change permission
        self.permission_service
            .edit_permission(EditPermissionOperationInput {
                auth_scope: Some(input.change.auth_scope),
                users: Some(input.change.users),
                user_groups: Some(input.change.user_groups),
                resource: Resource::ExternalCanister(ExternalCanisterResourceAction::Change(
                    ChangeExternalCanisterResourceTarget::Canister(external_canister.canister_id),
                )),
            })?;

        // calls permissions
        let mut updated_calls_resources = Vec::new();
        for call in input.calls {
            let call_resource = Resource::ExternalCanister(ExternalCanisterResourceAction::Call(
                CallExternalCanisterResourceTarget {
                    execution_method: ExecutionMethodResourceTarget::ExecutionMethod(
                        CanisterMethod {
                            canister_id: external_canister.canister_id,
                            method_name: call.execution_method,
                        },
                    ),
                    validation_method: call.validation_method,
                },
            ));

            self.permission_service
                .edit_permission(EditPermissionOperationInput {
                    auth_scope: Some(call.allow.auth_scope),
                    users: Some(call.allow.users),
                    user_groups: Some(call.allow.user_groups),
                    resource: call_resource.clone(),
                })?;

            updated_calls_resources.push(call_resource);
        }

        // removes outdated permissions
        self.permission_repository
            .find_external_canister_call_permissions(&external_canister.canister_id)
            .iter()
            .filter(|permission| !updated_calls_resources.contains(&permission.resource))
            .for_each(|permission| {
                self.permission_service
                    .remove_permission(&permission.resource);
            });

        Ok(())
    }

    fn configure_external_canister_request_policies(
        &self,
        external_canister: &ExternalCanister,
        input: ExternalCanisterRequestPoliciesInput,
    ) -> ServiceResult<()> {
        let current_policies: HashSet<UUID> = self
            .request_policy_repository
            .find_external_canister_policies(&external_canister.canister_id)
            .into_iter()
            .collect();

        // if the updated list of policies does not contain the current policy, remove it from the system
        let policies_to_remove = current_policies
            .iter()
            .filter(|policy_id| {
                !input
                    .calls
                    .iter()
                    .any(|policy| policy.policy_id == Some(**policy_id))
                    || !input
                        .change
                        .iter()
                        .any(|policy| policy.policy_id == Some(**policy_id))
            })
            .collect::<Vec<&UUID>>();

        for policy_id in policies_to_remove {
            self.request_policy_service
                .remove_request_policy(policy_id)?;
        }

        // add or update the `Change` policies
        for policy in input.change {
            match policy.policy_id {
                Some(policy_id) => {
                    if !current_policies.contains(&policy_id) {
                        print(format!(
                            "Policy with id {} not found for external canister {}",
                            Uuid::from_bytes(policy_id).hyphenated(),
                            external_canister.canister_id.to_text()
                        ));

                        continue;
                    }

                    self.request_policy_service.edit_request_policy(
                        EditRequestPolicyOperationInput {
                            policy_id,
                            rule: Some(policy.rule),
                            specifier: None,
                        },
                    )?;
                }
                None => {
                    self.request_policy_service.add_request_policy(
                        AddRequestPolicyOperationInput {
                            rule: policy.rule,
                            specifier: RequestSpecifier::ChangeExternalCanister(
                                ChangeExternalCanisterResourceTarget::Canister(
                                    external_canister.canister_id,
                                ),
                            ),
                        },
                    )?;
                }
            }
        }

        // add or update the `Call` policies
        for policy in input.calls {
            match policy.policy_id {
                Some(policy_id) => {
                    if !current_policies.contains(&policy_id) {
                        print(format!(
                            "Policy with id {} not found for external canister {}",
                            Uuid::from_bytes(policy_id).hyphenated(),
                            external_canister.canister_id.to_text()
                        ));

                        continue;
                    }

                    self.request_policy_service.edit_request_policy(
                        EditRequestPolicyOperationInput {
                            policy_id,
                            rule: Some(policy.rule),
                            specifier: None,
                        },
                    )?;
                }
                None => {
                    if let ValidationMethodResourceTarget::ValidationMethod(validation) =
                        &policy.validation_method
                    {
                        if validation.canister_id == external_canister.canister_id
                            && validation.method_name == policy.execution_method
                        {
                            Err(ExternalCanisterError::ValidationError {
                                info: format!("The validation method `{}` cannot be the same as the execution method.", policy.execution_method),
                            })?;
                        }
                    }

                    self.request_policy_service.add_request_policy(
                        AddRequestPolicyOperationInput {
                            rule: policy.rule,
                            specifier: RequestSpecifier::CallExternalCanister(
                                CallExternalCanisterResourceTarget {
                                    execution_method:
                                        ExecutionMethodResourceTarget::ExecutionMethod(
                                            CanisterMethod {
                                                canister_id: external_canister.canister_id,
                                                method_name: policy.execution_method,
                                            },
                                        ),
                                    validation_method: policy.validation_method,
                                },
                            ),
                        },
                    )?;
                }
            }
        }

        Ok(())
    }

    /// Edits an external canister's settings.
    pub fn edit_external_canister(
        &self,
        id: &ExternalCanisterId,
        input: ConfigureExternalCanisterSettingsInput,
    ) -> ServiceResult<ExternalCanister> {
        let mut external_canister = self.get_external_canister(id)?;

        external_canister.update_with(input.clone());
        external_canister.validate()?;

        self.external_canister_repository
            .insert(external_canister.to_key(), external_canister.clone());

        if let Some(updated_permissions) = input.permissions {
            self.configure_external_canister_permissions(&external_canister, updated_permissions)?;
        }

        if let Some(updated_request_policies) = input.request_policies {
            self.configure_external_canister_request_policies(
                &external_canister,
                updated_request_policies,
            )?;
        }

        Ok(external_canister)
    }

    /// Adds cycles to the external canister, the cycles are taken from the station's balance.
    pub async fn top_up_canister(&self, canister_id: Principal, cycles: u128) -> ServiceResult<()> {
        if let Err((err_code, err_msg)) =
            deposit_cycles(CanisterIdRecord { canister_id }, cycles).await
        {
            Err(ExternalCanisterError::Failed {
                reason: format!(
                    "Failed to top up canister {} with {} cycles, code: {:?} and reason: {:?}",
                    canister_id.to_text(),
                    cycles,
                    err_code,
                    err_msg
                ),
            })?;
        }

        Ok(())
    }

    /// Only deletes the external canister from the system.
    pub fn soft_delete_external_canister(
        &self,
        id: &ExternalCanisterId,
    ) -> ServiceResult<ExternalCanister> {
        let external_canister = self.get_external_canister(id)?;
        self.external_canister_repository
            .remove(&external_canister.to_key());

        // Removes the read & change permissions.
        self.permission_service
            .remove_permission(&Resource::ExternalCanister(
                ExternalCanisterResourceAction::Read(ReadExternalCanisterResourceTarget::Canister(
                    external_canister.canister_id,
                )),
            ));

        self.permission_service
            .remove_permission(&Resource::ExternalCanister(
                ExternalCanisterResourceAction::Change(
                    ChangeExternalCanisterResourceTarget::Canister(external_canister.canister_id),
                ),
            ));

        // Remove all permissions related to the external canister.
        self.permission_repository
            .find_external_canister_call_permissions(&external_canister.canister_id)
            .iter()
            .for_each(|permission| {
                self.permission_service
                    .remove_permission(&permission.resource);
            });

        // Remove all request policies related to the external canister.
        self.request_policy_repository
            .find_external_canister_policies(&external_canister.canister_id)
            .iter()
            .for_each(|policy_id| {
                if let Err(err) = self.request_policy_service.remove_request_policy(policy_id) {
                    // This can be ignored to not block the deletion of the external canister.
                    print(format!(
                        "Failed to remove request policy with id {}, reason: {}",
                        Uuid::from_bytes(*policy_id).hyphenated(),
                        err
                    ));
                }
            });

        Ok(external_canister)
    }

    /// Deletes an external canister from the system, as well as from the subnet.
    pub async fn hard_delete_external_canister(
        &self,
        id: &ExternalCanisterId,
    ) -> ServiceResult<ExternalCanister> {
        let external_canister = self.get_external_canister(id)?;

        // Deleting a canister requires the canister to be stopped first.
        //
        // See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-delete_canister
        if let Err((err_code, err_msg)) = stop_canister(CanisterIdRecord {
            canister_id: external_canister.canister_id,
        })
        .await
        {
            // We simply log the error and continue, this is because stopped canisters will fail this call
            // but we still want to delete the canister.
            print(format!(
                "Failed to stop canister {}, code: {:?} and reason: {:?}",
                external_canister.canister_id.to_text(),
                err_code,
                err_msg
            ));
        }

        if let Err((err_code, err_msg)) = delete_canister(CanisterIdRecord {
            canister_id: external_canister.canister_id,
        })
        .await
        {
            Err(ExternalCanisterError::Failed {
                reason: format!(
                    "Failed to delete canister {} from the subnet, code: {:?} and reason: {:?}",
                    external_canister.canister_id.to_text(),
                    err_code,
                    err_msg
                ),
            })?;
        }

        // The soft delete is done after the hard delete to ensure that the external canister
        // is removed from the subnet before it is removed from the system.
        //
        // The intercanister call is more likely to fail than the local operation.
        self.soft_delete_external_canister(id)?;

        Ok(external_canister)
    }

    /// Changes the IC settings of the external canister.
    pub async fn change_canister_ic_settings(
        &self,
        canister_id: Principal,
        settings: DefiniteCanisterSettingsInput,
    ) -> ServiceResult<()> {
        if let Err((err_code, err_msg)) = update_settings(UpdateSettingsArgument {
            canister_id,
            settings: settings.into(),
        })
        .await
        {
            Err(ExternalCanisterError::Failed {
                reason: format!(
                    "Failed to update canister {} settings, code: {:?} and reason: {:?}",
                    canister_id.to_text(),
                    err_code,
                    err_msg
                ),
            })?;
        }

        Ok(())
    }

    /// Verifies that the name is unique among external canisters.
    ///
    /// If `skip_id` is provided, it will be ignored if the match would be the same.
    fn check_unique_name(
        &self,
        name: &str,
        skip_id: Option<ExternalCanisterId>,
    ) -> ServiceResult<()> {
        if !self
            .external_canister_repository
            .is_unique_name(name, skip_id)
        {
            Err(ExternalCanisterError::ValidationError {
                info: format!("The name '{}' is already in use.", name),
            })?;
        }

        Ok(())
    }

    /// Verifies that the canister id is unique among external canisters.
    ///
    /// If `skip_id` is provided, it will be ignored if the match would be the same.
    fn check_unique_canister_id(
        &self,
        canister_id: &Principal,
        skip_id: Option<ExternalCanisterId>,
    ) -> ServiceResult<()> {
        if !self
            .external_canister_repository
            .is_unique_canister_id(canister_id, skip_id)
        {
            Err(ExternalCanisterError::ValidationError {
                info: format!("The canister id '{}' is already in use.", canister_id),
            })?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use orbit_essentials::api::ApiError;

    use super::*;
    use crate::{
        core::test_utils,
        models::{
            permission::Allow, resource::ValidationMethodResourceTarget,
            CreateExternalCanisterOperationKindAddExisting, ExternalCanisterCallPermission,
            ExternalCanisterCallRequestPolicyRuleInput,
            ExternalCanisterChangeRequestPolicyRuleInput, ExternalCanisterPermissionsInput,
            ExternalCanisterRequestPoliciesInput, RequestPolicyRule,
        },
    };

    fn setup() {
        test_utils::init_canister_system();
    }

    #[tokio::test]
    async fn test_add_external_canister() {
        setup();
        let result = EXTERNAL_CANISTER_SERVICE
            .add_external_canister(CreateExternalCanisterOperationInput {
                name: "test".to_string(),
                description: None,
                labels: None,
                permissions: ExternalCanisterPermissionsInput {
                    read: Allow::authenticated(),
                    change: Allow::authenticated(),
                    calls: vec![
                        ExternalCanisterCallPermission {
                            allow: Allow::authenticated(),
                            execution_method: "test".to_string(),
                            validation_method: ValidationMethodResourceTarget::No,
                        },
                        ExternalCanisterCallPermission {
                            allow: Allow::authenticated(),
                            execution_method: "test".to_string(),
                            validation_method: ValidationMethodResourceTarget::ValidationMethod(
                                CanisterMethod {
                                    canister_id: Principal::from_slice(&[10; 29]),
                                    method_name: "validate_test".to_string(),
                                },
                            ),
                        },
                    ],
                },
                request_policies: ExternalCanisterRequestPoliciesInput {
                    change: vec![ExternalCanisterChangeRequestPolicyRuleInput {
                        policy_id: None,
                        rule: RequestPolicyRule::AutoApproved,
                    }],
                    calls: vec![ExternalCanisterCallRequestPolicyRuleInput {
                        policy_id: None,
                        execution_method: "test".to_string(),
                        validation_method: ValidationMethodResourceTarget::No,
                        rule: RequestPolicyRule::AutoApproved,
                    }],
                },
                kind: CreateExternalCanisterOperationKind::AddExisting(
                    CreateExternalCanisterOperationKindAddExisting {
                        canister_id: Principal::from_slice(&[10; 29]),
                    },
                ),
            })
            .await;

        assert!(result.is_ok());

        let external_canister = result.unwrap();
        assert_eq!(external_canister.name, "test");
        assert_eq!(
            external_canister.canister_id,
            Principal::from_slice(&[10; 29])
        );

        let read_permission = PERMISSION_REPOSITORY
            .get(&Resource::ExternalCanister(
                ExternalCanisterResourceAction::Read(ReadExternalCanisterResourceTarget::Canister(
                    external_canister.canister_id,
                )),
            ))
            .unwrap();

        assert!(read_permission.allowed_authenticated());

        let change_permission = PERMISSION_REPOSITORY
            .get(&Resource::ExternalCanister(
                ExternalCanisterResourceAction::Change(
                    ChangeExternalCanisterResourceTarget::Canister(external_canister.canister_id),
                ),
            ))
            .unwrap();

        assert!(change_permission.allowed_authenticated());

        let call_permission = PERMISSION_REPOSITORY
            .find_external_canister_call_permissions(&external_canister.canister_id);

        assert_eq!(call_permission.len(), 2);

        for permission in call_permission {
            assert!(permission.allowed_authenticated());
        }

        let request_policies = REQUEST_POLICY_REPOSITORY
            .find_external_canister_policies(&external_canister.canister_id);

        assert_eq!(request_policies.len(), 2);

        for policy in request_policies {
            let policy = REQUEST_POLICY_REPOSITORY.get(&policy).unwrap();

            assert_eq!(policy.rule, RequestPolicyRule::AutoApproved);
        }
    }

    #[tokio::test]
    async fn add_external_canister_with_non_compatible_policy_is_ignored() {
        setup();
        let incompatible_policy = REQUEST_POLICY_SERVICE
            .add_request_policy(AddRequestPolicyOperationInput {
                rule: RequestPolicyRule::AutoApproved,
                specifier: RequestSpecifier::ChangeExternalCanister(
                    ChangeExternalCanisterResourceTarget::Canister(Principal::from_slice(&[1; 29])),
                ),
            })
            .unwrap();

        let external_canister = EXTERNAL_CANISTER_SERVICE
            .add_external_canister(CreateExternalCanisterOperationInput {
                name: "test".to_string(),
                description: None,
                labels: None,
                permissions: ExternalCanisterPermissionsInput {
                    read: Allow::authenticated(),
                    change: Allow::authenticated(),
                    calls: Vec::new(),
                },
                request_policies: ExternalCanisterRequestPoliciesInput {
                    change: vec![ExternalCanisterChangeRequestPolicyRuleInput {
                        policy_id: Some(incompatible_policy.id),
                        rule: RequestPolicyRule::AutoApproved,
                    }],
                    calls: Vec::new(),
                },
                kind: CreateExternalCanisterOperationKind::AddExisting(
                    CreateExternalCanisterOperationKindAddExisting {
                        canister_id: Principal::from_slice(&[10; 29]),
                    },
                ),
            })
            .await
            .unwrap();

        let request_policies = REQUEST_POLICY_REPOSITORY
            .find_external_canister_policies(&external_canister.canister_id);

        assert!(request_policies.is_empty());

        let policy = REQUEST_POLICY_REPOSITORY
            .get(&incompatible_policy.id)
            .unwrap();

        assert_eq!(
            policy.specifier,
            RequestSpecifier::ChangeExternalCanister(
                ChangeExternalCanisterResourceTarget::Canister(Principal::from_slice(&[1; 29])),
            )
        );
    }

    #[tokio::test]
    async fn same_validation_and_execution_method_should_fail() {
        setup();
        let result = EXTERNAL_CANISTER_SERVICE
            .add_external_canister(CreateExternalCanisterOperationInput {
                name: "test".to_string(),
                description: None,
                labels: None,
                permissions: ExternalCanisterPermissionsInput {
                    read: Allow::authenticated(),
                    change: Allow::authenticated(),
                    calls: Vec::new(),
                },
                request_policies: ExternalCanisterRequestPoliciesInput {
                    change: Vec::new(),
                    calls: vec![ExternalCanisterCallRequestPolicyRuleInput {
                        policy_id: None,
                        execution_method: "test".to_string(),
                        validation_method: ValidationMethodResourceTarget::ValidationMethod(
                            CanisterMethod {
                                canister_id: Principal::from_slice(&[10; 29]),
                                method_name: "test".to_string(),
                            },
                        ),
                        rule: RequestPolicyRule::AutoApproved,
                    }],
                },
                kind: CreateExternalCanisterOperationKind::AddExisting(
                    CreateExternalCanisterOperationKindAddExisting {
                        canister_id: Principal::from_slice(&[10; 29]),
                    },
                ),
            })
            .await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ApiError::from(ExternalCanisterError::ValidationError {
                info: "The validation method `test` cannot be the same as the execution method."
                    .to_string()
            })
        );
    }

    #[tokio::test]
    async fn fail_to_add_duplicate_name_external_canister() {
        setup();
        for i in 0..2 {
            let result = EXTERNAL_CANISTER_SERVICE
                .add_external_canister(CreateExternalCanisterOperationInput {
                    name: "test".to_string(),
                    description: None,
                    labels: None,
                    permissions: ExternalCanisterPermissionsInput {
                        read: Allow::authenticated(),
                        change: Allow::authenticated(),
                        calls: Vec::new(),
                    },
                    request_policies: ExternalCanisterRequestPoliciesInput {
                        change: Vec::new(),
                        calls: Vec::new(),
                    },
                    kind: CreateExternalCanisterOperationKind::AddExisting(
                        CreateExternalCanisterOperationKindAddExisting {
                            canister_id: Principal::from_slice(&[i; 29]),
                        },
                    ),
                })
                .await;

            match i {
                0 => assert!(result.is_ok()),
                1 => assert!(result.is_err()),
                _ => unreachable!("unexpected iteration"),
            }
        }
    }

    #[tokio::test]
    async fn fail_to_add_duplicate_canister_id_external_canister() {
        setup();
        for i in 0..2 {
            let result = EXTERNAL_CANISTER_SERVICE
                .add_external_canister(CreateExternalCanisterOperationInput {
                    name: format!("test{}", i),
                    description: None,
                    labels: None,
                    permissions: ExternalCanisterPermissionsInput {
                        read: Allow::authenticated(),
                        change: Allow::authenticated(),
                        calls: Vec::new(),
                    },
                    request_policies: ExternalCanisterRequestPoliciesInput {
                        change: Vec::new(),
                        calls: Vec::new(),
                    },
                    kind: CreateExternalCanisterOperationKind::AddExisting(
                        CreateExternalCanisterOperationKindAddExisting {
                            canister_id: Principal::from_slice(&[10; 29]),
                        },
                    ),
                })
                .await;

            match i {
                0 => assert!(result.is_ok()),
                1 => assert!(result.is_err()),
                _ => unreachable!("unexpected iteration"),
            }
        }
    }

    #[tokio::test]
    async fn test_soft_delete_of_canister() {
        setup();
        let canister = EXTERNAL_CANISTER_SERVICE
            .add_external_canister(CreateExternalCanisterOperationInput {
                name: "test".to_string(),
                description: None,
                labels: None,
                permissions: ExternalCanisterPermissionsInput {
                    read: Allow::authenticated(),
                    change: Allow::authenticated(),
                    calls: Vec::new(),
                },
                request_policies: ExternalCanisterRequestPoliciesInput {
                    change: Vec::new(),
                    calls: Vec::new(),
                },
                kind: CreateExternalCanisterOperationKind::AddExisting(
                    CreateExternalCanisterOperationKindAddExisting {
                        canister_id: Principal::from_slice(&[10; 29]),
                    },
                ),
            })
            .await
            .unwrap();

        let result = EXTERNAL_CANISTER_SERVICE.soft_delete_external_canister(&canister.id);

        assert!(result.is_ok());
        assert!(EXTERNAL_CANISTER_SERVICE
            .get_external_canister(&canister.id)
            .is_err());
    }

    #[tokio::test]
    async fn test_edit_external_canister() {
        setup();
        let canister = EXTERNAL_CANISTER_SERVICE
            .add_external_canister(CreateExternalCanisterOperationInput {
                name: "test".to_string(),
                description: None,
                labels: None,
                permissions: ExternalCanisterPermissionsInput {
                    read: Allow::authenticated(),
                    change: Allow::authenticated(),
                    calls: vec![ExternalCanisterCallPermission {
                        allow: Allow::authenticated(),
                        execution_method: "test".to_string(),
                        validation_method: ValidationMethodResourceTarget::No,
                    }],
                },
                request_policies: ExternalCanisterRequestPoliciesInput {
                    change: Vec::new(),
                    calls: Vec::new(),
                },
                kind: CreateExternalCanisterOperationKind::AddExisting(
                    CreateExternalCanisterOperationKindAddExisting {
                        canister_id: Principal::from_slice(&[10; 29]),
                    },
                ),
            })
            .await
            .unwrap();

        let updated_canister = EXTERNAL_CANISTER_SERVICE
            .edit_external_canister(
                &canister.id,
                ConfigureExternalCanisterSettingsInput {
                    name: Some("test2".to_string()),
                    description: None,
                    labels: None,
                    state: None,
                    permissions: Some(ExternalCanisterPermissionsInput {
                        read: Allow::authenticated(),
                        change: Allow::authenticated(),
                        calls: Vec::new(),
                    }),
                    request_policies: Some(ExternalCanisterRequestPoliciesInput {
                        change: Vec::new(),
                        calls: Vec::new(),
                    }),
                },
            )
            .unwrap();

        assert_eq!(updated_canister.name, "test2");

        let call_permission = PERMISSION_REPOSITORY
            .find_external_canister_call_permissions(&updated_canister.canister_id);

        assert!(call_permission.is_empty());
    }
}
