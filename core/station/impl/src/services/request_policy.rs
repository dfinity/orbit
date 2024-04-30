use crate::{
    core::{
        authorization::Authorization,
        generate_uuid_v4,
        utils::{paginated_items, retain_accessible_resources, PaginatedData, PaginatedItemsArgs},
        CallContext,
    },
    errors::RequestError,
    models::{
        request_policy_rule::RequestPolicyRuleInput,
        request_specifier::RequestSpecifier,
        resource::{Resource, ResourceAction, ResourceId},
        AddRequestPolicyOperationInput, EditRequestPolicyOperationInput, RequestPolicy,
        RequestPolicyCallerPrivileges,
    },
    repositories::request_policy::{RequestPolicyRepository, REQUEST_POLICY_REPOSITORY},
};
use lazy_static::lazy_static;
use orbit_essentials::{api::ServiceResult, cdk::api::print, types::UUID};
use orbit_essentials::{model::ModelValidator, repository::Repository};
use station_api::ListRequestPoliciesInput;
use std::sync::Arc;
use uuid::Uuid;

lazy_static! {
    pub static ref REQUEST_POLICY_SERVICE: Arc<RequestPolicyService> = Arc::new(
        RequestPolicyService::new(Arc::clone(&REQUEST_POLICY_REPOSITORY))
    );
}

#[derive(Default, Debug)]
pub struct RequestPolicyService {
    request_policy_repository: Arc<RequestPolicyRepository>,
}

impl RequestPolicyService {
    pub const DEFAULT_POLICIES_LIMIT: u16 = 100;
    pub const MAX_LIST_POLICIES_LIMIT: u16 = 1000;

    pub fn new(request_policy_repository: Arc<RequestPolicyRepository>) -> Self {
        Self {
            request_policy_repository,
        }
    }

    pub fn get_request_policy(&self, id: &UUID) -> ServiceResult<RequestPolicy, RequestError> {
        let policy =
            self.request_policy_repository
                .get(id)
                .ok_or(RequestError::PolicyNotFound {
                    id: Uuid::from_bytes(*id).hyphenated().to_string(),
                })?;

        Ok(policy)
    }

    pub async fn add_request_policy(
        &self,
        input: AddRequestPolicyOperationInput,
    ) -> ServiceResult<RequestPolicy> {
        let id: uuid::Uuid = generate_uuid_v4().await;
        let policy = RequestPolicy {
            id: *id.as_bytes(),
            specifier: input.specifier,
            rule: input.rule,
        };

        policy.validate()?;

        self.request_policy_repository
            .insert(policy.id, policy.clone());

        Ok(policy)
    }

