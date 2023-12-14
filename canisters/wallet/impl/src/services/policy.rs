use crate::{
    core::{
        generate_uuid_v4,
        utils::{paginated_items, PaginatedData, PaginatedItemsArgs},
    },
    errors::{AccessControlError, ProposalError},
    models::{
        access_control::AccessControlPolicy, criteria::Criteria, specifier::ProposalSpecifier,
        AddAccessPolicyOperationInput, EditAccessPolicyOperationInput, ProposalPolicy,
    },
    repositories::{
        access_control::{AccessControlRepository, ACCESS_CONTROL_REPOSITORY},
        policy::{ProposalPolicyRepository, PROPOSAL_POLICY_REPOSITORY},
    },
};
use ic_canister_core::repository::Repository;
use ic_canister_core::{api::ServiceResult, types::UUID};
use lazy_static::lazy_static;
use std::sync::Arc;
use uuid::Uuid;
use wallet_api::{ListAccessPoliciesInput, ListProposalPoliciesInput};

lazy_static! {
    pub static ref POLICY_SERVICE: Arc<PolicyService> = Arc::new(PolicyService::new(
        Arc::clone(&ACCESS_CONTROL_REPOSITORY),
        Arc::clone(&PROPOSAL_POLICY_REPOSITORY),
    ));
}

#[derive(Default, Debug)]
pub struct PolicyService {
    access_control_policy_repository: Arc<AccessControlRepository>,
    proposal_policy_repository: Arc<ProposalPolicyRepository>,
}

impl PolicyService {
    pub const DEFAULT_POLICIES_LIMIT: u16 = 100;
    pub const MAX_LIST_POLICIES_LIMIT: u16 = 1000;

