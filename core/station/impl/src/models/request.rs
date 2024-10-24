use super::request_policy_rule::{RequestEvaluationResult, RequestPolicyRuleInput};
use super::{
    ChangeAssets, ConfigureExternalCanisterOperationKind, DisplayUser, EvaluationStatus,
    RequestApproval, RequestApprovalStatus, RequestOperation, RequestStatus, UserId, UserKey,
};
use crate::core::evaluation::{
    Evaluate, REQUEST_APPROVE_RIGHTS_REQUEST_POLICY_RULE_EVALUATOR, REQUEST_POLICY_RULE_EVALUATOR,
    REQUEST_POSSIBLE_APPROVERS_REQUEST_POLICY_RULE_EVALUATOR,
};
use crate::core::ic_cdk::api::print;
use crate::core::ic_cdk::next_time;
use crate::core::request::{
    RequestApprovalRightsEvaluator, RequestEvaluator, RequestPossibleApproversFinder,
};
use crate::core::validation::{
    EnsureAccount, EnsureAddressBookEntry, EnsureAsset, EnsureIdExists, EnsureRequestPolicy,
    EnsureUser, EnsureUserGroup,
};
use crate::errors::{EvaluateError, RequestError, ValidationError};
use crate::models::resource::{ExecutionMethodResourceTarget, ValidationMethodResourceTarget};
use crate::repositories::USER_REPOSITORY;
use candid::{CandidType, Deserialize};
use orbit_essentials::model::{ContextualModel, ModelKey};
use orbit_essentials::repository::Repository;
use orbit_essentials::storable;
use orbit_essentials::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use std::collections::HashSet;

/// The request id, which is a UUID.
pub type RequestId = UUID;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RequestExecutionPlan {
    Immediate,
    Scheduled { execution_time: Timestamp },
}

/// Represents a request within the system.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Request {
    /// The request id, which is a UUID.
    pub id: RequestId,
    /// The title of the request.
    pub title: String,
    /// The summary of the request, this is a longer description of the request.
    pub summary: Option<String>,
    /// The user id that resulted in the request creation.
    pub requested_by: UserId,
    /// The status that the request is in.
    pub status: RequestStatus,
    /// An operation that the request should execute, e.g. "transfer".
    pub operation: RequestOperation,
    /// The expiration date of the request.
    pub expiration_dt: Timestamp,
    /// The execution plan of the request.
    pub execution_plan: RequestExecutionPlan,
    /// The list of user approvals on the request.
    pub approvals: Vec<RequestApproval>,
    /// The timestamp of the request creation.
    pub created_timestamp: Timestamp,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RequestKey {
    /// The request id, which is a UUID.
    pub id: RequestId,
}

