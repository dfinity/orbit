use crate::{
    core::generate_uuid_v4,
    errors::{RequestError, RequestExecuteError},
    models::{Request, RequestOperation},
    services::{
        permission::PERMISSION_SERVICE, CHANGE_CANISTER_SERVICE, DISASTER_RECOVERY_SERVICE,
        EXTERNAL_CANISTER_SERVICE, REQUEST_POLICY_SERVICE, SYSTEM_SERVICE,
    },
};
use async_trait::async_trait;
use fund_external_canister::{
    FundExternalCanisterRequestCreate, FundExternalCanisterRequestExecute,
};
use orbit_essentials::types::UUID;
use set_disaster_recovery::SetDisasterRecoveryRequestCreate;
use station_api::{CreateRequestInput, RequestOperationInput};
use std::sync::Arc;

mod add_account;
mod add_address_book_entry;
mod add_asset;
mod add_request_policy;
mod add_user;
mod add_user_group;
mod call_canister;
mod change_external_canister;
mod configure_external_canister;
mod create_canister;
mod edit_account;
mod edit_address_book_entry;
mod edit_asset;
mod edit_permission;
mod edit_request_policy;
mod edit_user;
mod edit_user_group;
mod fund_external_canister;
mod manage_system_info;
mod prune_external_canister;
mod remove_address_book_entry;
mod remove_asset;
mod remove_request_policy;
mod remove_user_group;
mod restore_external_canister;
mod set_disaster_recovery;
mod snapshot_external_canister;
mod system_upgrade;
mod transfer;

use self::{
    add_account::{AddAccountRequestCreate, AddAccountRequestExecute},
    add_address_book_entry::{AddAddressBookEntryRequestCreate, AddAddressBookEntryRequestExecute},
    add_request_policy::{AddRequestPolicyRequestCreate, AddRequestPolicyRequestExecute},
    add_user::{AddUserRequestCreate, AddUserRequestExecute},
    add_user_group::{AddUserGroupRequestCreate, AddUserGroupRequestExecute},
    call_canister::{CallExternalCanisterRequestCreate, CallExternalCanisterRequestExecute},
    change_external_canister::{
        ChangeExternalCanisterRequestCreate, ChangeExternalCanisterRequestExecute,
    },
    configure_external_canister::{
        ConfigureExternalCanisterRequestCreate, ConfigureExternalCanisterRequestExecute,
    },
    create_canister::{CreateExternalCanisterRequestCreate, CreateExternalCanisterRequestExecute},
    edit_account::{EditAccountRequestCreate, EditAccountRequestExecute},
    edit_address_book_entry::{
        EditAddressBookEntryRequestCreate, EditAddressBookEntryRequestExecute,
    },
    edit_permission::{EditPermissionRequestCreate, EditPermissionRequestExecute},
    edit_request_policy::{EditRequestPolicyRequestCreate, EditRequestPolicyRequestExecute},
    edit_user::{EditUserRequestCreate, EditUserRequestExecute},
    edit_user_group::{EditUserGroupRequestCreate, EditUserGroupRequestExecute},
    prune_external_canister::PruneExternalCanisterRequestCreate,
    prune_external_canister::PruneExternalCanisterRequestExecute,
    remove_address_book_entry::{
        RemoveAddressBookEntryRequestCreate, RemoveAddressBookEntryRequestExecute,
    },
    remove_request_policy::{RemoveRequestPolicyRequestCreate, RemoveRequestPolicyRequestExecute},
    remove_user_group::{RemoveUserGroupRequestCreate, RemoveUserGroupRequestExecute},
    restore_external_canister::RestoreExternalCanisterRequestCreate,
    restore_external_canister::RestoreExternalCanisterRequestExecute,
    snapshot_external_canister::SnapshotExternalCanisterRequestCreate,
    snapshot_external_canister::SnapshotExternalCanisterRequestExecute,
    system_upgrade::{SystemUpgradeRequestCreate, SystemUpgradeRequestExecute},
    transfer::{TransferRequestCreate, TransferRequestExecute},
};

#[derive(Debug, PartialEq, Eq)]
pub enum RequestExecuteStage {
    Completed(RequestOperation),
    Processing(RequestOperation),
}

#[async_trait]
pub trait Execute: Send + Sync {
    /// Executes the request and returns the operation that was executed with the stage that the execution is in.
    ///
    /// The stage is used to indicate if the operation was completed or if it is still processing.
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError>;
}

#[async_trait]
pub trait Create<T>: Send + Sync {
    /// Creates a new request for the operation but does not save it.
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: CreateRequestInput,
        operation_input: T,
    ) -> Result<Request, RequestError>;
}

