use super::permission::{PermissionService, PERMISSION_SERVICE};
use super::request_policy::{RequestPolicyService, REQUEST_POLICY_SERVICE};
use crate::core::authorization::Authorization;
use crate::core::ic_cdk::api::print;
use crate::core::utils::{retain_accessible_resources, PaginatedData};
use crate::core::validation::EnsureExternalCanister;
use crate::core::CallContext;
use crate::errors::ExternalCanisterError;
use crate::mappers::ExternalCanisterMapper;
use crate::models::permission::Permission;
use crate::models::request_specifier::RequestSpecifier;
use crate::models::resource::{
    CallExternalCanisterResourceTarget, ExecutionMethodResourceTarget, ExternalCanisterId,
    ExternalCanisterResourceAction, Resource, ValidationMethodResourceTarget,
};
use crate::models::{
    AddRequestPolicyOperationInput, CanisterExecutionAndValidationMethodPairInput, CanisterMethod,
    ConfigureExternalCanisterSettingsInput, CreateExternalCanisterOperationInput,
    CreateExternalCanisterOperationKind, CycleObtainStrategy, DefiniteCanisterSettingsInput,
    EditPermissionOperationInput, EditRequestPolicyOperationInput, ExternalCanister,
    ExternalCanisterAvailableFilters, ExternalCanisterCallPermission,
    ExternalCanisterCallRequestPolicyRule, ExternalCanisterCallRequestPolicyRuleInput,
    ExternalCanisterCallerMethodsPrivileges, ExternalCanisterCallerPrivileges,
    ExternalCanisterChangeCallPermissionsInput, ExternalCanisterChangeCallRequestPoliciesInput,
    ExternalCanisterChangeRequestPolicyRule, ExternalCanisterEntryId, ExternalCanisterKey,
    ExternalCanisterMonitoring, ExternalCanisterPermissions,
    ExternalCanisterPermissionsUpdateInput, ExternalCanisterRequestPolicies,
    ExternalCanisterRequestPoliciesUpdateInput, MonitorExternalCanisterStrategy, RequestPolicy,
};
use crate::repositories::permission::{PermissionRepository, PERMISSION_REPOSITORY};
use crate::repositories::{
    ExternalCanisterRepository, ExternalCanisterWhereClause, RequestPolicyRepository,
    EXTERNAL_CANISTER_REPOSITORY, REQUEST_POLICY_REPOSITORY,
};
use crate::services::cycle_manager::{CycleManager, CYCLE_MANAGER};
use candid::{Encode, Principal};
use ic_cdk::api::call::call_raw;
use ic_cdk::api::management_canister::main::{
    self as mgmt, delete_canister, deposit_cycles, stop_canister, update_settings,
    CanisterIdRecord, CanisterStatusResponse, Snapshot, UpdateSettingsArgument,
};
use lazy_static::lazy_static;
use orbit_essentials::api::ServiceResult;
use orbit_essentials::model::ModelKey;
use orbit_essentials::model::ModelValidator;
use orbit_essentials::pagination::{paginated_items, PaginatedItemsArgs};
use orbit_essentials::repository::Repository;
use orbit_essentials::types::UUID;
use station_api::{
    GetExternalCanisterFiltersInput, GetExternalCanisterFiltersResponseNameEntry,
    ListExternalCanistersInput,
};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use uuid::Uuid;

lazy_static! {
    pub static ref EXTERNAL_CANISTER_SERVICE: Arc<ExternalCanisterService> =
        Arc::new(ExternalCanisterService::new(
            Arc::clone(&CYCLE_MANAGER),
            Arc::clone(&EXTERNAL_CANISTER_REPOSITORY),
            Arc::clone(&PERMISSION_SERVICE),
            Arc::clone(&PERMISSION_REPOSITORY),
            Arc::clone(&REQUEST_POLICY_SERVICE),
            Arc::clone(&REQUEST_POLICY_REPOSITORY),
        ));
}

const CREATE_CANISTER_CYCLES: u128 = 2_250_000_000_000; // fee sufficient to create canisters on any ICP mainnet subnet

#[derive(Default, Debug)]
pub struct ExternalCanisterService {
    cycle_manager: Arc<CycleManager>,
    external_canister_repository: Arc<ExternalCanisterRepository>,
    permission_service: Arc<PermissionService>,
    permission_repository: Arc<PermissionRepository>,
    request_policy_service: Arc<RequestPolicyService>,
    request_policy_repository: Arc<RequestPolicyRepository>,
}

impl ExternalCanisterService {
    const DEFAULT_LIST_LIMIT: u16 = 25;
    const MAX_LIST_LIMIT: u16 = 250;

