use crate::{
    core::{
        authorization::Authorization,
        generate_uuid_v4,
        utils::{paginated_items, retain_accessible_resources, PaginatedData, PaginatedItemsArgs},
        CallContext,
    },
    errors::ProposalError,
    models::{
        criteria::ApprovalCriteriaInput,
        resource::{Resource, ResourceAction, ResourceId},
        specifier::ProposalSpecifier,
        AddProposalPolicyOperationInput, EditProposalPolicyOperationInput, ProposalPolicy,
        ProposalPolicyCallerPrivileges,
    },
    repositories::policy::{ProposalPolicyRepository, PROPOSAL_POLICY_REPOSITORY},
};
use ic_canister_core::{api::ServiceResult, types::UUID};
use ic_canister_core::{model::ModelValidator, repository::Repository};
use lazy_static::lazy_static;
use std::sync::Arc;
use uuid::Uuid;
use wallet_api::ListProposalPoliciesInput;

lazy_static! {
    pub static ref PROPOSAL_POLICY_SERVICE: Arc<ProposalPolicyService> = Arc::new(
        ProposalPolicyService::new(Arc::clone(&PROPOSAL_POLICY_REPOSITORY))
    );
}

#[derive(Default, Debug)]
pub struct ProposalPolicyService {
    proposal_policy_repository: Arc<ProposalPolicyRepository>,
}

impl ProposalPolicyService {
    pub const DEFAULT_POLICIES_LIMIT: u16 = 100;
    pub const MAX_LIST_POLICIES_LIMIT: u16 = 1000;

    pub fn new(proposal_policy_repository: Arc<ProposalPolicyRepository>) -> Self {
        Self {
            proposal_policy_repository,
        }
    }

    pub fn get_proposal_policy(&self, id: &UUID) -> ServiceResult<ProposalPolicy> {
        let policy =
            self.proposal_policy_repository
                .get(id)
                .ok_or(ProposalError::PolicyNotFound {
                    id: Uuid::from_bytes(*id).hyphenated().to_string(),
                })?;

        Ok(policy)
    }

    pub async fn add_proposal_policy(
        &self,
        input: AddProposalPolicyOperationInput,
    ) -> ServiceResult<ProposalPolicy> {
        let id: uuid::Uuid = generate_uuid_v4().await;
        let policy = ProposalPolicy {
            id: *id.as_bytes(),
            specifier: input.specifier,
            criteria: input.criteria,
        };

        policy.validate()?;

        self.proposal_policy_repository
            .insert(policy.id, policy.clone());

        Ok(policy)
    }