#[derive(Debug)]
pub struct RequestFactory {}

impl RequestFactory {
    pub async fn create_request(
        requested_by_user: UUID,
        input: CreateRequestInput,
    ) -> Result<Request, RequestError> {
        let id = *generate_uuid_v4().await.as_bytes();
        match &input.operation {
            RequestOperationInput::Transfer(operation) => {
                let creator = Box::new(TransferRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::AddAccount(operation) => {
                let creator = Box::new(AddAccountRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::EditAccount(operation) => {
                let creator = Box::new(EditAccountRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::AddAddressBookEntry(operation) => {
                let creator = Box::new(AddAddressBookEntryRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::EditAddressBookEntry(operation) => {
                let creator = Box::new(EditAddressBookEntryRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::RemoveAddressBookEntry(operation) => {
                let creator = Box::new(RemoveAddressBookEntryRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::AddUserGroup(operation) => {
                let creator = Box::new(AddUserGroupRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::EditUserGroup(operation) => {
                let creator = Box::new(EditUserGroupRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::RemoveUserGroup(operation) => {
                let creator = Box::new(RemoveUserGroupRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::AddUser(operation) => {
                let creator = Box::new(AddUserRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::EditUser(operation) => {
                let creator = Box::new(EditUserRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::SystemUpgrade(operation) => {
                let creator = Box::new(SystemUpgradeRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::SetDisasterRecovery(operation) => {
                let creator = Box::new(SetDisasterRecoveryRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::ChangeExternalCanister(operation) => {
                let creator = Box::new(ChangeExternalCanisterRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::FundExternalCanister(operation) => {
                let creator = Box::new(FundExternalCanisterRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::ConfigureExternalCanister(operation) => {
                let creator = Box::new(ConfigureExternalCanisterRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::CreateExternalCanister(operation) => {
                let creator = Box::new(CreateExternalCanisterRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::CallExternalCanister(operation) => {
                let creator = Box::new(CallExternalCanisterRequestCreate {
                    external_canister_service: Arc::clone(&EXTERNAL_CANISTER_SERVICE),
                });
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::SnapshotExternalCanister(operation) => {
                let creator = Box::new(SnapshotExternalCanisterRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::RestoreExternalCanister(operation) => {
                let creator = Box::new(RestoreExternalCanisterRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::PruneExternalCanister(operation) => {
                let creator = Box::new(PruneExternalCanisterRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::EditPermission(operation) => {
                let creator = Box::new(EditPermissionRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::AddRequestPolicy(operation) => {
                let creator = Box::new(AddRequestPolicyRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::EditRequestPolicy(operation) => {
                let creator = Box::new(EditRequestPolicyRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::RemoveRequestPolicy(operation) => {
                let creator = Box::new(RemoveRequestPolicyRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::ManageSystemInfo(operation) => {
                let creator = Box::new(manage_system_info::ManageSystemInfoRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::AddAsset(operation) => {
                let creator = Box::new(add_asset::AddAssetRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::EditAsset(operation) => {
                let creator = Box::new(edit_asset::EditAssetRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
            RequestOperationInput::RemoveAsset(operation) => {
                let creator = Box::new(remove_asset::RemoveAssetRequestCreate {});
                creator
                    .create(id, requested_by_user, input.clone(), operation.clone())
                    .await
            }
        }
    }

    pub fn executor<'p>(request: &'p Request) -> Box<dyn Execute + 'p> {
        match &request.operation {
            RequestOperation::Transfer(operation) => {
                Box::new(TransferRequestExecute::new(request, operation))
            }
            RequestOperation::AddAccount(operation) => {
                Box::new(AddAccountRequestExecute::new(request, operation))
            }
            RequestOperation::EditAccount(operation) => {
                Box::new(EditAccountRequestExecute::new(request, operation))
            }
            RequestOperation::AddAddressBookEntry(operation) => {
                Box::new(AddAddressBookEntryRequestExecute::new(request, operation))
            }
            RequestOperation::EditAddressBookEntry(operation) => {
                Box::new(EditAddressBookEntryRequestExecute::new(request, operation))
            }
            RequestOperation::RemoveAddressBookEntry(operation) => Box::new(
                RemoveAddressBookEntryRequestExecute::new(request, operation),
            ),
            RequestOperation::AddUserGroup(operation) => {
                Box::new(AddUserGroupRequestExecute::new(request, operation))
            }
            RequestOperation::EditUserGroup(operation) => {
                Box::new(EditUserGroupRequestExecute::new(request, operation))
            }
            RequestOperation::RemoveUserGroup(operation) => {
                Box::new(RemoveUserGroupRequestExecute::new(request, operation))
            }
            RequestOperation::AddUser(operation) => {
                Box::new(AddUserRequestExecute::new(request, operation))
            }
            RequestOperation::EditUser(operation) => {
                Box::new(EditUserRequestExecute::new(request, operation))
            }
            RequestOperation::SetDisasterRecovery(operation) => Box::new(
                set_disaster_recovery::SetDisasterRecoveryRequestExecute::new(request, operation),
            ),
            RequestOperation::SystemUpgrade(operation) => {
                Box::new(SystemUpgradeRequestExecute::new(
                    request,
                    operation,
                    Arc::clone(&SYSTEM_SERVICE),
                    Arc::clone(&DISASTER_RECOVERY_SERVICE),
                ))
            }
            RequestOperation::ChangeExternalCanister(operation) => {
                Box::new(ChangeExternalCanisterRequestExecute::new(
                    request,
                    operation,
                    Arc::clone(&CHANGE_CANISTER_SERVICE),
                ))
            }
            RequestOperation::CreateExternalCanister(operation) => {
                Box::new(CreateExternalCanisterRequestExecute::new(
                    request,
                    operation,
                    Arc::clone(&EXTERNAL_CANISTER_SERVICE),
                ))
            }
            RequestOperation::CallExternalCanister(operation) => {
                Box::new(CallExternalCanisterRequestExecute::new(
                    request,
                    operation,
                    Arc::clone(&EXTERNAL_CANISTER_SERVICE),
                ))
            }
            RequestOperation::ConfigureExternalCanister(operation) => {
                Box::new(ConfigureExternalCanisterRequestExecute::new(
                    request,
                    operation,
                    Arc::clone(&EXTERNAL_CANISTER_SERVICE),
                ))
            }
            RequestOperation::FundExternalCanister(operation) => {
                Box::new(FundExternalCanisterRequestExecute::new(
                    request,
                    operation,
                    Arc::clone(&EXTERNAL_CANISTER_SERVICE),
                ))
            }
            RequestOperation::SnapshotExternalCanister(operation) => {
                Box::new(SnapshotExternalCanisterRequestExecute::new(
                    operation,
                    Arc::clone(&CHANGE_CANISTER_SERVICE),
                ))
            }
            RequestOperation::RestoreExternalCanister(operation) => {
                Box::new(RestoreExternalCanisterRequestExecute::new(
                    operation,
                    Arc::clone(&CHANGE_CANISTER_SERVICE),
                ))
            }
            RequestOperation::PruneExternalCanister(operation) => {
                Box::new(PruneExternalCanisterRequestExecute::new(
                    operation,
                    Arc::clone(&CHANGE_CANISTER_SERVICE),
                ))
            }
            RequestOperation::EditPermission(operation) => {
                Box::new(EditPermissionRequestExecute::new(
                    request,
                    operation,
                    Arc::clone(&PERMISSION_SERVICE),
                ))
            }
            RequestOperation::AddRequestPolicy(operation) => {
                Box::new(AddRequestPolicyRequestExecute::new(
                    request,
                    operation,
                    Arc::clone(&REQUEST_POLICY_SERVICE),
                ))
            }
            RequestOperation::EditRequestPolicy(operation) => {
                Box::new(EditRequestPolicyRequestExecute::new(
                    request,
                    operation,
                    Arc::clone(&REQUEST_POLICY_SERVICE),
                ))
            }
            RequestOperation::RemoveRequestPolicy(operation) => {
                Box::new(RemoveRequestPolicyRequestExecute::new(
                    request,
                    operation,
                    Arc::clone(&REQUEST_POLICY_SERVICE),
                ))
            }
            RequestOperation::ManageSystemInfo(operation) => Box::new(
                manage_system_info::ManageSystemInfoRequestExecute::new(request, operation),
            ),
            RequestOperation::AddAsset(operation) => {
                Box::new(add_asset::AddAssetRequestExecute::new(request, operation))
            }
            RequestOperation::EditAsset(operation) => {
                Box::new(edit_asset::EditAssetRequestExecute::new(request, operation))
            }
            RequestOperation::RemoveAsset(operation) => Box::new(
                remove_asset::RemoveAssetRequestExecute::new(request, operation),
            ),
        }
    }
}

#[cfg(test)]
pub mod requests_test_utils {
    pub fn mock_request_api_input(
        operation: station_api::RequestOperationInput,
    ) -> station_api::CreateRequestInput {
        station_api::CreateRequestInput {
            operation,
            title: None,
            summary: None,
            execution_plan: None,
            expiration_dt: None,
        }
    }
}