    pub fn new(
        cycle_manager: Arc<CycleManager>,
        external_canister_repository: Arc<ExternalCanisterRepository>,
        permission_service: Arc<PermissionService>,
        permission_repository: Arc<PermissionRepository>,
        request_policy_service: Arc<RequestPolicyService>,
        request_policy_repository: Arc<RequestPolicyRepository>,
    ) -> Self {
        Self {
            cycle_manager,
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
        id: &ExternalCanisterEntryId,
    ) -> ServiceResult<ExternalCanister> {
        let resource = self
            .external_canister_repository
            .get(&ExternalCanisterKey { id: *id })
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
        let resource_id = self
            .external_canister_repository
            .find_by_canister_id(canister_id)
            .ok_or(ExternalCanisterError::InvalidExternalCanister {
                principal: *canister_id,
            })?;

        self.get_external_canister(&resource_id)
    }

    /// Returns all request policies of the external canister by its canister id.
    ///
    /// The policies are grouped by the type of request they are for:
    ///
    /// - `calls`: Policies for calling the external canister.
    /// - `change`: Policies for changing the external canister.
    pub fn get_external_canister_request_policies(
        &self,
        canister_id: &Principal,
    ) -> ExternalCanisterRequestPolicies {
        let policies = self
            .request_policy_repository
            .find_external_canister_policies(canister_id)
            .all()
            .iter()
            .filter_map(|policy_id| self.request_policy_repository.get(policy_id))
            .collect::<Vec<RequestPolicy>>();

        let calls = policies
            .iter()
            .filter_map(|policy| match &policy.specifier {
                RequestSpecifier::CallExternalCanister(target) => match target {
                    CallExternalCanisterResourceTarget {
                        execution_method:
                            ExecutionMethodResourceTarget::ExecutionMethod(CanisterMethod {
                                canister_id: target_canister_id,
                                method_name,
                            }),
                        validation_method,
                    } if *target_canister_id == *canister_id => {
                        Some(ExternalCanisterCallRequestPolicyRule {
                            policy_id: policy.id,
                            execution_method: method_name.clone(),
                            validation_method: validation_method.clone(),
                            rule: policy.rule.clone(),
                        })
                    }
                    _ => None,
                },
                _ => None,
            })
            .collect::<Vec<ExternalCanisterCallRequestPolicyRule>>();

        let change = policies
            .iter()
            .filter_map(|policy| match &policy.specifier {
                RequestSpecifier::ChangeExternalCanister(target) => match target {
                    ExternalCanisterId::Canister(target_canister_id)
                        if *target_canister_id == *canister_id =>
                    {
                        Some(ExternalCanisterChangeRequestPolicyRule {
                            policy_id: policy.id,
                            rule: policy.rule.clone(),
                        })
                    }
                    _ => None,
                },
                _ => None,
            })
            .collect::<Vec<ExternalCanisterChangeRequestPolicyRule>>();

        ExternalCanisterRequestPolicies { calls, change }
    }

    /// Returns the permissions of the external canister by its canister id.
    ///
    /// The permissions are grouped by the type of action they are for:
    ///
    /// - `read`: Permissions for reading the external canister.
    /// - `change`: Permissions for changing the external canister.
    /// - `calls`: Permissions for calling the external canister.
    pub fn get_external_canister_permissions(
        &self,
        canister_id: &Principal,
    ) -> ExternalCanisterPermissions {
        let read_permission = self
            .permission_service
            .get_permission(&Resource::ExternalCanister(
                ExternalCanisterResourceAction::Read(ExternalCanisterId::Canister(*canister_id)),
            ));

        let change_permission =
            self.permission_service
                .get_permission(&Resource::ExternalCanister(
                    ExternalCanisterResourceAction::Change(ExternalCanisterId::Canister(
                        *canister_id,
                    )),
                ));

        ExternalCanisterPermissions {
            read: read_permission.allow,
            change: change_permission.allow,
            calls: self.find_external_canister_call_permissions(canister_id),
        }
    }

    fn find_external_canister_call_permissions(
        &self,
        canister_id: &Principal,
    ) -> Vec<ExternalCanisterCallPermission> {
        self.permission_repository
            .find_external_canister_call_permissions(canister_id)
            .iter()
            .filter_map(|permission| match &permission.resource {
                Resource::ExternalCanister(ExternalCanisterResourceAction::Call(target)) => {
                    match &target {
                        CallExternalCanisterResourceTarget {
                            execution_method:
                                ExecutionMethodResourceTarget::ExecutionMethod(CanisterMethod {
                                    canister_id: target_canister_id,
                                    method_name,
                                }),
                            validation_method,
                        } if *target_canister_id == *canister_id => {
                            Some(ExternalCanisterCallPermission {
                                allow: permission.allow.clone(),
                                execution_method: method_name.clone(),
                                validation_method: validation_method.clone(),
                            })
                        }
                        _ => None,
                    }
                }
                _ => None,
            })
            .collect()
    }

    /// Returns the permissions of the caller for the external canister.
    pub fn get_caller_privileges_for_external_canister(
        &self,
        entry_id: &UUID,
        canister_id: &Principal,
        ctx: &CallContext,
    ) -> ExternalCanisterCallerPrivileges {
        ExternalCanisterCallerPrivileges {
            id: *entry_id,
            canister_id: *canister_id,
            can_change: Authorization::is_allowed(
                ctx,
                &Resource::ExternalCanister(ExternalCanisterResourceAction::Change(
                    ExternalCanisterId::Canister(*canister_id),
                )),
            ),
            can_fund: Authorization::is_allowed(
                ctx,
                &Resource::ExternalCanister(ExternalCanisterResourceAction::Fund(
                    ExternalCanisterId::Canister(*canister_id),
                )),
            ),
            can_call: self
                .find_external_canister_call_permissions(canister_id)
                .iter()
                .filter_map(|p| {
                    let can_call = Authorization::is_allowed(
                        ctx,
                        &Resource::ExternalCanister(ExternalCanisterResourceAction::Call(
                            CallExternalCanisterResourceTarget {
                                execution_method: ExecutionMethodResourceTarget::ExecutionMethod(
                                    CanisterMethod {
                                        canister_id: *canister_id,
                                        method_name: p.execution_method.clone(),
                                    },
                                ),
                                validation_method: p.validation_method.clone(),
                            },
                        )),
                    );

                    match can_call {
                        true => Some(ExternalCanisterCallerMethodsPrivileges {
                            validation_method: p.validation_method.clone(),
                            execution_method: p.execution_method.clone(),
                        }),
                        false => None,
                    }
                })
                .collect(),
        }
    }

    /// Lists all external canisters that match the given filters.
    ///
    /// Filters can contain:
    ///
    /// - `canister_ids`: A list of canister ids to filter by.
    /// - `labels`: A list of labels to filter by.
    /// - `paginate`: Pagination settings.
    ///
    pub fn list_external_canisters(
        &self,
        input: ListExternalCanistersInput,
        ctx: &CallContext,
    ) -> ServiceResult<PaginatedData<ExternalCanister>> {
        let mut found_ids = self.external_canister_repository.find_canister_ids_where(
            ExternalCanisterWhereClause {
                canister_ids: input.canister_ids.clone().unwrap_or_default(),
                labels: input.labels.clone().unwrap_or_default(),
                states: input
                    .states
                    .map(|states| states.into_iter().map(Into::into).collect())
                    .unwrap_or_default(),
                sort_by: input.sort_by.clone().map(Into::into),
            },
        );

        // filter out external canisters that the caller does not have access to read
        retain_accessible_resources(ctx, &mut found_ids, |id| {
            Resource::ExternalCanister(ExternalCanisterResourceAction::Read(
                ExternalCanisterId::Canister(*id),
            ))
        });

        let paginated_ids = paginated_items(PaginatedItemsArgs {
            offset: input.paginate.to_owned().and_then(|p| p.offset),
            limit: input.paginate.and_then(|p| p.limit),
            default_limit: Some(Self::DEFAULT_LIST_LIMIT),
            max_limit: Some(Self::MAX_LIST_LIMIT),
            items: &found_ids,
        })?;

        Ok(PaginatedData {
            total: paginated_ids.total,
            next_offset: paginated_ids.next_offset,
            items: paginated_ids
                .items
                .into_iter()
                .flat_map(|id| match self.get_external_canister_by_canister_id(&id) {
                    Ok(entry) => Some(entry),
                    Err(error) => {
                        print(format!(
                            "Failed to get external canister entry {}: {:?}",
                            id.to_text(),
                            error
                        ));
                        None
                    }
                })
                .collect::<Vec<ExternalCanister>>(),
        })
    }

    /// Lists the available information that facilitates filtering external canisters.
    ///
    /// These helpers can contain:
    ///
    /// - `name`: The available names of existing external canisters.
    /// - `labels`: The available labels of existing external canisters.
    pub fn available_external_canisters_filters(
        &self,
        input: GetExternalCanisterFiltersInput,
        ctx: &CallContext,
    ) -> ExternalCanisterAvailableFilters {
        let mut names = input.with_name.as_ref().map(|name| {
            self.external_canister_repository
                .find_names_by_prefix(name.prefix.clone().unwrap_or_default().as_str())
                .iter()
                .map(
                    |(name, _, canister_id)| GetExternalCanisterFiltersResponseNameEntry {
                        name: name.clone(),
                        canister_id: *canister_id,
                    },
                )
                .collect::<Vec<GetExternalCanisterFiltersResponseNameEntry>>()
        });

        // filter out names that the caller does not have access to read
        if let Some(ref mut names) = &mut names {
            retain_accessible_resources(ctx, names, |entry| {
                Resource::ExternalCanister(ExternalCanisterResourceAction::Read(
                    ExternalCanisterId::Canister(entry.canister_id),
                ))
            });

            names.truncate(Self::MAX_LIST_LIMIT as usize)
        }

        let labels = match input.with_labels {
            Some(true) => Some(self.external_canister_repository.find_all_labels()),
            _ => None,
        };

        ExternalCanisterAvailableFilters { names, labels }
    }

    /// Calls the management canister to get the status of the canister with the given id.
    ///
    /// The station needs to be a controller of the target canister.
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

    /// Calls the management canister to get the snapshots of the canister with the given id.
    ///
    /// The station needs to be a controller of the target canister.
    pub async fn canister_snapshots(
        &self,
        input: CanisterIdRecord,
    ) -> ServiceResult<Vec<Snapshot>> {
        let canister_snapshots_arg = CanisterIdRecord {
            canister_id: input.canister_id,
        };

        let canister_snapshots_response = mgmt::list_canister_snapshots(canister_snapshots_arg)
            .await
            .map_err(|(_, err)| ExternalCanisterError::Failed {
                reason: err.to_string(),
            })?
            .0;

        Ok(canister_snapshots_response)
    }

    /// Calls the target canister with the given method, argument, and cycles.
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
        self.check_unique_name(input.name.as_str(), None)?;
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
                external_canister.canister_id = orbit_essentials::cmc::create_canister(
                    opts.subnet_selection.clone(),
                    opts.initial_cycles
                        .map(|cycles| cycles as u128)
                        .unwrap_or(CREATE_CANISTER_CYCLES),
                )
                .await
                .map_err(|err| ExternalCanisterError::Failed { reason: err })?;

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
            .insert(external_canister.key(), external_canister.clone());

        self.configure_external_canister_permissions(
            &external_canister,
            // maps from create to update type to reuse the same permission configuration logic
            ExternalCanisterPermissionsUpdateInput {
                read: Some(input.permissions.read),
                change: Some(input.permissions.change),
                calls: Some(ExternalCanisterChangeCallPermissionsInput::ReplaceAllBy(
                    input.permissions.calls,
                )),
            },
        )
        .map_err(|err| {
            // remove the external canister if the permission configuration failed
            self.external_canister_repository
                .remove(&external_canister.key());

            err
        })?;
        self.configure_external_canister_request_policies(
            &external_canister,
            // maps from create to update type to reuse the same request policy configuration logic
            ExternalCanisterRequestPoliciesUpdateInput {
                change: Some(input.request_policies.change),
                calls: Some(
                    ExternalCanisterChangeCallRequestPoliciesInput::ReplaceAllBy(
                        input.request_policies.calls,
                    ),
                ),
            },
        )
        .map_err(|err| {
            // remove the external canister if the request policy configuration failed
            self.external_canister_repository
                .remove(&external_canister.key());

            err
        })?;

        Ok(external_canister)
    }