    /// Handles the policy change operation.
    ///
    /// Removes the existing policy if the criteria is `Remove`, otherwise edits the existing policy or adds a new one.
    pub async fn handle_policy_change(
        &self,
        specifier: ProposalSpecifier,
        criteria: ApprovalCriteriaInput,
        editable_policy_id: &mut Option<UUID>,
    ) -> ServiceResult<()> {
        match criteria {
            ApprovalCriteriaInput::Remove => {
                if let Some(existing_policy_id) = editable_policy_id {
                    self.remove_proposal_policy(existing_policy_id).await?;

                    // Directly modify the policy_id in place
                    *editable_policy_id = None;
                }
            }
            ApprovalCriteriaInput::Set(criteria) => {
                match editable_policy_id {
                    Some(existing_policy_id) => {
                        // If there's an existing policy, edit it
                        self.edit_proposal_policy(EditProposalPolicyOperationInput {
                            policy_id: *existing_policy_id,
                            specifier: Some(specifier),
                            criteria: Some(criteria),
                        })
                        .await?;
                    }
                    None => {
                        // If there's no existing policy, add a new one
                        let policy = self
                            .add_proposal_policy(AddProposalPolicyOperationInput {
                                specifier,
                                criteria,
                            })
                            .await?;

                        *editable_policy_id = Some(policy.id);
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn edit_proposal_policy(
        &self,
        input: EditProposalPolicyOperationInput,
    ) -> ServiceResult<ProposalPolicy> {
        let mut policy = self.get_proposal_policy(&input.policy_id)?;

        if let Some(specifier) = input.specifier {
            policy.specifier = specifier;
        }

        if let Some(criteria) = input.criteria {
            policy.criteria = criteria;
        }

        policy.validate()?;

        self.proposal_policy_repository
            .insert(policy.id, policy.to_owned());

        Ok(policy)
    }

    pub async fn remove_proposal_policy(&self, id: &UUID) -> ServiceResult<()> {
        let policy = self.get_proposal_policy(id)?;

        self.proposal_policy_repository.remove(&policy.id);

        Ok(())
    }

    pub async fn get_caller_privileges_for_proposal_policy(
        &self,
        policy_id: &UUID,
        ctx: &CallContext,
    ) -> ServiceResult<ProposalPolicyCallerPrivileges> {
        Ok(ProposalPolicyCallerPrivileges {
            id: *policy_id,
            can_edit: Authorization::is_allowed(
                ctx,
                &Resource::ProposalPolicy(ResourceAction::Update(ResourceId::Id(*policy_id))),
            ),
            can_delete: Authorization::is_allowed(
                ctx,
                &Resource::ProposalPolicy(ResourceAction::Delete(ResourceId::Id(*policy_id))),
            ),
        })
    }

    pub async fn list_proposal_policies(
        &self,
        input: ListProposalPoliciesInput,
        ctx: &CallContext,
    ) -> ServiceResult<PaginatedData<ProposalPolicy>> {
        let mut policies = self.proposal_policy_repository.list();

        retain_accessible_resources(ctx, &mut policies, |policy| {
            Resource::ProposalPolicy(ResourceAction::Read(ResourceId::Id(policy.id)))
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
            account_test_utils::mock_account, criteria::Criteria,
            proposal_policy_test_utils::mock_proposal_policy, resource::ResourceIds,
            specifier::ProposalSpecifier,
        },
    };

    #[tokio::test]
    async fn test_proposal_policy_operations() {
        let service = PROPOSAL_POLICY_SERVICE.clone();
        let policy = service
            .add_proposal_policy(AddProposalPolicyOperationInput {
                specifier: ProposalSpecifier::AddAccount,
                criteria: Criteria::AutoAdopted,
            })
            .await;

        assert!(policy.is_ok());

        let policy = policy.unwrap();
        let fetched_policy = service.get_proposal_policy(&policy.id).unwrap();

        assert_eq!(fetched_policy.specifier, policy.specifier);
        assert_eq!(fetched_policy.criteria, policy.criteria);

        let policy = service
            .edit_proposal_policy(EditProposalPolicyOperationInput {
                policy_id: policy.id,
                specifier: Some(ProposalSpecifier::AddAccount),
                criteria: Some(Criteria::AutoAdopted),
            })
            .await;

        assert!(policy.is_ok());

        let policy = policy.unwrap();
        let updated_policy = service.get_proposal_policy(&policy.id).unwrap();

        assert_eq!(updated_policy.specifier, policy.specifier);
        assert_eq!(updated_policy.criteria, policy.criteria);
    }

    #[test]
    fn test_get_proposal_policy_not_found() {
        let service = PROPOSAL_POLICY_SERVICE.clone();
        let result = service.get_proposal_policy(&[1; 16]);

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn list_proposal_policies_should_use_offset_and_limit() {
        for i in 0..50 {
            let mut policy = mock_proposal_policy();
            policy.id = [i; 16];
            policy.specifier = ProposalSpecifier::AddAccount;
            PROPOSAL_POLICY_REPOSITORY.insert(policy.id, policy.to_owned());
        }

        let input = ListProposalPoliciesInput {
            offset: Some(15),
            limit: Some(30),
        };

        let result = PROPOSAL_POLICY_SERVICE
            .list_proposal_policies(input, &CallContext::new(self_canister_id()))
            .await
            .unwrap();
        assert_eq!(result.items.len(), 30);
        assert_eq!(result.next_offset, Some(45));
    }

    #[tokio::test]
    async fn test_remove_proposal_policy() {
        let service = PROPOSAL_POLICY_SERVICE.clone();
        let policy = service
            .add_proposal_policy(AddProposalPolicyOperationInput {
                specifier: ProposalSpecifier::AddAccount,
                criteria: Criteria::AutoAdopted,
            })
            .await
            .unwrap();

        assert!(service.get_proposal_policy(&policy.id).is_ok());

        service.remove_proposal_policy(&policy.id).await.unwrap();

        assert!(service.get_proposal_policy(&policy.id).is_err());
    }

    #[tokio::test]
    async fn test_handle_policy_change() {
        let mut account = mock_account();
        account.update_approval_policy_id = None;

        PROPOSAL_POLICY_SERVICE
            .handle_policy_change(
                ProposalSpecifier::EditAccount(ResourceIds::Ids(vec![account.id])),
                ApprovalCriteriaInput::Set(Criteria::AutoAdopted),
                &mut account.update_approval_policy_id,
            )
            .await
            .unwrap();

        assert!(account.update_approval_policy_id.is_some());

        PROPOSAL_POLICY_SERVICE
            .handle_policy_change(
                ProposalSpecifier::EditAccount(ResourceIds::Ids(vec![account.id])),
                ApprovalCriteriaInput::Remove,
                &mut account.update_approval_policy_id,
            )
            .await
            .unwrap();

        assert!(account.update_approval_policy_id.is_none());
    }
}