    /// Handles the policy change operation.
    ///
    /// Removes the existing policy rule if variant is `Remove`, otherwise edits the existing rule or adds a new one.
    pub async fn handle_policy_change(
        &self,
        specifier: RequestSpecifier,
        policy_rule: RequestPolicyRuleInput,
        editable_policy_id: &mut Option<UUID>,
    ) -> ServiceResult<()> {
        match policy_rule {
            RequestPolicyRuleInput::Remove => {
                if let Some(existing_policy_id) = editable_policy_id {
                    if let Err(RequestError::PolicyNotFound { id }) =
                        self.remove_request_policy(existing_policy_id).await
                    {
                        print(format!(
                            "Cannot handle policy change: policy {} not found",
                            id
                        ));
                    }

                    // Directly modify the policy_id in place
                    *editable_policy_id = None;
                }
            }
            RequestPolicyRuleInput::Set(policy_rule) => {
                match editable_policy_id {
                    Some(existing_policy_id) => {
                        // If there's an existing policy, edit it
                        self.edit_request_policy(EditRequestPolicyOperationInput {
                            policy_id: *existing_policy_id,
                            specifier: Some(specifier),
                            rule: Some(policy_rule),
                        })
                        .await?;
                    }
                    None => {
                        // If there's no existing policy, add a new one
                        let policy = self
                            .add_request_policy(AddRequestPolicyOperationInput {
                                specifier,
                                rule: policy_rule,
                            })
                            .await?;

                        *editable_policy_id = Some(policy.id);
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn edit_request_policy(
        &self,
        input: EditRequestPolicyOperationInput,
    ) -> ServiceResult<RequestPolicy> {
        let mut policy = self.get_request_policy(&input.policy_id)?;

        if let Some(specifier) = input.specifier {
            policy.specifier = specifier;
        }

        if let Some(policy_rule) = input.rule {
            policy.rule = policy_rule;
        }

        policy.validate()?;

        self.request_policy_repository
            .insert(policy.id, policy.to_owned());

        Ok(policy)
    }

    pub async fn remove_request_policy(&self, id: &UUID) -> ServiceResult<(), RequestError> {
        let policy = self.get_request_policy(id)?;

        self.request_policy_repository.remove(&policy.id);

        Ok(())
    }

    pub async fn get_caller_privileges_for_request_policy(
        &self,
        policy_id: &UUID,
        ctx: &CallContext,
    ) -> ServiceResult<RequestPolicyCallerPrivileges> {
        Ok(RequestPolicyCallerPrivileges {
            id: *policy_id,
            can_edit: Authorization::is_allowed(
                ctx,
                &Resource::RequestPolicy(ResourceAction::Update(ResourceId::Id(*policy_id))),
            ),
            can_delete: Authorization::is_allowed(
                ctx,
                &Resource::RequestPolicy(ResourceAction::Delete(ResourceId::Id(*policy_id))),
            ),
        })
    }

    pub async fn list_request_policies(
        &self,
        input: ListRequestPoliciesInput,
        ctx: &CallContext,
    ) -> ServiceResult<PaginatedData<RequestPolicy>> {
        let mut policies = self.request_policy_repository.list();

        retain_accessible_resources(ctx, &mut policies, |policy| {
            Resource::RequestPolicy(ResourceAction::Read(ResourceId::Id(policy.id)))
        });

        let result = paginated_items(PaginatedItemsArgs {
            offset: input.offset,
            limit: input.limit,
            default_limit: Some(Self::DEFAULT_POLICIES_LIMIT),
            max_limit: Some(Self::MAX_LIST_POLICIES_LIMIT),
            items: &policies,
        })?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::ic_cdk::api::id as self_canister_id,
        models::{
            account_test_utils::mock_account, request_policy_rule::RequestPolicyRule,
            request_policy_test_utils::mock_request_policy, request_specifier::RequestSpecifier,
            resource::ResourceIds,
        },
    };

    #[tokio::test]
    async fn test_request_policy_operations() {
        let service = REQUEST_POLICY_SERVICE.clone();
        let policy = service
            .add_request_policy(AddRequestPolicyOperationInput {
                specifier: RequestSpecifier::AddAccount,
                rule: RequestPolicyRule::AutoApproved,
            })
            .await;

        assert!(policy.is_ok());

        let policy = policy.unwrap();
        let fetched_policy = service.get_request_policy(&policy.id).unwrap();

        assert_eq!(fetched_policy.specifier, policy.specifier);
        assert_eq!(fetched_policy.rule, policy.rule);

        let policy = service
            .edit_request_policy(EditRequestPolicyOperationInput {
                policy_id: policy.id,
                specifier: Some(RequestSpecifier::AddAccount),
                rule: Some(RequestPolicyRule::AutoApproved),
            })
            .await;

        assert!(policy.is_ok());

        let policy = policy.unwrap();
        let updated_policy = service.get_request_policy(&policy.id).unwrap();

        assert_eq!(updated_policy.specifier, policy.specifier);
        assert_eq!(updated_policy.rule, policy.rule);
    }

    #[test]
    fn test_get_request_policy_not_found() {
        let service = REQUEST_POLICY_SERVICE.clone();
        let result = service.get_request_policy(&[1; 16]);

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn list_request_policies_should_use_offset_and_limit() {
        for i in 0..50 {
            let mut policy = mock_request_policy();
            policy.id = [i; 16];
            policy.specifier = RequestSpecifier::AddAccount;
            REQUEST_POLICY_REPOSITORY.insert(policy.id, policy.to_owned());
        }

        let input = ListRequestPoliciesInput {
            offset: Some(15),
            limit: Some(30),
        };

        let result = REQUEST_POLICY_SERVICE
            .list_request_policies(input, &CallContext::new(self_canister_id()))
            .await
            .unwrap();
        assert_eq!(result.items.len(), 30);
        assert_eq!(result.next_offset, Some(45));
    }

    #[tokio::test]
    async fn test_remove_request_policy() {
        let service = REQUEST_POLICY_SERVICE.clone();
        let policy = service
            .add_request_policy(AddRequestPolicyOperationInput {
                specifier: RequestSpecifier::AddAccount,
                rule: RequestPolicyRule::AutoApproved,
            })
            .await
            .unwrap();

        assert!(service.get_request_policy(&policy.id).is_ok());

        service.remove_request_policy(&policy.id).await.unwrap();

        assert!(service.get_request_policy(&policy.id).is_err());
    }

    #[tokio::test]
    async fn test_handle_policy_change() {
        let mut account = mock_account();
        account.configs_request_policy_id = None;

        REQUEST_POLICY_SERVICE
            .handle_policy_change(
                RequestSpecifier::EditAccount(ResourceIds::Ids(vec![account.id])),
                RequestPolicyRuleInput::Set(RequestPolicyRule::AutoApproved),
                &mut account.configs_request_policy_id,
            )
            .await
            .unwrap();

        assert!(account.configs_request_policy_id.is_some());

        REQUEST_POLICY_SERVICE
            .handle_policy_change(
                RequestSpecifier::EditAccount(ResourceIds::Ids(vec![account.id])),
                RequestPolicyRuleInput::Remove,
                &mut account.configs_request_policy_id,
            )
            .await
            .unwrap();

        assert!(account.configs_request_policy_id.is_none());
    }
}