    /// Edits an external canister's settings.
    pub fn edit_external_canister(
        &self,
        id: &ExternalCanisterEntryId,
        input: ConfigureExternalCanisterSettingsInput,
    ) -> ServiceResult<ExternalCanister> {
        let mut external_canister = self.get_external_canister(id)?;

        if let Some(name) = &input.name {
            self.check_unique_name(name.as_str(), Some(external_canister.id))?;
        }

        external_canister.update_with(input.clone());
        external_canister.validate()?;

        self.external_canister_repository
            .insert(external_canister.key(), external_canister.clone());

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

    // Updates the request policies of the external canister.
    fn configure_external_canister_request_policies(
        &self,
        external_canister: &ExternalCanister,
        input: ExternalCanisterRequestPoliciesUpdateInput,
    ) -> ServiceResult<()> {
        let current_policies = self
            .request_policy_repository
            .find_external_canister_policies(&external_canister.canister_id);

        // manage the policies for the `Change` requests
        if let Some(updated_change_policies) = input.change {
            // first remove all existing policies that are not in the updated list
            let current_change_policy_ids: HashSet<_> =
                current_policies.change.iter().cloned().collect();
            let updated_change_policy_ids: HashSet<_> = updated_change_policies
                .iter()
                .filter_map(|policy| policy.policy_id)
                .collect();

            for removed_policy_id in
                current_change_policy_ids.difference(&updated_change_policy_ids)
            {
                self.request_policy_service
                    .remove_request_policy(removed_policy_id)?;
            }

            // then add or update the `Change` policies
            for updated_change_policy in updated_change_policies {
                match updated_change_policy.policy_id {
                    Some(policy_id) => {
                        // IMPORTANT: makes sure the policy exists and is associated with the target external canister
                        if !current_change_policy_ids.contains(&policy_id) {
                            return Err(ExternalCanisterError::ValidationError {
                                info: format!(
                                    "The policy with id {} does not exist for the external canister.",
                                    Uuid::from_bytes(policy_id).hyphenated()
                                ),
                            })?;
                        }

                        self.request_policy_service.edit_request_policy(
                            EditRequestPolicyOperationInput {
                                policy_id,
                                rule: Some(updated_change_policy.rule),
                                specifier: None,
                            },
                        )?;
                    }
                    None => {
                        self.request_policy_service.add_request_policy(
                            AddRequestPolicyOperationInput {
                                rule: updated_change_policy.rule,
                                specifier: RequestSpecifier::ChangeExternalCanister(
                                    ExternalCanisterId::Canister(external_canister.canister_id),
                                ),
                            },
                        )?;
                    }
                }
            }
        }

        // manage the policies for the `Call` requests
        if let Some(updated_call_policies) = input.calls {
            let current_calls_policy_ids: HashSet<_> = current_policies.calls.iter().collect();

            match &updated_call_policies {
                ExternalCanisterChangeCallRequestPoliciesInput::ReplaceAllBy(calls) => self
                    .maybe_mutate_canister_calls_request_policies(
                        external_canister,
                        &current_policies.calls,
                        calls,
                    )?,
                ExternalCanisterChangeCallRequestPoliciesInput::OverrideSpecifiedByExecutionValidationMethodPairs(calls) => {
                    let mut calls_by_method_pairs: HashMap<CanisterExecutionAndValidationMethodPairInput, HashSet<ExternalCanisterCallRequestPolicyRuleInput>> = HashMap::new();
                    calls.iter().for_each(|call| {
                        let entries = calls_by_method_pairs
                            .entry(call.method_configuration.clone())
                            .or_default();

                        call.policies.iter().for_each(|policy| {
                            entries.insert(ExternalCanisterCallRequestPolicyRuleInput {
                                policy_id: policy.policy_id,
                                rule: policy.rule.clone(),
                                execution_method: call.method_configuration.execution_method.clone(),
                                validation_method: call.method_configuration.validation_method.clone(),
                            });
                        });
                    });

                    for (method_pair, calls) in calls_by_method_pairs {
                        self.maybe_mutate_canister_calls_request_policies(
                            external_canister,
                            &self.request_policy_repository
                            .find_external_canister_call_policies_by_execution_and_validation_method(
                                &external_canister.canister_id,
                                &method_pair.execution_method,
                                &method_pair.validation_method
                            ),
                            &calls.into_iter().collect::<Vec<_>>(),
                        )?;
                    }
                }
                ExternalCanisterChangeCallRequestPoliciesInput::OverrideSpecifiedByExecutionMethods(
                    calls,
                ) => {
                    // first aggregates the calls by the execution method
                    let mut calls_by_execution_method: HashMap<String, HashSet<ExternalCanisterCallRequestPolicyRuleInput>> = HashMap::new();
                    calls.iter().for_each(|call| {
                        let entries = calls_by_execution_method
                            .entry(call.execution_method.clone())
                            .or_default();

                        call.policies.iter().for_each(|policy| {
                            entries.insert(ExternalCanisterCallRequestPolicyRuleInput {
                                policy_id: policy.policy_id,
                                rule: policy.rule.clone(),
                                execution_method: call.execution_method.clone(),
                                validation_method: policy.validation_method.clone(),
                            });
                        });
                    });

                    for (execution_method, calls) in calls_by_execution_method {
                        self.maybe_mutate_canister_calls_request_policies(
                            external_canister,
                            &self
                                .request_policy_repository
                                .find_external_canister_call_policies_by_execution_method(
                                    &external_canister.canister_id,
                                    &execution_method,
                                ),

                            &calls.into_iter().collect::<Vec<_>>(),
                        )?;
                    }
                }
                ExternalCanisterChangeCallRequestPoliciesInput::RemoveByPolicyIds(
                    policy_ids_to_remove,
                ) => {
                    for policy_id in policy_ids_to_remove {
                        // IMPORTANT: makes sure the policy exists and is associated with the target external canister
                        if !current_calls_policy_ids.contains(policy_id) {
                            return Err(ExternalCanisterError::ValidationError {
                                info: format!(
                                    "The policy with id {} does not exist for the external canister.",
                                    Uuid::from_bytes(*policy_id).hyphenated()
                                ),
                            })?;
                        }

                        self.request_policy_service
                            .remove_request_policy(policy_id)?;
                    }
                }
            };
        }

        Ok(())
    }

    // Given the current and updated call policies, this function will remove the outdated policies and
    // add or update the new ones for the external canister.
    fn maybe_mutate_canister_calls_request_policies(
        &self,
        external_canister: &ExternalCanister,
        existing_calls_policy_ids: &[UUID],
        updated_calls: &[ExternalCanisterCallRequestPolicyRuleInput],
    ) -> ServiceResult<()> {
        let current_calls_policy_ids: HashSet<_> =
            existing_calls_policy_ids.iter().cloned().collect();
        let updated_calls_policy_ids: HashSet<_> = updated_calls
            .iter()
            .filter_map(|call| call.policy_id)
            .collect();

        for removed_policy_id in current_calls_policy_ids.difference(&updated_calls_policy_ids) {
            self.request_policy_service
                .remove_request_policy(removed_policy_id)?;
        }

        for updated_call_policy in updated_calls {
            match &updated_call_policy.policy_id {
                Some(policy_id) => {
                    // IMPORTANT: makes sure the policy exists and is associated with the target external canister
                    if !current_calls_policy_ids.contains(policy_id) {
                        Err(ExternalCanisterError::ValidationError {
                            info: format!(
                                "The policy with id {} does not exist for the external canister.",
                                Uuid::from_bytes(*policy_id).hyphenated()
                            ),
                        })?;
                    }

                    self.request_policy_service.edit_request_policy(
                        EditRequestPolicyOperationInput {
                            policy_id: *policy_id,
                            rule: Some(updated_call_policy.rule.clone()),
                            specifier: None,
                        },
                    )?;
                }
                None => {
                    if let ValidationMethodResourceTarget::ValidationMethod(validation) =
                        &updated_call_policy.validation_method
                    {
                        if validation.canister_id == external_canister.canister_id
                            && validation.method_name == updated_call_policy.execution_method
                        {
                            Err(ExternalCanisterError::ValidationError {
                                info: format!(
                                    "The validation method `{}` cannot be the same as the execution method.",
                                    updated_call_policy.execution_method
                                ),
                            })?;
                        }
                    }

                    self.request_policy_service.add_request_policy(
                        AddRequestPolicyOperationInput {
                            rule: updated_call_policy.rule.clone(),
                            specifier: RequestSpecifier::CallExternalCanister(
                                CallExternalCanisterResourceTarget {
                                    execution_method:
                                        ExecutionMethodResourceTarget::ExecutionMethod(
                                            CanisterMethod {
                                                canister_id: external_canister.canister_id,
                                                method_name: updated_call_policy
                                                    .execution_method
                                                    .clone(),
                                            },
                                        ),
                                    validation_method: updated_call_policy
                                        .validation_method
                                        .clone(),
                                },
                            ),
                        },
                    )?;
                }
            }
        }

        Ok(())
    }

    /// Updates the permissions of the external canister.
    fn configure_external_canister_permissions(
        &self,
        external_canister: &ExternalCanister,
        input: ExternalCanisterPermissionsUpdateInput,
    ) -> ServiceResult<()> {
        // read permission
        if let Some(permission) = input.read {
            self.permission_service
                .edit_permission(EditPermissionOperationInput {
                    auth_scope: Some(permission.auth_scope),
                    users: Some(permission.users),
                    user_groups: Some(permission.user_groups),
                    resource: Resource::ExternalCanister(ExternalCanisterResourceAction::Read(
                        ExternalCanisterId::Canister(external_canister.canister_id),
                    )),
                })?;
        }

        // change permission (for updating the external canister settings, gives admin access to the external canister)
        if let Some(permission) = input.change {
            self.permission_service
                .edit_permission(EditPermissionOperationInput {
                    auth_scope: Some(permission.auth_scope),
                    users: Some(permission.users),
                    user_groups: Some(permission.user_groups),
                    resource: Resource::ExternalCanister(ExternalCanisterResourceAction::Change(
                        ExternalCanisterId::Canister(external_canister.canister_id),
                    )),
                })?;
        }

        // calls permissions (for calling methods of the external canister)
        if let Some(calls_permissions) = input.calls {
            match calls_permissions {
                ExternalCanisterChangeCallPermissionsInput::ReplaceAllBy(calls) => {
                    self.maybe_mutate_canister_calls_permissions(
                        external_canister,
                        &calls,
                        // always remove all existing call permissions to override them
                        |_| true,
                    )?;
                }
                ExternalCanisterChangeCallPermissionsInput::OverrideSpecifiedByExecutionValidationMethodPairs(calls) => {
                    let initial_permissions = self.permission_repository
                        .find_external_canister_call_permissions(&external_canister.canister_id)
                        .into_iter()
                        .map(|p| p.resource)
                        .collect::<HashSet<Resource>>();

                    for call in calls {
                        let call_resource = Resource::ExternalCanister(ExternalCanisterResourceAction::Call(
                            CallExternalCanisterResourceTarget {
                                execution_method: ExecutionMethodResourceTarget::ExecutionMethod(
                                    CanisterMethod {
                                        canister_id: external_canister.canister_id,
                                        method_name: call.method_configuration.execution_method,
                                    },
                                ),
                                validation_method: call.method_configuration.validation_method,
                            },
                        ));

                        match call.allow {
                            Some(allow) => {
                                self.permission_service.edit_permission(EditPermissionOperationInput {
                                    auth_scope: Some(allow.auth_scope),
                                    users: Some(allow.users),
                                    user_groups: Some(allow.user_groups),
                                    resource: call_resource,
                                })?;
                            }
                            None => {
                                if initial_permissions.contains(&call_resource) {
                                    self.permission_service.remove_permission(&call_resource);
                                }
                            }
                        }
                    }
                }
                ExternalCanisterChangeCallPermissionsInput::OverrideSpecifiedByExecutionMethods(
                    calls,
                ) => {
                    let mut calls_by_execution_method: HashMap<String, HashSet<ExternalCanisterCallPermission>> = HashMap::new();
                    calls.iter().for_each(|call| {
                        let entries = calls_by_execution_method
                            .entry(call.execution_method.clone())
                            .or_default();

                        call.permissions.iter().for_each(|policy| {
                            entries.insert(ExternalCanisterCallPermission {
                                allow: policy.allow.clone(),
                                execution_method: call.execution_method.clone(),
                                validation_method: policy.validation_method.clone(),
                            });
                        });
                    });

                    for (execution_method, calls) in calls_by_execution_method {
                        self.maybe_mutate_canister_calls_permissions(
                            external_canister,
                            &calls.into_iter().collect::<Vec<_>>(),
                            // always remove all existing call permissions to override them
                            |permission| {
                                matches!(
                                    &permission.resource,
                                    Resource::ExternalCanister(
                                        ExternalCanisterResourceAction::Call(
                                            CallExternalCanisterResourceTarget {
                                                execution_method:
                                                    ExecutionMethodResourceTarget::ExecutionMethod(
                                                        CanisterMethod {
                                                            canister_id: _,
                                                            method_name,
                                                        },
                                                    ),
                                                validation_method: _,
                                            },
                                        ),
                                    ) if *method_name == execution_method
                                )
                            },
                        )?;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn maybe_mutate_canister_calls_permissions<F>(
        &self,
        external_canister: &ExternalCanister,
        updated_calls: &[ExternalCanisterCallPermission],
        is_affected_permission: F,
    ) -> ServiceResult<()>
    where
        F: Fn(&Permission) -> bool,
    {
        let current_calls_permissions = self
            .permission_repository
            .find_external_canister_call_permissions(&external_canister.canister_id);

        // first remove all existing call permissions that are affected by the updated permission set
        for permission in current_calls_permissions
            .iter()
            .filter(|permission| is_affected_permission(permission))
        {
            self.permission_service
                .remove_permission(&permission.resource);
        }

        // adds the new call permissions
        for updated_call_permission in updated_calls {
            let updated_call_permission = updated_call_permission.clone();
            let call_resource = Resource::ExternalCanister(ExternalCanisterResourceAction::Call(
                CallExternalCanisterResourceTarget {
                    execution_method: ExecutionMethodResourceTarget::ExecutionMethod(
                        CanisterMethod {
                            canister_id: external_canister.canister_id,
                            method_name: updated_call_permission.execution_method,
                        },
                    ),
                    validation_method: updated_call_permission.validation_method,
                },
            ));

            self.permission_service
                .edit_permission(EditPermissionOperationInput {
                    auth_scope: Some(updated_call_permission.allow.auth_scope),
                    users: Some(updated_call_permission.allow.users),
                    user_groups: Some(updated_call_permission.allow.user_groups),
                    resource: call_resource,
                })?;
        }

        Ok(())
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

    /// Restarts monitoring for all external canisters after upgrade
    pub fn canister_monitor_restart(&self) {
        for canister in self.external_canister_repository.find_all() {
            if let Some(monitoring) = &canister.monitoring {
                self.cycle_manager.add_canister(
                    canister.canister_id,
                    monitoring.funding_strategy.clone(),
                    monitoring.cycle_obtain_strategy,
                );
            }
        }
    }

    pub fn canister_monitor_start(
        &self,
        canister_id: Principal,
        strategy: MonitorExternalCanisterStrategy,
        cycle_obtain_strategy: Option<CycleObtainStrategy>,
    ) -> ServiceResult<()> {
        let mut external_canister = self.get_external_canister_by_canister_id(&canister_id)?;
        if external_canister.monitoring.is_some() {
            Err(ExternalCanisterError::Failed {
                reason: format!(
                    "Failed to monitor canister {}. The canister is already monitored.",
                    canister_id.to_text(),
                ),
            })?;
        }

        self.cycle_manager
            .add_canister(canister_id, strategy.clone(), cycle_obtain_strategy);

        external_canister.monitoring = Some(ExternalCanisterMonitoring {
            funding_strategy: strategy,
            cycle_obtain_strategy,
        });

        self.external_canister_repository
            .insert(external_canister.key(), external_canister.clone());

        Ok(())
    }

    pub fn canister_monitor_stop(&self, canister_id: Principal) -> ServiceResult<()> {
        let mut external_canister = self.get_external_canister_by_canister_id(&canister_id)?;

        self.cycle_manager.remove_canister(canister_id);

        external_canister.monitoring = None;

        self.external_canister_repository
            .insert(external_canister.key(), external_canister.clone());

        Ok(())
    }

    /// Only deletes the external canister from the system.
    pub fn soft_delete_external_canister(
        &self,
        id: &ExternalCanisterEntryId,
    ) -> ServiceResult<ExternalCanister> {
        let external_canister = self.get_external_canister(id)?;
        self.external_canister_repository
            .remove(&external_canister.key());

        // Removes the read, change & fund permissions.
        self.permission_service
            .remove_permission(&Resource::ExternalCanister(
                ExternalCanisterResourceAction::Read(ExternalCanisterId::Canister(
                    external_canister.canister_id,
                )),
            ));

        self.permission_service
            .remove_permission(&Resource::ExternalCanister(
                ExternalCanisterResourceAction::Change(ExternalCanisterId::Canister(
                    external_canister.canister_id,
                )),
            ));

        self.permission_service
            .remove_permission(&Resource::ExternalCanister(
                ExternalCanisterResourceAction::Fund(ExternalCanisterId::Canister(
                    external_canister.canister_id,
                )),
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
            .all()
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
        id: &ExternalCanisterEntryId,
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
        // The inter-canister call is more likely to fail than the local operation.
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
        skip_id: Option<ExternalCanisterEntryId>,
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
        skip_id: Option<ExternalCanisterEntryId>,
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
    use std::collections::BTreeMap;

    use super::*;
    use crate::{
        core::test_utils,
        errors::ExternalCanisterValidationError,
        models::{
            permission::{Allow, AuthScope},
            resource::ValidationMethodResourceTarget,
            CanisterExecutionAndValidationMethodPairInput,
            CreateExternalCanisterOperationKindAddExisting, ExternalCanisterCallPermission,
            ExternalCanisterCallPermissionExecMethodEntryInput,
            ExternalCanisterCallPermissionMethodPairInput,
            ExternalCanisterCallPermissionsExecMethodInput,
            ExternalCanisterCallRequestPoliciesExecMethodInput,
            ExternalCanisterCallRequestPoliciesMethodPairInput,
            ExternalCanisterCallRequestPolicyRuleInput,
            ExternalCanisterCallRequestPolicyRuleValidationInput,
            ExternalCanisterChangeCallPermissionsInput,
            ExternalCanisterChangeCallRequestPoliciesInput,
            ExternalCanisterChangeRequestPolicyRuleInput, ExternalCanisterPermissionsCreateInput,
            ExternalCanisterRequestPoliciesCreateInput, Metadata, RequestPolicyRule,
        },
    };
    use orbit_essentials::api::ApiError;

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
                metadata: None,
                permissions: ExternalCanisterPermissionsCreateInput {
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
                request_policies: ExternalCanisterRequestPoliciesCreateInput {
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
                ExternalCanisterResourceAction::Read(ExternalCanisterId::Canister(
                    external_canister.canister_id,
                )),
            ))
            .unwrap();

        assert!(read_permission.allowed_authenticated());

        let change_permission = PERMISSION_REPOSITORY
            .get(&Resource::ExternalCanister(
                ExternalCanisterResourceAction::Change(ExternalCanisterId::Canister(
                    external_canister.canister_id,
                )),
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

        for policy in request_policies.all() {
            let policy = REQUEST_POLICY_REPOSITORY.get(&policy).unwrap();

            assert_eq!(policy.rule, RequestPolicyRule::AutoApproved);
        }
    }

    #[tokio::test]
    async fn add_external_canister_with_non_compatible_policy_fails_validation() {
        setup();
        let incompatible_policy = REQUEST_POLICY_SERVICE
            .add_request_policy(AddRequestPolicyOperationInput {
                rule: RequestPolicyRule::AutoApproved,
                specifier: RequestSpecifier::ChangeExternalCanister(ExternalCanisterId::Canister(
                    Principal::from_slice(&[1; 29]),
                )),
            })
            .unwrap();

        let mut create_input = CreateExternalCanisterOperationInput {
            name: "test".to_string(),
            description: None,
            labels: None,
            metadata: None,
            permissions: ExternalCanisterPermissionsCreateInput {
                read: Allow::authenticated(),
                change: Allow::authenticated(),
                calls: Vec::new(),
            },
            request_policies: ExternalCanisterRequestPoliciesCreateInput {
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
        };

        let result = EXTERNAL_CANISTER_SERVICE
            .add_external_canister(create_input.clone())
            .await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ApiError::from(ExternalCanisterValidationError::ValidationError {
                info: format!(
                    "The policy with id {} does not exist for the external canister.",
                    Uuid::from_bytes(incompatible_policy.id).hyphenated()
                )
            })
        );

        let incompatible_policy = REQUEST_POLICY_SERVICE
            .add_request_policy(AddRequestPolicyOperationInput {
                rule: RequestPolicyRule::AutoApproved,
                specifier: RequestSpecifier::AddAccount,
            })
            .unwrap();

        create_input.request_policies.change = vec![ExternalCanisterChangeRequestPolicyRuleInput {
            policy_id: Some(incompatible_policy.id),
            rule: RequestPolicyRule::AutoApproved,
        }];

        let result = EXTERNAL_CANISTER_SERVICE
            .add_external_canister(create_input.clone())
            .await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ApiError::from(ExternalCanisterValidationError::ValidationError {
                info: format!(
                    "The policy with id {} does not exist for the external canister.",
                    Uuid::from_bytes(incompatible_policy.id).hyphenated()
                )
            })
        );

        create_input.request_policies.change = vec![];
        create_input.request_policies.calls = vec![ExternalCanisterCallRequestPolicyRuleInput {
            policy_id: Some(incompatible_policy.id),
            execution_method: "test".to_string(),
            validation_method: ValidationMethodResourceTarget::No,
            rule: RequestPolicyRule::AutoApproved,
        }];

        let result = EXTERNAL_CANISTER_SERVICE
            .add_external_canister(create_input.clone())
            .await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ApiError::from(ExternalCanisterValidationError::ValidationError {
                info: format!(
                    "The policy with id {} does not exist for the external canister.",
                    Uuid::from_bytes(incompatible_policy.id).hyphenated()
                )
            })
        );
    }

    #[tokio::test]
    async fn edit_external_canister_with_non_compatible_policy_fails_validation() {
        setup();
        let incompatible_policy = REQUEST_POLICY_SERVICE
            .add_request_policy(AddRequestPolicyOperationInput {
                rule: RequestPolicyRule::AutoApproved,
                specifier: RequestSpecifier::AddAccount,
            })
            .unwrap();

        let external_canister = EXTERNAL_CANISTER_SERVICE
            .add_external_canister(CreateExternalCanisterOperationInput {
                name: "test".to_string(),
                description: None,
                labels: None,
                metadata: None,
                permissions: ExternalCanisterPermissionsCreateInput {
                    read: Allow::authenticated(),
                    change: Allow::authenticated(),
                    calls: Vec::new(),
                },
                request_policies: ExternalCanisterRequestPoliciesCreateInput {
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

        let result = EXTERNAL_CANISTER_SERVICE.edit_external_canister(
            &external_canister.id,
            ConfigureExternalCanisterSettingsInput {
                request_policies: Some(ExternalCanisterRequestPoliciesUpdateInput {
                    change: Some(vec![ExternalCanisterChangeRequestPolicyRuleInput {
                        policy_id: Some(incompatible_policy.id),
                        rule: RequestPolicyRule::AutoApproved,
                    }]),
                    calls: None,
                }),
                ..Default::default()
            },
        );

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ApiError::from(ExternalCanisterValidationError::ValidationError {
                info: format!(
                    "The policy with id {} does not exist for the external canister.",
                    Uuid::from_bytes(incompatible_policy.id).hyphenated()
                )
            })
        );

        let result = EXTERNAL_CANISTER_SERVICE.edit_external_canister(
            &external_canister.id,
            ConfigureExternalCanisterSettingsInput {
                request_policies: Some(ExternalCanisterRequestPoliciesUpdateInput {
                    change: None,
                    calls: Some(
                        ExternalCanisterChangeCallRequestPoliciesInput::ReplaceAllBy(vec![
                            ExternalCanisterCallRequestPolicyRuleInput {
                                policy_id: Some(incompatible_policy.id),
                                execution_method: "test".to_string(),
                                validation_method: ValidationMethodResourceTarget::No,
                                rule: RequestPolicyRule::AutoApproved,
                            },
                        ]),
                    ),
                }),
                ..Default::default()
            },
        );

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ApiError::from(ExternalCanisterValidationError::ValidationError {
                info: format!(
                    "The policy with id {} does not exist for the external canister.",
                    Uuid::from_bytes(incompatible_policy.id).hyphenated()
                )
            })
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
                metadata: None,
                permissions: ExternalCanisterPermissionsCreateInput {
                    read: Allow::authenticated(),
                    change: Allow::authenticated(),
                    calls: Vec::new(),
                },
                request_policies: ExternalCanisterRequestPoliciesCreateInput {
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
                    metadata: None,
                    permissions: ExternalCanisterPermissionsCreateInput {
                        read: Allow::authenticated(),
                        change: Allow::authenticated(),
                        calls: Vec::new(),
                    },
                    request_policies: ExternalCanisterRequestPoliciesCreateInput {
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
                    metadata: None,
                    permissions: ExternalCanisterPermissionsCreateInput {
                        read: Allow::authenticated(),
                        change: Allow::authenticated(),
                        calls: Vec::new(),
                    },
                    request_policies: ExternalCanisterRequestPoliciesCreateInput {
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
                metadata: None,
                permissions: ExternalCanisterPermissionsCreateInput {
                    read: Allow::authenticated(),
                    change: Allow::authenticated(),
                    calls: Vec::new(),
                },
                request_policies: ExternalCanisterRequestPoliciesCreateInput {
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
                metadata: Some(Metadata::new(BTreeMap::from([
                    ("key1".to_string(), "value1".to_string()),
                    ("key2".to_string(), "value2".to_string()),
                ]))),
                permissions: ExternalCanisterPermissionsCreateInput {
                    read: Allow::authenticated(),
                    change: Allow::authenticated(),
                    calls: vec![ExternalCanisterCallPermission {
                        allow: Allow::authenticated(),
                        execution_method: "test".to_string(),
                        validation_method: ValidationMethodResourceTarget::No,
                    }],
                },
                request_policies: ExternalCanisterRequestPoliciesCreateInput {
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

        let updated_canister =
            EXTERNAL_CANISTER_SERVICE
                .edit_external_canister(
                    &canister.id,
                    ConfigureExternalCanisterSettingsInput {
                        name: Some("test2".to_string()),
                        description: None,
                        labels: None,
                        change_metadata: Some(crate::models::ChangeMetadata::OverrideSpecifiedBy(
                            BTreeMap::from([("key2".to_string(), "test".to_string())]),
                        )),
                        state: None,
                        permissions: Some(ExternalCanisterPermissionsUpdateInput {
                            read: Some(Allow::authenticated()),
                            change: Some(Allow::authenticated()),
                            calls: Some(ExternalCanisterChangeCallPermissionsInput::ReplaceAllBy(
                                Vec::new(),
                            )),
                        }),
                        request_policies: Some(ExternalCanisterRequestPoliciesUpdateInput {
                            change: Some(Vec::new()),
                            calls: Some(
                                ExternalCanisterChangeCallRequestPoliciesInput::ReplaceAllBy(
                                    Vec::new(),
                                ),
                            ),
                        }),
                    },
                )
                .unwrap();

        assert_eq!(updated_canister.name, "test2");
        assert_eq!(
            updated_canister.metadata,
            Metadata::new(BTreeMap::from([
                ("key1".to_string(), "value1".to_string()),
                ("key2".to_string(), "test".to_string())
            ]))
        );

        let call_permission = PERMISSION_REPOSITORY
            .find_external_canister_call_permissions(&updated_canister.canister_id);

        assert!(call_permission.is_empty());
    }

    #[tokio::test]
    async fn test_edit_external_canister_only_overriding_method_pairs() {
        setup();
        let canister = EXTERNAL_CANISTER_SERVICE
            .add_external_canister(CreateExternalCanisterOperationInput {
                name: "test".to_string(),
                description: None,
                labels: None,
                metadata: None,
                permissions: ExternalCanisterPermissionsCreateInput {
                    read: Allow::authenticated(),
                    change: Allow::authenticated(),
                    calls: vec![
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
                        ExternalCanisterCallPermission {
                            allow: Allow::authenticated(),
                            execution_method: "test".to_string(),
                            validation_method: ValidationMethodResourceTarget::ValidationMethod(
                                CanisterMethod {
                                    canister_id: Principal::from_slice(&[10; 29]),
                                    method_name: "validate_test_2".to_string(),
                                },
                            ),
                        },
                        ExternalCanisterCallPermission {
                            allow: Allow::authenticated(),
                            execution_method: "to_remove_method".to_string(),
                            validation_method: ValidationMethodResourceTarget::No,
                        },
                    ],
                },
                request_policies: ExternalCanisterRequestPoliciesCreateInput {
                    change: Vec::new(),
                    calls: vec![
                        ExternalCanisterCallRequestPolicyRuleInput {
                            policy_id: None,
                            execution_method: "test".to_string(),
                            validation_method: ValidationMethodResourceTarget::ValidationMethod(
                                CanisterMethod {
                                    canister_id: Principal::from_slice(&[10; 29]),
                                    method_name: "validate_test".to_string(),
                                },
                            ),
                            rule: RequestPolicyRule::AutoApproved,
                        },
                        ExternalCanisterCallRequestPolicyRuleInput {
                            policy_id: None,
                            execution_method: "test".to_string(),
                            validation_method: ValidationMethodResourceTarget::ValidationMethod(
                                CanisterMethod {
                                    canister_id: Principal::from_slice(&[10; 29]),
                                    method_name: "validate_test_2".to_string(),
                                },
                            ),
                            rule: RequestPolicyRule::AutoApproved,
                        },
                        ExternalCanisterCallRequestPolicyRuleInput {
                            policy_id: None,
                            execution_method: "to_remove_method".to_string(),
                            validation_method: ValidationMethodResourceTarget::No,
                            rule: RequestPolicyRule::AutoApproved,
                        },
                    ],
                },
                kind: CreateExternalCanisterOperationKind::AddExisting(
                    CreateExternalCanisterOperationKindAddExisting {
                        canister_id: Principal::from_slice(&[10; 29]),
                    },
                ),
            })
            .await
            .unwrap();

        // updates a single method pair and ensure the permission and policy are updated
        let updated_method_pair = CanisterExecutionAndValidationMethodPairInput {
            execution_method: "test".to_string(),
            validation_method: ValidationMethodResourceTarget::ValidationMethod(CanisterMethod {
                canister_id: canister.canister_id,
                method_name: "validate_test".to_string(),
            }),
        };
        let to_remove_method_pair = CanisterExecutionAndValidationMethodPairInput {
            execution_method: "to_remove_method".to_string(),
            validation_method: ValidationMethodResourceTarget::No,
        };
        let target_policy_id = REQUEST_POLICY_REPOSITORY
            .find_external_canister_call_policies_by_execution_and_validation_method(
                &canister.canister_id,
                &updated_method_pair.execution_method,
                &updated_method_pair.validation_method,
            )
            .pop()
            .unwrap();

        let updated_canister =
            EXTERNAL_CANISTER_SERVICE
                .edit_external_canister(
                    &canister.id,
                    ConfigureExternalCanisterSettingsInput {
                        name: None,
                        description: None,
                        labels: None,
                        change_metadata: None,
                        state: None,
                        permissions: Some(ExternalCanisterPermissionsUpdateInput {
                            read: None,
                            change: None,
                            calls: Some(
                                ExternalCanisterChangeCallPermissionsInput::OverrideSpecifiedByExecutionValidationMethodPairs(
                                    vec![ExternalCanisterCallPermissionMethodPairInput {
                                        method_configuration: updated_method_pair.clone(),
                                        allow: Some(Allow::public()),
                                    }],
                                ),
                            ),
                        }),
                        request_policies: Some(ExternalCanisterRequestPoliciesUpdateInput {
                            change: None,
                            calls: Some(
                                ExternalCanisterChangeCallRequestPoliciesInput::OverrideSpecifiedByExecutionValidationMethodPairs(
                                    vec![ExternalCanisterCallRequestPoliciesMethodPairInput {
                                        method_configuration: updated_method_pair.clone(),
                                        policies: vec![ExternalCanisterChangeRequestPolicyRuleInput {
                                            policy_id: Some(target_policy_id),
                                            rule: RequestPolicyRule::Not(Box::new(RequestPolicyRule::AutoApproved)),
                                        }],
                                    }],
                                ),
                            ),
                        }),
                    },
                )
                .unwrap();

        let permissions = PERMISSION_REPOSITORY
            .find_external_canister_call_permissions(&updated_canister.canister_id);

        assert_eq!(permissions.len(), 3);

        let updated_permission = permissions
            .iter()
            .find(|permission| {
                matches!(
                    &permission.resource,
                    Resource::ExternalCanister(
                        ExternalCanisterResourceAction::Call(
                            CallExternalCanisterResourceTarget {
                                execution_method:
                                    ExecutionMethodResourceTarget::ExecutionMethod(
                                        CanisterMethod {
                                            canister_id: _,
                                            method_name,
                                        },
                                    ),
                                validation_method: _,
                            },
                        ),
                    ) if *method_name == updated_method_pair.execution_method
                )
            })
            .unwrap();

        assert_eq!(updated_permission.allow.auth_scope, AuthScope::Public);

        let policies = EXTERNAL_CANISTER_SERVICE
            .get_external_canister_request_policies(&updated_canister.canister_id)
            .calls;

        assert_eq!(policies.len(), 3);

        let updated_policy = policies
            .iter()
            .find(|policy| {
                policy.execution_method == updated_method_pair.execution_method
                    && policy.validation_method == updated_method_pair.validation_method
            })
            .unwrap();

        assert_eq!(
            updated_policy.rule,
            RequestPolicyRule::Not(Box::new(RequestPolicyRule::AutoApproved))
        );

        // remove the method pair and ensure the permission and policy are removed
        let updated_canister =
            EXTERNAL_CANISTER_SERVICE
                .edit_external_canister(
                    &canister.id,
                    ConfigureExternalCanisterSettingsInput {
                        name: None,
                        description: None,
                        labels: None,
                        change_metadata: None,
                        state: None,
                        permissions: Some(ExternalCanisterPermissionsUpdateInput {
                            read: None,
                            change: None,
                            calls: Some(
                                ExternalCanisterChangeCallPermissionsInput::OverrideSpecifiedByExecutionValidationMethodPairs(
                                    vec![ExternalCanisterCallPermissionMethodPairInput {
                                        method_configuration: to_remove_method_pair.clone(),
                                        allow: None,
                                    }],
                                ),
                            ),
                        }),
                        request_policies: Some(ExternalCanisterRequestPoliciesUpdateInput {
                            change: None,
                            calls: Some(
                                ExternalCanisterChangeCallRequestPoliciesInput::OverrideSpecifiedByExecutionValidationMethodPairs(
                                    vec![ExternalCanisterCallRequestPoliciesMethodPairInput {
                                        method_configuration: to_remove_method_pair.clone(),
                                        policies: Vec::new(),
                                    }],
                                ),
                            ),
                        }),
                    },
                )
                .unwrap();

        let permissions = PERMISSION_REPOSITORY
            .find_external_canister_call_permissions(&updated_canister.canister_id);

        assert_eq!(permissions.len(), 2);

        let policies = EXTERNAL_CANISTER_SERVICE
            .get_external_canister_request_policies(&updated_canister.canister_id)
            .calls;

        assert_eq!(policies.len(), 2);

        // and finally update by the execution method only, which should override the list with the new values
        let updated_canister =
            EXTERNAL_CANISTER_SERVICE
                .edit_external_canister(
                    &canister.id,
                    ConfigureExternalCanisterSettingsInput {
                        name: None,
                        description: None,
                        labels: None,
                        change_metadata: None,
                        state: None,
                        permissions: Some(ExternalCanisterPermissionsUpdateInput {
                            read: None,
                            change: None,
                            calls: Some(
                                ExternalCanisterChangeCallPermissionsInput::OverrideSpecifiedByExecutionMethods(
                                    vec![ExternalCanisterCallPermissionsExecMethodInput {
                                        execution_method: updated_method_pair.execution_method.to_string(),
                                        permissions: vec![ExternalCanisterCallPermissionExecMethodEntryInput {
                                            validation_method: ValidationMethodResourceTarget::No,
                                            allow: Allow::public(),
                                        }],
                                    }],
                                ),
                            ),
                        }),
                        request_policies: Some(ExternalCanisterRequestPoliciesUpdateInput {
                            change: None,
                            calls: Some(
                                ExternalCanisterChangeCallRequestPoliciesInput::OverrideSpecifiedByExecutionMethods(
                                    vec![ExternalCanisterCallRequestPoliciesExecMethodInput {
                                        execution_method: updated_method_pair.execution_method.to_string(),
                                        policies: vec![ExternalCanisterCallRequestPolicyRuleValidationInput {
                                            policy_id: None,
                                            rule: RequestPolicyRule::AutoApproved,
                                            validation_method: ValidationMethodResourceTarget::No,
                                        }],
                                    }],
                                ),
                            ),
                        }),
                    },
                )
                .unwrap();

        let permissions = PERMISSION_REPOSITORY
            .find_external_canister_call_permissions(&updated_canister.canister_id);

        assert_eq!(permissions.len(), 1);

        let policies = EXTERNAL_CANISTER_SERVICE
            .get_external_canister_request_policies(&updated_canister.canister_id)
            .calls;

        assert_eq!(policies.len(), 1);
    }

    #[tokio::test]
    async fn finds_all_call_permissions() {
        setup();
        for i in 0..2 {
            let _ = EXTERNAL_CANISTER_SERVICE
                .add_external_canister(CreateExternalCanisterOperationInput {
                    name: format!("test{}", i),
                    description: None,
                    labels: None,
                    metadata: None,
                    permissions: ExternalCanisterPermissionsCreateInput {
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
                                        canister_id: Principal::from_slice(&[i; 29]),
                                        method_name: "validate_test".to_string(),
                                    },
                                ),
                            },
                        ],
                    },
                    request_policies: ExternalCanisterRequestPoliciesCreateInput {
                        change: Vec::new(),
                        calls: Vec::new(),
                    },
                    kind: CreateExternalCanisterOperationKind::AddExisting(
                        CreateExternalCanisterOperationKindAddExisting {
                            canister_id: Principal::from_slice(&[i; 29]),
                        },
                    ),
                })
                .await
                .unwrap();
        }

        let permissions = EXTERNAL_CANISTER_SERVICE
            .get_external_canister_permissions(&Principal::from_slice(&[1; 29]));

        assert_eq!(permissions.read.auth_scope, AuthScope::Authenticated);
        assert_eq!(permissions.change.auth_scope, AuthScope::Authenticated);
        assert_eq!(permissions.calls.len(), 2);
    }

    #[tokio::test]
    async fn finds_request_policies_of_external_canister() {
        setup();
        for i in 0..2 {
            let _ = EXTERNAL_CANISTER_SERVICE
                .add_external_canister(CreateExternalCanisterOperationInput {
                    name: format!("test{}", i),
                    description: None,
                    labels: None,
                    metadata: None,
                    permissions: ExternalCanisterPermissionsCreateInput {
                        read: Allow::authenticated(),
                        change: Allow::authenticated(),
                        calls: Vec::new(),
                    },
                    request_policies: ExternalCanisterRequestPoliciesCreateInput {
                        change: vec![
                            ExternalCanisterChangeRequestPolicyRuleInput {
                                policy_id: None,
                                rule: RequestPolicyRule::AutoApproved,
                            },
                            ExternalCanisterChangeRequestPolicyRuleInput {
                                policy_id: None,
                                rule: RequestPolicyRule::AutoApproved,
                            },
                        ],
                        calls: vec![ExternalCanisterCallRequestPolicyRuleInput {
                            policy_id: None,
                            execution_method: "test".to_string(),
                            validation_method: ValidationMethodResourceTarget::No,
                            rule: RequestPolicyRule::AutoApproved,
                        }],
                    },
                    kind: CreateExternalCanisterOperationKind::AddExisting(
                        CreateExternalCanisterOperationKindAddExisting {
                            canister_id: Principal::from_slice(&[i; 29]),
                        },
                    ),
                })
                .await
                .unwrap();
        }

        let policies = EXTERNAL_CANISTER_SERVICE
            .get_external_canister_request_policies(&Principal::from_slice(&[1; 29]));

        assert_eq!(policies.calls.len(), 1);
        assert_eq!(policies.change.len(), 2);
    }

    #[tokio::test]
    async fn external_canister_name_is_unique() {
        setup();
        let _ = EXTERNAL_CANISTER_SERVICE
            .add_external_canister(CreateExternalCanisterOperationInput {
                name: "test".to_string(),
                description: None,
                labels: None,
                metadata: None,
                permissions: ExternalCanisterPermissionsCreateInput {
                    read: Allow::authenticated(),
                    change: Allow::authenticated(),
                    calls: Vec::new(),
                },
                request_policies: ExternalCanisterRequestPoliciesCreateInput {
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

        let result = EXTERNAL_CANISTER_SERVICE
            .add_external_canister(CreateExternalCanisterOperationInput {
                name: "test".to_string(),
                description: None,
                labels: None,
                metadata: None,
                permissions: ExternalCanisterPermissionsCreateInput {
                    read: Allow::authenticated(),
                    change: Allow::authenticated(),
                    calls: Vec::new(),
                },
                request_policies: ExternalCanisterRequestPoliciesCreateInput {
                    change: Vec::new(),
                    calls: Vec::new(),
                },
                kind: CreateExternalCanisterOperationKind::AddExisting(
                    CreateExternalCanisterOperationKindAddExisting {
                        canister_id: Principal::from_slice(&[11; 29]),
                    },
                ),
            })
            .await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ApiError::from(ExternalCanisterError::ValidationError {
                info: "The name 'test' is already in use.".to_string()
            })
        );
    }

    #[tokio::test]
    async fn edits_to_external_canister_name_still_checks_uniqueness() {
        setup();

        let mut external_canisters = Vec::new();
        for i in 0..2 {
            external_canisters.push(
                EXTERNAL_CANISTER_SERVICE
                    .add_external_canister(CreateExternalCanisterOperationInput {
                        name: format!("test{}", i),
                        description: None,
                        labels: None,
                        metadata: None,
                        permissions: ExternalCanisterPermissionsCreateInput {
                            read: Allow::authenticated(),
                            change: Allow::authenticated(),
                            calls: Vec::new(),
                        },
                        request_policies: ExternalCanisterRequestPoliciesCreateInput {
                            change: Vec::new(),
                            calls: Vec::new(),
                        },
                        kind: CreateExternalCanisterOperationKind::AddExisting(
                            CreateExternalCanisterOperationKindAddExisting {
                                canister_id: Principal::from_slice(&[i; 29]),
                            },
                        ),
                    })
                    .await
                    .unwrap(),
            );
        }

        let edit_same_is_ok = EXTERNAL_CANISTER_SERVICE
            .edit_external_canister(
                &external_canisters[0].id,
                ConfigureExternalCanisterSettingsInput {
                    name: Some(external_canisters[0].name.to_string()),
                    description: None,
                    labels: None,
                    change_metadata: None,
                    state: None,
                    permissions: None,
                    request_policies: None,
                },
            )
            .unwrap();

        assert_eq!(edit_same_is_ok.name, external_canisters[0].name);

        let edit_non_unique_name_fails = EXTERNAL_CANISTER_SERVICE.edit_external_canister(
            &external_canisters[0].id,
            ConfigureExternalCanisterSettingsInput {
                name: Some("test1".to_string()),
                description: None,
                labels: None,
                change_metadata: None,
                state: None,
                permissions: None,
                request_policies: None,
            },
        );

        assert!(edit_non_unique_name_fails.is_err());
        assert_eq!(
            edit_non_unique_name_fails.unwrap_err(),
            ApiError::from(ExternalCanisterError::ValidationError {
                info: "The name 'test1' is already in use.".to_string()
            })
        );
    }
}

#[cfg(feature = "canbench")]
mod benchs {
    use super::*;
    use crate::{models::ExternalCanisterState, services::user_service_test_utils::add_users};
    use canbench_rs::{bench, BenchResult};
    use external_canister_test_utils::add_test_external_canisters;

    #[bench(raw)]
    fn list_external_canisters_with_all_statuses() -> BenchResult {
        // creates 20 admin users with 5 groups assigned
        let admins = add_users(20, 5);
        // and 100 employees with 10 groups assigned
        let _ = add_users(100, 10);

        let first_admin = admins.first().expect("Unexpected admin not set");
        let first_admin_identity = first_admin
            .identities
            .first()
            .expect("Unexpected admin identity not available");

        let first_admin = first_admin.clone();
        let caller_identity = first_admin_identity.clone();

        // these should only be accessible to admins
        add_test_external_canisters(
            500, // adds 500 external canisters managed by the station
            10,  // with 10 individual method calls each
            ExternalCanisterState::Active,
            Some(first_admin.groups.clone()),
        );

        // these are accessible by any employee
        add_test_external_canisters(
            1500, // adds 1500 external canisters managed by the station
            5,    // with 5 individual method calls each
            ExternalCanisterState::Active,
            None,
        );

        // also adds 1000 archived external canisters
        add_test_external_canisters(
            1000, // adds 1000 external canisters managed by the station
            5,    // with 5 individual method calls each
            ExternalCanisterState::Archived,
            None,
        );

        canbench_rs::bench_fn(|| {
            let result = EXTERNAL_CANISTER_SERVICE
                .list_external_canisters(
                    ListExternalCanistersInput {
                        canister_ids: None,
                        labels: None,
                        states: Some(vec![
                            station_api::ExternalCanisterStateDTO::Active,
                            station_api::ExternalCanisterStateDTO::Archived,
                        ]),
                        paginate: Some(station_api::PaginationInput {
                            limit: Some(25),
                            offset: None,
                        }),
                        sort_by: Some(station_api::ListExternalCanistersSortInput::Name(
                            station_api::SortDirection::Asc,
                        )),
                    },
                    &CallContext::new(caller_identity),
                )
                .expect("Unexpected failed search of external canisters");

            if result.total != 3000 {
                panic!(
                    "Unexpected total count of external canisters, expected 3000, got {}",
                    result.total
                );
            }
        })
    }
}

#[cfg(feature = "canbench")]
mod external_canister_test_utils {
    use super::*;
    use crate::models::{
        external_canister_test_utils::mock_external_canister, permission::Allow,
        ExternalCanisterChangeCallPermissionsInput, ExternalCanisterState,
    };

    pub fn add_test_external_canisters(
        canisters_count: usize,
        calls_count: usize,
        state: ExternalCanisterState,
        allow_user_groups: Option<Vec<UUID>>,
    ) {
        let allow = match allow_user_groups {
            Some(groups) => Allow::user_groups(groups),
            None => Allow::authenticated(),
        };

        for _ in 0..canisters_count {
            let mut external_canister = mock_external_canister();
            external_canister.state = state.clone();
            let calls = (0..calls_count)
                .map(|i| ExternalCanisterCallPermission {
                    allow: Allow::authenticated(),
                    execution_method: format!("exec_method_{}", i),
                    validation_method: ValidationMethodResourceTarget::No,
                })
                .collect::<Vec<ExternalCanisterCallPermission>>();

            EXTERNAL_CANISTER_REPOSITORY.insert(external_canister.key(), external_canister.clone());

            let mut input = ConfigureExternalCanisterSettingsInput::default();
            input.permissions = Some(ExternalCanisterPermissionsUpdateInput {
                calls: Some(ExternalCanisterChangeCallPermissionsInput::ReplaceAllBy(
                    calls,
                )),
                read: Some(allow.clone()),
                change: Some(allow.clone()),
            });

            EXTERNAL_CANISTER_SERVICE
                .edit_external_canister(&external_canister.id, input)
                .expect("Unexpected error while configuring external canister");
        }
    }
}