    pub fn new(
        access_control_policy_repository: Arc<AccessControlRepository>,
        proposal_policy_repository: Arc<ProposalPolicyRepository>,
    ) -> Self {
        Self {
            access_control_policy_repository,
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
        specifier: ProposalSpecifier,
        criteria: Criteria,
    ) -> ServiceResult<ProposalPolicy> {
        let id: uuid::Uuid = generate_uuid_v4().await;
        let policy = ProposalPolicy {
            id: *id.as_bytes(),
            specifier,
            criteria,
        };

        self.proposal_policy_repository
            .insert(policy.id, policy.clone());

        Ok(policy)
    }

    pub async fn edit_proposal_policy(
        &self,
        id: &UUID,
        specifier: ProposalSpecifier,
        criteria: Criteria,
    ) -> ServiceResult<ProposalPolicy> {
        let mut policy = self.get_proposal_policy(id)?;

        policy.specifier = specifier;
        policy.criteria = criteria;

        self.proposal_policy_repository
            .insert(policy.id, policy.to_owned());

        Ok(policy)
    }

    pub fn get_access_policy(&self, id: &UUID) -> ServiceResult<AccessControlPolicy> {
        let policy = self.access_control_policy_repository.get(id).ok_or(
            AccessControlError::PolicyNotFound {
                id: Uuid::from_bytes(*id).hyphenated().to_string(),
            },
        )?;

        Ok(policy)
    }

    pub async fn add_access_policy(
        &self,
        input: AddAccessPolicyOperationInput,
    ) -> ServiceResult<AccessControlPolicy> {
        let id: uuid::Uuid = generate_uuid_v4().await;
        let policy = AccessControlPolicy {
            id: *id.as_bytes(),
            user: input.user,
            resource: input.resource,
        };

        self.access_control_policy_repository
            .insert(policy.id, policy.clone());

        Ok(policy)
    }

    pub async fn remove_access_policy(&self, input: &UUID) -> ServiceResult<()> {
        let policy = self.get_access_policy(input)?;

        self.access_control_policy_repository.remove(&policy.id);

        Ok(())
    }

    pub async fn edit_access_policy(
        &self,
        input: EditAccessPolicyOperationInput,
    ) -> ServiceResult<AccessControlPolicy> {
        let mut policy = self.get_access_policy(&input.policy_id)?;

        if let Some(user) = input.user {
            policy.user = user;
        }

        if let Some(resource) = input.resource {
            policy.resource = resource;
        }

        self.access_control_policy_repository
            .insert(policy.id, policy.to_owned());

        Ok(policy)
    }

    pub fn list_access_policies(
        &self,
        input: ListAccessPoliciesInput,
    ) -> ServiceResult<PaginatedData<AccessControlPolicy>> {
        let result = paginated_items(PaginatedItemsArgs {
            offset: input.offset,
            limit: input.limit,
            default_limit: Some(Self::DEFAULT_POLICIES_LIMIT),
            max_limit: Some(Self::MAX_LIST_POLICIES_LIMIT),
            items: Box::new(|| self.access_control_policy_repository.list()),
        })?;

        Ok(result)
    }

    pub fn list_proposal_policies(
        &self,
        input: ListProposalPoliciesInput,
    ) -> ServiceResult<PaginatedData<ProposalPolicy>> {
        let result = paginated_items(PaginatedItemsArgs {
            offset: input.offset,
            limit: input.limit,
            default_limit: Some(Self::DEFAULT_POLICIES_LIMIT),
            max_limit: Some(Self::MAX_LIST_POLICIES_LIMIT),
            items: Box::new(|| self.proposal_policy_repository.list()),
        })?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        access_control::{
            access_control_test_utils::mock_access_policy, ProposalActionSpecifier,
            ResourceSpecifier, UserSpecifier,
        },
        proposal_policy_test_utils::mock_proposal_policy,
    };

    #[tokio::test]
    async fn test_proposal_policy_operations() {
        let service = POLICY_SERVICE.clone();
        let policy = service
            .add_proposal_policy(ProposalSpecifier::AddAccount, Criteria::AutoAdopted)
            .await;

        assert!(policy.is_ok());

        let policy = policy.unwrap();
        let fetched_policy = service.get_proposal_policy(&policy.id).unwrap();

        assert_eq!(fetched_policy.specifier, policy.specifier);
        assert_eq!(fetched_policy.criteria, policy.criteria);

        let policy = service
            .edit_proposal_policy(
                &policy.id,
                ProposalSpecifier::AddAccount,
                Criteria::AutoAdopted,
            )
            .await;

        assert!(policy.is_ok());

        let policy = policy.unwrap();
        let updated_policy = service.get_proposal_policy(&policy.id).unwrap();

        assert_eq!(updated_policy.specifier, policy.specifier);
        assert_eq!(updated_policy.criteria, policy.criteria);
    }

    #[tokio::test]
    async fn test_access_policy_operations() {
        let service = POLICY_SERVICE.clone();
        let policy = service
            .add_access_policy(AddAccessPolicyOperationInput {
                user: UserSpecifier::Any,
                resource: ResourceSpecifier::Proposal(ProposalActionSpecifier::List),
            })
            .await;

        assert!(policy.is_ok());

        let policy = policy.unwrap();
        let fetched_policy = service.get_access_policy(&policy.id).unwrap();

        assert_eq!(fetched_policy.user, policy.user);
        assert_eq!(fetched_policy.resource, policy.resource);

        let policy = service
            .edit_access_policy(EditAccessPolicyOperationInput {
                policy_id: policy.id,
                user: Some(UserSpecifier::Id(vec![[1; 16]])),
                resource: Some(ResourceSpecifier::Proposal(ProposalActionSpecifier::List)),
            })
            .await;

        assert!(policy.is_ok());

        let policy = policy.unwrap();
        let updated_policy = service.get_access_policy(&policy.id).unwrap();

        assert_eq!(updated_policy.user, policy.user);
        assert_eq!(updated_policy.resource, policy.resource);
    }

    #[test]
    fn test_get_proposal_policy_not_found() {
        let service = POLICY_SERVICE.clone();
        let result = service.get_proposal_policy(&[1; 16]);

        assert!(result.is_err());
    }

    #[test]
    fn test_get_access_policy_not_found() {
        let service = POLICY_SERVICE.clone();
        let result = service.get_access_policy(&[1; 16]);

        assert!(result.is_err());
    }

    #[test]
    fn list_access_policies_should_use_offset_and_limit() {
        for i in 0..50 {
            let mut policy = mock_access_policy();
            policy.id = [i; 16];
            policy.user = UserSpecifier::Id(vec![[i; 16]]);
            ACCESS_CONTROL_REPOSITORY.insert(policy.id, policy.to_owned());
        }

        let input = ListAccessPoliciesInput {
            offset: Some(15),
            limit: Some(30),
        };

        let result = POLICY_SERVICE.list_access_policies(input).unwrap();
        assert_eq!(result.items.len(), 30);
        assert_eq!(result.next_offset, Some(45));
    }

    #[test]
    fn list_proposal_policies_should_use_offset_and_limit() {
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

        let result = POLICY_SERVICE.list_proposal_policies(input).unwrap();
        assert_eq!(result.items.len(), 30);
        assert_eq!(result.next_offset, Some(45));
    }

    #[tokio::test]
    async fn test_remove_access_policy() {
        let service = POLICY_SERVICE.clone();
        let policy = service
            .add_access_policy(AddAccessPolicyOperationInput {
                user: UserSpecifier::Any,
                resource: ResourceSpecifier::Proposal(ProposalActionSpecifier::List),
            })
            .await
            .unwrap();

        assert!(service.get_access_policy(&policy.id).is_ok());

        service.remove_access_policy(&policy.id).await.unwrap();

        assert!(service.get_access_policy(&policy.id).is_err());
    }
}
