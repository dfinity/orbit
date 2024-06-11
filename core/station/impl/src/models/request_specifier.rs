use super::resource::{Resource, ResourceIds};
use super::{MetadataItem, Request, RequestId, RequestOperation, RequestOperationType};
use crate::core::validation::{
    EnsureAccount, EnsureAddressBookEntry, EnsureIdExists, EnsureRequestPolicy,
    EnsureResourceIdExists, EnsureUser, EnsureUserGroup,
};
use crate::errors::ValidationError;
use crate::models::resource::{
    CallExternalCanisterResourceTarget, ChangeExternalCanisterResourceTarget,
    CreateExternalCanisterResourceTarget, ExecutionMethodResourceTarget,
};
use crate::models::user::User;
use crate::models::{
    CallExternalCanisterOperation, ChangeExternalCanisterOperation, CreateExternalCanisterOperation,
};
use crate::repositories::ADDRESS_BOOK_REPOSITORY;
use crate::services::ACCOUNT_SERVICE;
use crate::{errors::MatchError, repositories::USER_REPOSITORY};
use orbit_essentials::model::{ModelValidator, ModelValidatorResult};
use orbit_essentials::repository::Repository;
use orbit_essentials::storable;
use orbit_essentials::types::UUID;
use std::sync::Arc;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UserSpecifier {
    Any,
    Group(Vec<UUID>),
    Id(Vec<UUID>),
}

