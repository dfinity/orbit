use crate::{
    core::generate_uuid_v4,
    errors::{AccessControlError, ProposalError},
    models::{
        access_control::{AccessControlPolicy, ResourceSpecifier, UserSpecifier},
        criteria::Criteria,
        specifier::ProposalSpecifier,
        ProposalPolicy,
    },
    repositories::{access_control::AccessControlRepository, policy::ProposalPolicyRepository},
};
use ic_canister_core::repository::Repository;
use ic_canister_core::{api::ServiceResult, types::UUID};
use lazy_static::lazy_static;
use uuid::Uuid;

lazy_static! {
    pub static ref POLICY_SERVICE: PolicyService =
        PolicyService::new(AccessControlRepository::default(), ProposalPolicyRepository,);
}

#[derive(Default, Debug)]
pub struct PolicyService {
    access_control_policy_repository: AccessControlRepository,
    proposal_policy_repository: ProposalPolicyRepository,
}

impl PolicyService {
    pub fn new(
        access_control_policy_repository: AccessControlRepository,
        proposal_policy_repository: ProposalPolicyRepository,
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
        specifier: UserSpecifier,
        resource: ResourceSpecifier,
    ) -> ServiceResult<AccessControlPolicy> {
        let id: uuid::Uuid = generate_uuid_v4().await;
        let policy = AccessControlPolicy {
            id: *id.as_bytes(),
            user: specifier,
            resource,
        };

        self.access_control_policy_repository
            .insert(policy.id, policy.clone());

        Ok(policy)
    }

    pub async fn edit_access_policy(
        &self,
        id: &UUID,
        specifier: UserSpecifier,
        resource: ResourceSpecifier,
    ) -> ServiceResult<AccessControlPolicy> {
        let mut policy = self.get_access_policy(id)?;

        policy.user = specifier;
        policy.resource = resource;

        self.access_control_policy_repository
            .insert(policy.id, policy.to_owned());

        Ok(policy)
    }
}
