use crate::{
    core::generate_uuid_v4,
    models::{
        access_control::{AccessControlPolicy, ResourceSpecifier, UserSpecifier},
        criteria::Criteria,
        specifier::ProposalSpecifier,
        ProposalPolicy,
    },
    repositories::{access_control::AccessControlRepository, policy::ProposalPolicyRepository},
};
use ic_canister_core::api::ServiceResult;
use ic_canister_core::repository::Repository;
use lazy_static::lazy_static;

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
}