impl ModelValidator<ValidationError> for UserSpecifier {
    fn validate(&self) -> Result<(), ValidationError> {
        match self {
            UserSpecifier::Any => Ok(()),
            UserSpecifier::Group(group_ids) => {
                for group_id in group_ids {
                    EnsureUserGroup::id_exists(group_id)?;
                }
                Ok(())
            }
            UserSpecifier::Id(user_ids) => {
                for user_id in user_ids {
                    EnsureUser::id_exists(user_id)?;
                }
                Ok(())
            }
        }
    }
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceSpecifier {
    Any,
    Resource(Resource),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RequestSpecifier {
    AddAccount,
    AddUser,
    EditAccount(ResourceIds),
    EditUser(ResourceIds),
    AddAddressBookEntry,
    EditAddressBookEntry(ResourceIds),
    RemoveAddressBookEntry(ResourceIds),
    Transfer(ResourceIds),
    ChangeCanister,
    ChangeExternalCanister(ChangeExternalCanisterResourceTarget),
    CreateExternalCanister(CreateExternalCanisterResourceTarget),
    CallExternalCanister(CallExternalCanisterResourceTarget),
    EditPermission(ResourceSpecifier),
    AddRequestPolicy,
    EditRequestPolicy(ResourceIds),
    RemoveRequestPolicy(ResourceIds),
    AddUserGroup,
    EditUserGroup(ResourceIds),
    RemoveUserGroup(ResourceIds),
    ManageSystemInfo,
}

impl ModelValidator<ValidationError> for RequestSpecifier {
    fn validate(&self) -> ModelValidatorResult<ValidationError> {
        match self {
            RequestSpecifier::AddAccount
            | RequestSpecifier::AddUser
            | RequestSpecifier::AddAddressBookEntry
            | RequestSpecifier::ChangeCanister
            | RequestSpecifier::ChangeExternalCanister(_)
            | RequestSpecifier::CreateExternalCanister(_)
            | RequestSpecifier::AddRequestPolicy
            | RequestSpecifier::ManageSystemInfo
            | RequestSpecifier::AddUserGroup => (),

            RequestSpecifier::CallExternalCanister(target) => {
                target.validate()?;
            }

            RequestSpecifier::Transfer(resource_ids)
            | RequestSpecifier::EditAccount(resource_ids) => {
                EnsureAccount::resource_ids_exist(resource_ids)?
            }
            RequestSpecifier::EditUser(resource_ids) => {
                EnsureUser::resource_ids_exist(resource_ids)?
            }
            RequestSpecifier::RemoveAddressBookEntry(resource_ids)
            | RequestSpecifier::EditAddressBookEntry(resource_ids) => {
                EnsureAddressBookEntry::resource_ids_exist(resource_ids)?
            }
            RequestSpecifier::EditPermission(resource_specifier) => match resource_specifier {
                ResourceSpecifier::Any => (),
                ResourceSpecifier::Resource(resource) => resource.validate()?,
            },

            RequestSpecifier::EditRequestPolicy(resource_ids)
            | RequestSpecifier::RemoveRequestPolicy(resource_ids) => {
                EnsureRequestPolicy::resource_ids_exist(resource_ids)?
            }
            RequestSpecifier::EditUserGroup(resource_ids)
            | RequestSpecifier::RemoveUserGroup(resource_ids) => {
                EnsureUserGroup::resource_ids_exist(resource_ids)?
            }
        }
        Ok(())
    }
}

impl From<&RequestSpecifier> for RequestOperationType {
    fn from(value: &RequestSpecifier) -> Self {
        match value {
            RequestSpecifier::AddAccount => RequestOperationType::AddAccount,
            RequestSpecifier::AddUser => RequestOperationType::AddUser,
            RequestSpecifier::EditAccount(_) => RequestOperationType::EditAccount,
            RequestSpecifier::EditUser(_) => RequestOperationType::EditUser,
            RequestSpecifier::AddAddressBookEntry => RequestOperationType::AddAddressBookEntry,
            RequestSpecifier::EditAddressBookEntry(_) => RequestOperationType::EditAddressBookEntry,
            RequestSpecifier::RemoveAddressBookEntry(_) => {
                RequestOperationType::RemoveAddressBookEntry
            }
            RequestSpecifier::Transfer(_) => RequestOperationType::Transfer,
            RequestSpecifier::EditPermission(_) => RequestOperationType::EditPermission,
            RequestSpecifier::ChangeCanister => RequestOperationType::ChangeCanister,
            RequestSpecifier::ChangeExternalCanister(_) => {
                RequestOperationType::ChangeExternalCanister
            }
            RequestSpecifier::CreateExternalCanister(_) => {
                RequestOperationType::CreateExternalCanister
            }
            RequestSpecifier::CallExternalCanister(_) => RequestOperationType::CallExternalCanister,
            RequestSpecifier::AddRequestPolicy => RequestOperationType::AddRequestPolicy,
            RequestSpecifier::EditRequestPolicy(_) => RequestOperationType::EditRequestPolicy,
            RequestSpecifier::RemoveRequestPolicy(_) => RequestOperationType::RemoveRequestPolicy,
            RequestSpecifier::AddUserGroup => RequestOperationType::AddUserGroup,
            RequestSpecifier::EditUserGroup(_) => RequestOperationType::EditUserGroup,
            RequestSpecifier::RemoveUserGroup(_) => RequestOperationType::RemoveUserGroup,
            RequestSpecifier::ManageSystemInfo => RequestOperationType::ManageSystemInfo,
        }
    }
}

pub trait Match<T>: Sync + Send {
    fn is_match(&self, v: T) -> Result<bool, MatchError>;
}

#[derive(Clone)]
pub struct AccountMatcher;

impl Match<(Request, UUID, ResourceIds)> for AccountMatcher {
    fn is_match(&self, v: (Request, UUID, ResourceIds)) -> Result<bool, MatchError> {
        let (_, account_id, specifier) = v;

        match specifier {
            ResourceIds::Any => Ok(true),
            ResourceIds::Ids(ids) => Ok(ids.contains(&account_id)),
        }
    }
}

#[derive(Clone)]
pub struct CommonIdMatcher;

impl Match<(Request, UUID, ResourceIds)> for CommonIdMatcher {
    fn is_match(&self, v: (Request, UUID, ResourceIds)) -> Result<bool, MatchError> {
        let (_, entity_id, specifier) = v;

        match specifier {
            ResourceIds::Any => Ok(true),
            ResourceIds::Ids(ids) => Ok(ids.contains(&entity_id)),
        }
    }
}

#[derive(Clone)]
pub struct UserMatcher;

pub struct UserInvolvedInPolicyRuleForRequestResource {
    pub request_operation_resources: Vec<Resource>,
    pub policy_rule_user_specifier: UserSpecifier,
    pub user_id: UUID,
    pub request_id: RequestId,
}

impl Match<UserInvolvedInPolicyRuleForRequestResource> for UserMatcher {
    fn is_match(
        &self,
        input: UserInvolvedInPolicyRuleForRequestResource,
    ) -> Result<bool, MatchError> {
        match input.policy_rule_user_specifier {
            UserSpecifier::Any => Ok(true),
            UserSpecifier::Group(ids) => {
                if let Some(user) = USER_REPOSITORY.get(&User::key(input.user_id)) {
                    return Ok(user.groups.iter().any(|g| ids.contains(g)));
                }

                Ok(false)
            }
            UserSpecifier::Id(ids) => Ok(ids.contains(&input.user_id)),
        }
    }
}

#[derive(Clone)]
pub struct RequestMatcher {
    pub account_matcher: Arc<dyn Match<(Request, UUID, ResourceIds)>>,
    pub user_matcher: Arc<dyn Match<UserInvolvedInPolicyRuleForRequestResource>>,
    pub common_id_matcher: Arc<dyn Match<(Request, UUID, ResourceIds)>>,
}

impl Match<(Request, RequestSpecifier)> for RequestMatcher {
    fn is_match(&self, v: (Request, RequestSpecifier)) -> Result<bool, MatchError> {
        let (p, s) = v;

        Ok(match (p.operation.to_owned(), s.to_owned()) {
            (RequestOperation::AddAccount(_), RequestSpecifier::AddAccount) => true,
            (RequestOperation::AddUser(_), RequestSpecifier::AddUser) => true,
            (RequestOperation::EditAccount(params), RequestSpecifier::EditAccount(account)) => self
                .account_matcher
                .is_match((p, params.input.account_id, account))?,
            (RequestOperation::EditUser(params), RequestSpecifier::EditUser(user)) => self
                .user_matcher
                .is_match(UserInvolvedInPolicyRuleForRequestResource {
                    request_operation_resources: p.operation.to_resources(),
                    policy_rule_user_specifier: match user {
                        ResourceIds::Any => UserSpecifier::Any,
                        ResourceIds::Ids(ids) => UserSpecifier::Id(ids),
                    },
                    user_id: params.input.user_id,
                    request_id: p.id,
                })?,
            (RequestOperation::AddAddressBookEntry(_), RequestSpecifier::AddAddressBookEntry) => {
                true
            }
            (
                RequestOperation::EditAddressBookEntry(params),
                RequestSpecifier::EditAddressBookEntry(address_book_entry),
            ) => self.common_id_matcher.is_match((
                p,
                params.input.address_book_entry_id,
                address_book_entry,
            ))?,
            (
                RequestOperation::RemoveAddressBookEntry(params),
                RequestSpecifier::RemoveAddressBookEntry(address_book_entry),
            ) => self.common_id_matcher.is_match((
                p,
                params.input.address_book_entry_id,
                address_book_entry,
            ))?,
            (RequestOperation::Transfer(params), RequestSpecifier::Transfer(account)) => self
                .account_matcher
                .is_match((p.clone(), params.input.from_account_id, account))?,
            (RequestOperation::ChangeCanister(_), RequestSpecifier::ChangeCanister) => true,
            (
                RequestOperation::ChangeExternalCanister(ChangeExternalCanisterOperation {
                    input,
                    ..
                }),
                RequestSpecifier::ChangeExternalCanister(specifier),
            ) => match specifier {
                ChangeExternalCanisterResourceTarget::Any => true,
                ChangeExternalCanisterResourceTarget::Canister(target_id) => {
                    input.canister_id == target_id
                }
            },
            (
                RequestOperation::CreateExternalCanister(CreateExternalCanisterOperation {
                    ..
                }),
                RequestSpecifier::CreateExternalCanister(specifier),
            ) => match specifier {
                CreateExternalCanisterResourceTarget::Any => true,
            },
            (
                RequestOperation::CallExternalCanister(CallExternalCanisterOperation {
                    input, ..
                }),
                RequestSpecifier::CallExternalCanister(specifier),
            ) => match specifier.execution_method {
                ExecutionMethodResourceTarget::Any => true,
                ExecutionMethodResourceTarget::ExecutionMethod(canister_method) => {
                    input.execution_method.canister_id == canister_method.canister_id
                }
            },
            (RequestOperation::AddUserGroup(_), RequestSpecifier::AddUserGroup) => true,
            (
                RequestOperation::EditPermission(operation),
                RequestSpecifier::EditPermission(specifier),
            ) => match specifier {
                ResourceSpecifier::Any => true,
                ResourceSpecifier::Resource(resource) => resource == operation.input.resource,
            },
            (RequestOperation::AddRequestPolicy(_), RequestSpecifier::AddRequestPolicy) => true,
            (
                RequestOperation::EditRequestPolicy(operation),
                RequestSpecifier::EditRequestPolicy(specifier),
            ) => self
                .common_id_matcher
                .is_match((p, operation.input.policy_id, specifier))?,
            (
                RequestOperation::RemoveRequestPolicy(operation),
                RequestSpecifier::RemoveRequestPolicy(specifier),
            ) => self
                .common_id_matcher
                .is_match((p, operation.input.policy_id, specifier))?,
            (
                RequestOperation::EditUserGroup(operation),
                RequestSpecifier::EditUserGroup(specifier),
            ) => self
                .common_id_matcher
                .is_match((p, operation.input.user_group_id, specifier))?,
            (
                RequestOperation::RemoveUserGroup(operation),
                RequestSpecifier::RemoveUserGroup(specifier),
            ) => self
                .common_id_matcher
                .is_match((p, operation.input.user_group_id, specifier))?,
            // this is here to make sure that new operations are not added without updating this
            (RequestOperation::AddAccount(_), _)
            | (RequestOperation::AddUser(_), _)
            | (RequestOperation::EditAccount(_), _)
            | (RequestOperation::EditUser(_), _)
            | (RequestOperation::AddAddressBookEntry(_), _)
            | (RequestOperation::EditAddressBookEntry(_), _)
            | (RequestOperation::RemoveAddressBookEntry(_), _)
            | (RequestOperation::ChangeCanister(_), _)
            | (RequestOperation::ChangeExternalCanister(_), _)
            | (RequestOperation::CreateExternalCanister(_), _)
            | (RequestOperation::CallExternalCanister(_), _)
            | (RequestOperation::AddRequestPolicy(_), _)
            | (RequestOperation::EditRequestPolicy(_), _)
            | (RequestOperation::EditPermission(_), _)
            | (RequestOperation::EditUserGroup(_), _)
            | (RequestOperation::RemoveUserGroup(_), _)
            | (RequestOperation::RemoveRequestPolicy(_), _)
            | (RequestOperation::AddUserGroup(_), _)
            | (RequestOperation::ManageSystemInfo(_), _)
            | (RequestOperation::Transfer(_), _) => false,
        })
    }
}

#[derive(Clone)]
pub struct AddressBookMetadataMatcher;

pub type RequestHasMetadata = (Request, MetadataItem);

impl Match<RequestHasMetadata> for AddressBookMetadataMatcher {
    fn is_match(&self, v: RequestHasMetadata) -> Result<bool, MatchError> {
        let (request, metadata) = v;

        Ok(match request.operation.to_owned() {
            RequestOperation::Transfer(transfer) => {
                if let Ok(account) = ACCOUNT_SERVICE.get_account(&transfer.input.from_account_id) {
                    if let Some(address_book_entry) = ADDRESS_BOOK_REPOSITORY.find_by_address(
                        account.blockchain,
                        account.standard,
                        transfer.input.to,
                    ) {
                        address_book_entry.metadata.contains(&metadata)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            _ => false,
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        core::validation::disable_mock_resource_validation,
        core::write_system_info,
        models::{
            permission::Allow,
            request_policy_rule::RequestPolicyRule,
            request_specifier::{
                AccountMatcher, Match, RequestMatcher, RequestSpecifier,
                UserInvolvedInPolicyRuleForRequestResource, UserMatcher, UserSpecifier,
            },
            request_test_utils::mock_request,
            resource::{
                CallExternalCanisterResourceTarget, ChangeExternalCanisterResourceTarget,
                CreateExternalCanisterResourceTarget, ExecutionMethodResourceTarget, ResourceIds,
                ValidationMethodResourceTarget,
            },
            system::SystemInfo,
            AddAccountOperation, AddAccountOperationInput, AddUserOperation, AddUserOperationInput,
            Blockchain, CanisterMethod, EditAccountOperation, EditAccountOperationInput,
            EditUserOperation, EditUserOperationInput, Metadata, RequestKey, RequestOperation,
            TransferOperation, TransferOperationInput, UserStatus,
        },
        repositories::REQUEST_REPOSITORY,
    };
    use anyhow::{anyhow, Error};
    use candid::{Nat, Principal};
    use orbit_essentials::cdk::mocks::api::id;
    use orbit_essentials::cdk::mocks::TEST_CANISTER_ID;
    use orbit_essentials::{model::ModelValidator, repository::Repository};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_request_matcher_empty_request() -> Result<(), Error> {
        let m = RequestMatcher {
            account_matcher: Arc::new(AccountMatcher),
            user_matcher: Arc::new(UserMatcher),
            common_id_matcher: Arc::new(AccountMatcher),
        };

        let tcs = vec![
            (
                RequestOperation::AddAccount(AddAccountOperation {
                    account_id: None,
                    input: AddAccountOperationInput {
                        name: "account-1".into(),
                        blockchain: Blockchain::InternetComputer,
                        standard: crate::models::BlockchainStandard::Native,
                        metadata: Metadata::default(),
                        transfer_request_policy: Some(RequestPolicyRule::AutoApproved),
                        configs_request_policy: Some(RequestPolicyRule::AutoApproved),
                        read_permission: Allow::authenticated(),
                        configs_permission: Allow::authenticated(),
                        transfer_permission: Allow::authenticated(),
                    },
                }),
                RequestSpecifier::AddAccount,
            ),
            (
                RequestOperation::AddUser(AddUserOperation {
                    user_id: None,
                    input: AddUserOperationInput {
                        name: "user-1".to_string(),
                        identities: vec![],
                        groups: vec![],
                        status: UserStatus::Active,
                    },
                }),
                RequestSpecifier::AddUser,
            ),
            (
                RequestOperation::EditAccount(EditAccountOperation {
                    input: EditAccountOperationInput {
                        account_id: [0; 16],
                        name: None,
                        read_permission: None,
                        configs_permission: None,
                        transfer_permission: None,
                        transfer_request_policy: None,
                        configs_request_policy: None,
                    },
                }),
                RequestSpecifier::EditAccount(ResourceIds::Any),
            ),
            (
                RequestOperation::EditUser(EditUserOperation {
                    input: EditUserOperationInput {
                        user_id: [0; 16],
                        name: None,
                        identities: None,
                        groups: None,
                        status: None,
                    },
                }),
                RequestSpecifier::EditUser(ResourceIds::Any),
            ),
            (
                RequestOperation::Transfer(TransferOperation {
                    transfer_id: None,
                    input: TransferOperationInput {
                        from_account_id: [0; 16],
                        to: "address-1".into(),
                        amount: Nat::from(1_u64),
                        metadata: Metadata::default(),
                        network: "network-1".into(),
                        fee: None,
                    },
                }),
                RequestSpecifier::Transfer(ResourceIds::Any),
            ),
        ];

        for tc in tcs {
            let mut request = mock_request();
            request.operation = tc.0;

            let specifier = tc.1;

            if !m.is_match((request, specifier))? {
                return Err(anyhow!("expected true but got false"));
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_user_matcher() {
        let m = UserMatcher;

        let tcs = vec![
            (
                [0; 16],            // requester
                [1; 16],            // approver
                UserSpecifier::Any, // specifier
            ),
            (
                [0; 16],                          // requester
                [1; 16],                          // approver
                UserSpecifier::Id(vec![[1; 16]]), // specifier
            ),
        ];

        for tc in tcs {
            let mut request = mock_request();
            request.requested_by = tc.0;
            REQUEST_REPOSITORY.insert(RequestKey { id: request.id }, request.clone());

            let approver = tc.1;
            let specifier = tc.2;

            assert!(m
                .is_match(UserInvolvedInPolicyRuleForRequestResource {
                    request_operation_resources: request.operation.to_resources(),
                    policy_rule_user_specifier: specifier,
                    user_id: approver,
                    request_id: request.id,
                })
                .expect("Could not test user matcher"));
        }
    }

    #[test]
    fn test_valid_user_specifier() {
        disable_mock_resource_validation();

        UserSpecifier::Any.validate().expect("Any should be valid");
    }

    #[test]
    fn fail_invalid_user_specifier() {
        disable_mock_resource_validation();

        UserSpecifier::Id(vec![[0; 16]])
            .validate()
            .expect_err("Non existent user ID should be invalid");
        UserSpecifier::Group(vec![[0; 16]])
            .validate()
            .expect_err("Non existent group ID should be invalid");
    }

    #[test]
    fn test_valid_request_specifier() {
        disable_mock_resource_validation();

        // needed to validate call external canister request specifiers
        let station_canister_id = TEST_CANISTER_ID;
        assert_eq!(station_canister_id, id());
        let upgrader_canister_id =
            Principal::from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01]);
        let external_canister_id =
            Principal::from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01]);
        let ledger_canister_id =
            Principal::from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x01, 0x01]);
        let system_info = SystemInfo::new(upgrader_canister_id, Vec::new());
        write_system_info(system_info);

        RequestSpecifier::AddAccount
            .validate()
            .expect("AddAccount should be valid");
        RequestSpecifier::AddUser
            .validate()
            .expect("AddUser should be valid");
        RequestSpecifier::AddAddressBookEntry
            .validate()
            .expect("AddAddressBookEntry should be valid");
        RequestSpecifier::ChangeCanister
            .validate()
            .expect("ChangeCanister should be valid");
        RequestSpecifier::ChangeExternalCanister(ChangeExternalCanisterResourceTarget::Any)
            .validate()
            .expect("ChangeExternalCanister should be valid");
        RequestSpecifier::ChangeExternalCanister(ChangeExternalCanisterResourceTarget::Canister(
            Principal::management_canister(),
        ))
        .validate()
        .expect("ChangeExternalCanister should be valid");
        RequestSpecifier::CreateExternalCanister(CreateExternalCanisterResourceTarget::Any)
            .validate()
            .expect("CreateExternalCanister should be valid");
        RequestSpecifier::CallExternalCanister(CallExternalCanisterResourceTarget {
            validation_method: ValidationMethodResourceTarget::No,
            execution_method: ExecutionMethodResourceTarget::Any,
        })
        .validate()
        .expect("CallExternalCanister should be valid");
        RequestSpecifier::CallExternalCanister(CallExternalCanisterResourceTarget {
            validation_method: ValidationMethodResourceTarget::ValidationMethod(CanisterMethod {
                canister_id: Principal::management_canister(),
                method_name: "install_code".to_string(),
            }),
            execution_method: ExecutionMethodResourceTarget::Any,
        })
        .validate()
        .expect_err("Management canister in CallExternalCanister should be invalid");
        RequestSpecifier::CallExternalCanister(CallExternalCanisterResourceTarget {
            validation_method: ValidationMethodResourceTarget::ValidationMethod(CanisterMethod {
                canister_id: station_canister_id,
                method_name: "foo".to_string(),
            }),
            execution_method: ExecutionMethodResourceTarget::Any,
        })
        .validate()
        .expect_err("Station canister in CallExternalCanister should be invalid");
        RequestSpecifier::CallExternalCanister(CallExternalCanisterResourceTarget {
            validation_method: ValidationMethodResourceTarget::ValidationMethod(CanisterMethod {
                canister_id: upgrader_canister_id,
                method_name: "foo".to_string(),
            }),
            execution_method: ExecutionMethodResourceTarget::Any,
        })
        .validate()
        .expect_err("Upgrader canister in CallExternalCanister should be invalid");
        RequestSpecifier::CallExternalCanister(CallExternalCanisterResourceTarget {
            validation_method: ValidationMethodResourceTarget::ValidationMethod(CanisterMethod {
                canister_id: ledger_canister_id,
                method_name: "foo".to_string(),
            }),
            execution_method: ExecutionMethodResourceTarget::Any,
        })
        .validate()
        .expect_err("Ledger canister in CallExternalCanister should be invalid");
        RequestSpecifier::CallExternalCanister(CallExternalCanisterResourceTarget {
            validation_method: ValidationMethodResourceTarget::ValidationMethod(CanisterMethod {
                canister_id: external_canister_id,
                method_name: "foo".to_string(),
            }),
            execution_method: ExecutionMethodResourceTarget::Any,
        })
        .validate()
        .expect("CallExternalCanister should be valid");
        RequestSpecifier::CallExternalCanister(CallExternalCanisterResourceTarget {
            validation_method: ValidationMethodResourceTarget::No,
            execution_method: ExecutionMethodResourceTarget::ExecutionMethod(CanisterMethod {
                canister_id: Principal::management_canister(),
                method_name: "install_code".to_string(),
            }),
        })
        .validate()
        .expect_err("Management canister in CallExternalCanister should be invalid");
        RequestSpecifier::CallExternalCanister(CallExternalCanisterResourceTarget {
            validation_method: ValidationMethodResourceTarget::No,
            execution_method: ExecutionMethodResourceTarget::ExecutionMethod(CanisterMethod {
                canister_id: station_canister_id,
                method_name: "foo".to_string(),
            }),
        })
        .validate()
        .expect_err("Station canister in CallExternalCanister should be invalid");
        RequestSpecifier::CallExternalCanister(CallExternalCanisterResourceTarget {
            validation_method: ValidationMethodResourceTarget::No,
            execution_method: ExecutionMethodResourceTarget::ExecutionMethod(CanisterMethod {
                canister_id: upgrader_canister_id,
                method_name: "foo".to_string(),
            }),
        })
        .validate()
        .expect_err("Upgrader canister in CallExternalCanister should be invalid");
        RequestSpecifier::CallExternalCanister(CallExternalCanisterResourceTarget {
            validation_method: ValidationMethodResourceTarget::No,
            execution_method: ExecutionMethodResourceTarget::ExecutionMethod(CanisterMethod {
                canister_id: ledger_canister_id,
                method_name: "foo".to_string(),
            }),
        })
        .validate()
        .expect_err("Ledger canister in CallExternalCanister should be invalid");
        RequestSpecifier::CallExternalCanister(CallExternalCanisterResourceTarget {
            validation_method: ValidationMethodResourceTarget::No,
            execution_method: ExecutionMethodResourceTarget::ExecutionMethod(CanisterMethod {
                canister_id: external_canister_id,
                method_name: "foo".to_string(),
            }),
        })
        .validate()
        .expect("CallExternalCanister should be valid");
        RequestSpecifier::AddRequestPolicy
            .validate()
            .expect("AddRequestPolicy should be valid");
        RequestSpecifier::AddUserGroup
            .validate()
            .expect("AddUserGroup should be valid");
    }

    #[test]
    fn fail_invalid_request_specifier() {
        disable_mock_resource_validation();

        RequestSpecifier::Transfer(ResourceIds::Ids(vec![[0; 16]]))
            .validate()
            .expect_err("Non existent account ID should be invalid");
        RequestSpecifier::EditAccount(ResourceIds::Ids(vec![[0; 16]]))
            .validate()
            .expect_err("Non existent account ID should be invalid");
        RequestSpecifier::EditUser(ResourceIds::Ids(vec![[0; 16]]))
            .validate()
            .expect_err("Non existent user ID should be invalid");
        RequestSpecifier::EditAddressBookEntry(ResourceIds::Ids(vec![[0; 16]]))
            .validate()
            .expect_err("Non existent address book entry ID should be invalid");
        RequestSpecifier::RemoveAddressBookEntry(ResourceIds::Ids(vec![[0; 16]]))
            .validate()
            .expect_err("Non existent address book entry ID should be invalid");
        RequestSpecifier::EditRequestPolicy(ResourceIds::Ids(vec![[0; 16]]))
            .validate()
            .expect_err("Non existent request policy ID should be invalid");
        RequestSpecifier::RemoveRequestPolicy(ResourceIds::Ids(vec![[0; 16]]))
            .validate()
            .expect_err("Non existent request policy ID should be invalid");
        RequestSpecifier::EditUserGroup(ResourceIds::Ids(vec![[0; 16]]))
            .validate()
            .expect_err("Non existent user group ID should be invalid");
        RequestSpecifier::RemoveUserGroup(ResourceIds::Ids(vec![[0; 16]]))
            .validate()
            .expect_err("Non existent user group ID should be invalid");
    }
}