impl ModelKey<RequestKey> for Request {
    fn key(&self) -> RequestKey {
        RequestKey { id: self.id }
    }
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct RequestCallerPrivileges {
    pub id: UUID,
    pub can_approve: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RequestAdditionalInfo {
    pub id: UUID,
    pub requester_name: String,
    pub approvers: Vec<DisplayUser>,
    pub evaluation_result: Option<RequestEvaluationResult>,
}

fn validate_title(title: &str) -> ModelValidatorResult<RequestError> {
    if title.len() > Request::MAX_TITLE_LEN as usize {
        return Err(RequestError::ValidationError {
            info: format!(
                "Request title length exceeds the maximum allowed: {}",
                Request::MAX_TITLE_LEN
            ),
        });
    }

    Ok(())
}

fn validate_summary(summary: &Option<String>) -> ModelValidatorResult<RequestError> {
    if let Some(summary) = summary {
        if summary.len() > Request::MAX_SUMMARY_LEN as usize {
            return Err(RequestError::ValidationError {
                info: format!(
                    "Request summary length exceeds the maximum allowed: {}",
                    Request::MAX_SUMMARY_LEN
                ),
            });
        }
    }

    Ok(())
}

fn validate_requested_by(requested_by: &UserId) -> ModelValidatorResult<RequestError> {
    USER_REPOSITORY
        .get(&UserKey { id: *requested_by })
        .ok_or(RequestError::ValidationError {
            info: "The requested_by user does not exist".to_owned(),
        })?;
    Ok(())
}

fn validate_request_operation_foreign_keys(
    operation: &RequestOperation,
) -> ModelValidatorResult<ValidationError> {
    match operation {
        RequestOperation::ManageSystemInfo(_) => (),
        RequestOperation::Transfer(op) => {
            EnsureAccount::id_exists(&op.input.from_account_id)?;
        }
        RequestOperation::AddAccount(op) => {
            op.input.read_permission.validate()?;
            op.input.configs_permission.validate()?;
            op.input.transfer_permission.validate()?;

            if let Some(policy_rule) = &op.input.transfer_request_policy {
                policy_rule.validate()?;
            }

            if let Some(policy_rule) = &op.input.configs_request_policy {
                policy_rule.validate()?;
            }
        }
        RequestOperation::EditAccount(op) => {
            EnsureAccount::id_exists(&op.input.account_id)?;

            if let Some(allow) = &op.input.read_permission {
                allow.validate()?;
            }

            if let Some(allow) = &op.input.configs_permission {
                allow.validate()?;
            }

            if let Some(allow) = &op.input.transfer_permission {
                allow.validate()?;
            }

            if let Some(RequestPolicyRuleInput::Set(criteria)) = &op.input.configs_request_policy {
                criteria.validate()?;
            }

            if let Some(RequestPolicyRuleInput::Set(policy_rule)) =
                &op.input.transfer_request_policy
            {
                policy_rule.validate()?;
            }

            if let Some(ChangeAssets::ReplaceWith { assets }) = &op.input.change_assets {
                EnsureAsset::id_list_exists(assets)?;
            }

            if let Some(ChangeAssets::Change {
                add_assets,
                remove_assets,
            }) = &op.input.change_assets
            {
                EnsureAsset::id_list_exists(add_assets)?;
                EnsureAsset::id_list_exists(remove_assets)?;
            }
        }
        RequestOperation::AddAddressBookEntry(_) => (),
        RequestOperation::EditAddressBookEntry(op) => {
            EnsureAddressBookEntry::id_exists(&op.input.address_book_entry_id)?;
        }
        RequestOperation::RemoveAddressBookEntry(op) => {
            EnsureAddressBookEntry::id_exists(&op.input.address_book_entry_id)?;
        }
        RequestOperation::AddUser(op) => {
            EnsureUserGroup::id_list_exists(&op.input.groups)?;
        }
        RequestOperation::EditUser(op) => {
            EnsureUser::id_exists(&op.input.user_id)?;

            if let Some(group_ids) = &op.input.groups {
                EnsureUserGroup::id_list_exists(group_ids)?;
            }
        }
        RequestOperation::EditPermission(op) => {
            op.input.resource.validate()?;

            if let Some(user_ids) = &op.input.users {
                EnsureUser::id_list_exists(user_ids)?;
            }

            if let Some(group_ids) = &op.input.user_groups {
                EnsureUserGroup::id_list_exists(group_ids)?;
            }
        }
        RequestOperation::AddUserGroup(_) => (),
        RequestOperation::EditUserGroup(op) => {
            EnsureUserGroup::id_exists(&op.input.user_group_id)?;
        }
        RequestOperation::RemoveUserGroup(ok) => {
            EnsureUserGroup::id_exists(&ok.input.user_group_id)?;
        }
        RequestOperation::SystemUpgrade(_) => (),
        RequestOperation::ChangeExternalCanister(_) => (),
        RequestOperation::ConfigureExternalCanister(op) => {
            let canister_id = op.canister_id;
            if let ConfigureExternalCanisterOperationKind::Settings(settings) = &op.kind {
                if let Some(updated_request_policies) = &settings.request_policies {
                    ContextualModel::new(updated_request_policies.clone(), canister_id)
                        .validate()?;
                }
            }
        }
        RequestOperation::FundExternalCanister(_) => (),
        RequestOperation::CreateExternalCanister(op) => {
            op.input.validate()?;
        }
        RequestOperation::CallExternalCanister(op) => {
            let validation_method_target: ValidationMethodResourceTarget =
                op.input.validation_method.clone().into();
            validation_method_target.validate()?;
            let execution_method_target: ExecutionMethodResourceTarget =
                op.input.execution_method.clone().into();
            execution_method_target.validate()?;
        }
        RequestOperation::AddRequestPolicy(op) => {
            op.input.specifier.validate()?;
            op.input.rule.validate()?;
        }
        RequestOperation::EditRequestPolicy(op) => {
            EnsureRequestPolicy::id_exists(&op.input.policy_id)?;

            if let Some(specifier) = &op.input.specifier {
                specifier.validate()?;
            }

            if let Some(policy_rule) = &op.input.rule {
                policy_rule.validate()?;
            }
        }
        RequestOperation::RemoveRequestPolicy(op) => {
            EnsureRequestPolicy::id_exists(&op.input.policy_id)?;
        }
        RequestOperation::SetDisasterRecovery(op) => {
            if let Some(committee) = &op.input.committee {
                EnsureUserGroup::id_exists(&committee.user_group_id)?;
            }
        }
        RequestOperation::AddAsset(_) => (),
        RequestOperation::EditAsset(op) => {
            EnsureAsset::id_exists(&op.input.asset_id)?;
        }
        RequestOperation::RemoveAsset(op) => {
            EnsureAsset::id_exists(&op.input.asset_id)?;
        }
    }
    Ok(())
}

impl ModelValidator<RequestError> for Request {
    fn validate(&self) -> ModelValidatorResult<RequestError> {
        validate_title(&self.title)?;
        validate_summary(&self.summary)?;
        validate_requested_by(&self.requested_by)?;

        validate_request_operation_foreign_keys(&self.operation)?;

        Ok(())
    }
}

impl Request {
    pub const MAX_TITLE_LEN: u8 = 255;
    pub const MAX_SUMMARY_LEN: u16 = 1000;

    /// Creates a new request key from the given key components.
    pub fn key(request_id: RequestId) -> RequestKey {
        RequestKey { id: request_id }
    }

    pub fn to_key(&self) -> RequestKey {
        Request::key(self.id.to_owned())
    }

    pub fn approvers(&self) -> HashSet<UserId> {
        let mut approvers = HashSet::new();

        self.approvals
            .iter()
            .map(|decision| decision.approver_id.to_owned())
            .for_each(|user_id| {
                approvers.insert(user_id);
            });

        approvers
    }

    /// Gives the default expiration date for a request which is 30 days from the current time.
    pub fn default_expiration_dt_ns() -> Timestamp {
        let time_in_ns: u64 = 30 * 24 * 60 * 60 * 1_000_000_000;

        next_time() + time_in_ns
    }

    /// Checks if the user can approve the request.
    pub fn can_approve(&self, user_id: &UUID) -> bool {
        // Only requests that are in the created state can be approved.
        if self.status != RequestStatus::Created {
            return false;
        }

        // If the user has already added their approval, they can't add again.
        if self
            .approvals
            .iter()
            .any(|approval| approval.approver_id == *user_id)
        {
            return false;
        }

        let approval_rights_evaluator = RequestApprovalRightsEvaluator {
            request: &self.index_fields(),
            approver_id: *user_id,
            approval_rights_evaluator: REQUEST_APPROVE_RIGHTS_REQUEST_POLICY_RULE_EVALUATOR.clone(),
        };

        match approval_rights_evaluator.evaluate() {
            Ok(has_approval_right) => has_approval_right,
            Err(_) => {
                print(format!(
                    "Failed to evaluate voting rights for request: {:?}",
                    self
                ));

                false
            }
        }
    }

    pub fn add_approval(
        &mut self,
        user_id: UUID,
        decision: RequestApprovalStatus,
        reason: Option<String>,
    ) -> ModelValidatorResult<RequestError> {
        if self
            .approvals
            .iter()
            .any(|approval| approval.approver_id == user_id)
        {
            // users can only approval once per request
            return Err(RequestError::ApprovalNotAllowed);
        }

        let now = next_time();
        let approval = RequestApproval {
            approver_id: user_id,
            status: decision,
            status_reason: reason,
            decided_dt: now,
            last_modification_timestamp: now,
        };

        approval.validate()?;

        self.approvals.push(approval);

        Ok(())
    }

    pub async fn reevaluate(&mut self) -> Result<Option<RequestEvaluationResult>, EvaluateError> {
        if self.status == RequestStatus::Created {
            let evaluator = RequestEvaluator {
                request: self.to_owned(),
                policy_rule_evaluator: REQUEST_POLICY_RULE_EVALUATOR.to_owned(),
            };

            let evaluation_result = evaluator.evaluate()?;

            if evaluation_result.status == EvaluationStatus::Approved {
                self.status = RequestStatus::Approved;
            } else if evaluation_result.status == EvaluationStatus::Rejected {
                self.status = RequestStatus::Rejected;
            }

            Ok(Some(evaluation_result))
        } else {
            Ok(None)
        }
    }

    pub async fn find_all_possible_approvers(&self) -> Result<HashSet<UUID>, EvaluateError> {
        let evaluator = RequestPossibleApproversFinder {
            request: self,
            possible_approvers_policy_rule_evaluator:
                REQUEST_POSSIBLE_APPROVERS_REQUEST_POLICY_RULE_EVALUATOR.to_owned(),
        };

        evaluator.evaluate()
    }

    /// Checks if the request is finalized.
    ///
    /// A request that is finalized won't have its status changed anymore.
    pub fn is_finalized(&self) -> bool {
        matches!(
            self.status,
            RequestStatus::Completed { .. }
                | RequestStatus::Cancelled { .. }
                | RequestStatus::Failed { .. }
                | RequestStatus::Rejected
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::core::validation::disable_mock_resource_validation;
    use crate::models::asset_test_utils::mock_asset;
    use crate::models::permission::Allow;
    use crate::models::{
        Account, AccountKey, AddAccountOperationInput, AddAssetOperationInput, AddUserOperation,
        AddUserOperationInput, Blockchain, Metadata, TokenStandard, TransferOperation,
        TransferOperationInput,
    };
    use crate::repositories::ACCOUNT_REPOSITORY;
    use crate::services::{AccountService, AssetService};

    use super::request_test_utils::mock_request;
    use super::*;

    #[test]
    fn fail_request_title_too_big() {
        let mut request = mock_request();
        request.title = "a".repeat(Request::MAX_TITLE_LEN as usize + 1);

        let result = validate_title(&request.title);

        assert!(result.is_err());
    }

    #[test]
    fn test_request_title_is_valid() {
        let mut request = mock_request();
        request.title = "a".repeat(Request::MAX_TITLE_LEN as usize);

        let result = validate_title(&request.title);

        assert!(result.is_ok());
    }

    #[test]
    fn fail_request_summary_too_big() {
        let mut request = mock_request();
        request.summary = Some("a".repeat(Request::MAX_SUMMARY_LEN as usize + 1));

        let result = validate_summary(&request.summary);

        assert!(result.is_err());
    }

    #[test]
    fn test_request_summary_is_valid() {
        let mut request = mock_request();
        request.summary = Some("a".repeat(Request::MAX_SUMMARY_LEN as usize));

        let result = validate_summary(&request.summary);

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_request_operation_is_valid() {
        disable_mock_resource_validation();

        let asset = AssetService::default()
            .create(
                AddAssetOperationInput {
                    name: "a".to_owned(),
                    symbol: "a".to_owned(),
                    decimals: 0,
                    metadata: Metadata::default(),
                    blockchain: Blockchain::InternetComputer,
                    standards: vec![TokenStandard::InternetComputerNative],
                },
                None,
            )
            .expect("Failed to create asset");

        let account_service = AccountService::default();
        let account = account_service
            .create_account(
                AddAccountOperationInput {
                    name: "a".to_owned(),
                    assets: vec![asset.id],
                    metadata: Metadata::default(),
                    read_permission: Allow::default(),
                    configs_permission: Allow::default(),
                    transfer_permission: Allow::default(),
                    configs_request_policy: None,
                    transfer_request_policy: None,
                },
                None,
            )
            .await
            .expect("Failed to create account");

        let operation = RequestOperation::Transfer(TransferOperation {
            transfer_id: None,
            fee: None,

            input: TransferOperationInput {
                network: "mainnet".to_string(),
                amount: 1u64.into(),
                fee: None,
                metadata: Metadata::default(),
                to: "0x1234".to_string(),
                from_account_id: account.id,
                from_asset_id: asset.id,
                with_standard: TokenStandard::InternetComputerNative,
            },
            asset,
        });

        let result = validate_request_operation_foreign_keys(&operation);

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn fail_request_operation_with_invalid_id() {
        disable_mock_resource_validation();

        validate_request_operation_foreign_keys(&RequestOperation::Transfer(TransferOperation {
            transfer_id: None,
            fee: None,
            input: TransferOperationInput {
                network: "mainnet".to_string(),
                amount: 1u64.into(),
                fee: None,
                metadata: Metadata::default(),
                to: "0x1234".to_string(),
                from_account_id: [0; 16],
                from_asset_id: [0; 16],
                with_standard: TokenStandard::InternetComputerNative,
            },
            asset: mock_asset(),
        }))
        .expect_err("Invalid account id should fail");

        validate_request_operation_foreign_keys(&RequestOperation::AddUser(AddUserOperation {
            user_id: None,
            input: AddUserOperationInput {
                name: "user-1".to_string(),
                identities: vec![],
                groups: vec![[1; 16]],
                status: crate::models::UserStatus::Active,
            },
        }))
        .expect_err("Invalid user group id should fail");

        validate_request_operation_foreign_keys(&RequestOperation::EditUserGroup(
            crate::models::EditUserGroupOperation {
                input: crate::models::EditUserGroupOperationInput {
                    user_group_id: [0; 16],
                    name: "a".to_owned(),
                },
            },
        ))
        .expect_err("Invalid user group id should fail");
        validate_request_operation_foreign_keys(&RequestOperation::RemoveUserGroup(
            crate::models::RemoveUserGroupOperation {
                input: crate::models::RemoveUserGroupOperationInput {
                    user_group_id: [0; 16],
                },
            },
        ))
        .expect_err("Invalid user group id should fail");

        validate_request_operation_foreign_keys(&RequestOperation::AddRequestPolicy(
            crate::models::AddRequestPolicyOperation {
                policy_id: None,
                input: crate::models::AddRequestPolicyOperationInput {
                    specifier: crate::models::request_specifier::RequestSpecifier::EditUser(
                        crate::models::resource::ResourceIds::Ids(vec![[1; 16]]),
                    ),
                    rule: crate::models::request_policy_rule::RequestPolicyRule::AutoApproved,
                },
            },
        ))
        .expect_err("Invalid request specifier should fail");

        validate_request_operation_foreign_keys(&RequestOperation::EditRequestPolicy(
            crate::models::EditRequestPolicyOperation {
                input: crate::models::EditRequestPolicyOperationInput {
                    policy_id: [0; 16],
                    specifier: None,
                    rule: None,
                },
            },
        ))
        .expect_err("Invalid request policy id should fail");

        validate_request_operation_foreign_keys(&RequestOperation::RemoveRequestPolicy(
            crate::models::RemoveRequestPolicyOperation {
                input: crate::models::RemoveRequestPolicyOperationInput { policy_id: [0; 16] },
            },
        ))
        .expect_err("Invalid request policy id should fail");

        validate_request_operation_foreign_keys(&RequestOperation::AddAccount(
            crate::models::AddAccountOperation {
                account_id: None,
                input: crate::models::AddAccountOperationInput {
                    name: "a".to_owned(),
                    assets: vec![],
                    metadata: Metadata::default(),
                    read_permission: Allow {
                        auth_scope: crate::models::permission::AuthScope::Restricted,
                        users: vec![[1; 16]],
                        user_groups: vec![],
                    },
                    configs_permission: Allow::default(),
                    transfer_permission: Allow::default(),
                    configs_request_policy: None,
                    transfer_request_policy: None,
                },
            },
        ))
        .expect_err("Invalid user id should fail");

        validate_request_operation_foreign_keys(&RequestOperation::EditAccount(
            crate::models::EditAccountOperation {
                input: crate::models::EditAccountOperationInput {
                    account_id: [0; 16],
                    change_assets: None,
                    read_permission: None,
                    configs_permission: None,
                    transfer_permission: None,
                    configs_request_policy: None,
                    transfer_request_policy: None,
                    name: None,
                },
            },
        ))
        .expect_err("Invalid account id should fail");

        ACCOUNT_REPOSITORY.insert(
            AccountKey { id: [0; 16] },
            Account {
                id: [0; 16],
                name: "a".to_owned(),
                seed: [0; 16],
                assets: vec![],
                addresses: vec![],
                metadata: Metadata::default(),
                transfer_request_policy_id: None,
                configs_request_policy_id: None,
                last_modification_timestamp: 0,
            },
        );

        validate_request_operation_foreign_keys(&RequestOperation::EditAccount(
            crate::models::EditAccountOperation {
                input: crate::models::EditAccountOperationInput {
                    account_id: [0; 16],
                    change_assets: Some(ChangeAssets::ReplaceWith {
                        assets: vec![[0; 16]],
                    }),
                    read_permission: None,
                    configs_permission: None,
                    transfer_permission: None,
                    configs_request_policy: None,
                    transfer_request_policy: None,
                    name: None,
                },
            },
        ))
        .expect_err("Invalid asset id should fail");

        ACCOUNT_REPOSITORY.clear();

        validate_request_operation_foreign_keys(&RequestOperation::EditAddressBookEntry(
            crate::models::EditAddressBookEntryOperation {
                input: crate::models::EditAddressBookEntryOperationInput {
                    address_book_entry_id: [0; 16],
                    address_owner: None,
                    change_metadata: None,
                    labels: None,
                },
            },
        ))
        .expect_err("Invalid address book entry id should fail");

        validate_request_operation_foreign_keys(&RequestOperation::RemoveAddressBookEntry(
            crate::models::RemoveAddressBookEntryOperation {
                input: crate::models::RemoveAddressBookEntryOperationInput {
                    address_book_entry_id: [0; 16],
                },
            },
        ))
        .expect_err("Invalid address book entry id should fail");

        validate_request_operation_foreign_keys(&RequestOperation::EditUser(
            crate::models::EditUserOperation {
                input: crate::models::EditUserOperationInput {
                    user_id: [0; 16],
                    groups: None,
                    name: None,
                    identities: None,
                    status: None,
                    cancel_pending_requests: None,
                },
            },
        ))
        .expect_err("Invalid user id should fail");

        validate_request_operation_foreign_keys(&RequestOperation::EditPermission(
            crate::models::EditPermissionOperation {
                input: crate::models::EditPermissionOperationInput {
                    resource: crate::models::resource::Resource::Account(
                        crate::models::resource::AccountResourceAction::Read(
                            crate::models::resource::ResourceId::Id([0; 16]),
                        ),
                    ),
                    users: None,
                    user_groups: None,
                    auth_scope: None,
                },
            },
        ))
        .expect_err("Invalid resource id should fail");
    }
}

#[cfg(any(test, feature = "canbench"))]
pub mod request_test_utils {
    use super::*;
    use crate::models::{
        asset_test_utils::mock_asset, Metadata, RequestApprovalStatus, TokenStandard,
        TransferOperation, TransferOperationInput,
    };
    use num_bigint::BigUint;
    use uuid::Uuid;

    pub fn mock_request() -> Request {
        Request {
            id: *Uuid::new_v4().as_bytes(),
            title: "foo".to_string(),
            summary: Some("bar".to_string()),
            requested_by: [1; 16],
            status: RequestStatus::Approved,
            expiration_dt: 100,
            execution_plan: RequestExecutionPlan::Immediate,
            operation: RequestOperation::Transfer(TransferOperation {
                transfer_id: None,
                fee: None,
                input: TransferOperationInput {
                    network: "mainnet".to_string(),
                    amount: candid::Nat(BigUint::from(100u32)),
                    fee: None,
                    metadata: Metadata::default(),
                    to: "0x1234".to_string(),
                    from_account_id: [1; 16],
                    from_asset_id: [0; 16],
                    with_standard: TokenStandard::InternetComputerNative,
                },
                asset: mock_asset(),
            }),
            approvals: vec![RequestApproval {
                approver_id: [1; 16],
                status: RequestApprovalStatus::Approved,
                status_reason: None,
                decided_dt: 0,
                last_modification_timestamp: 0,
            }],
            created_timestamp: 0,
            last_modification_timestamp: 0,
        }
    }
}
