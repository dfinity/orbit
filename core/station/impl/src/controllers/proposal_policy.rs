use crate::{
    core::middlewares::{authorize, call_context},
    mappers::HelperMapper,
    models::resource::{Resource, ResourceAction},
    services::{ProposalPolicyService, PROPOSAL_POLICY_SERVICE},
};
use ic_cdk_macros::query;
use lazy_static::lazy_static;
use orbit_essentials::api::ApiResult;
use orbit_essentials::with_middleware;
use station_api::{
    GetProposalPolicyInput, GetProposalPolicyResponse, ListProposalPoliciesInput,
    ListProposalPoliciesResponse, ProposalPolicyCallerPrivilegesDTO,
};
use std::sync::Arc;

// Canister entrypoints for the controller.
#[query(name = "get_proposal_policy")]
async fn get_proposal_policy(
    input: GetProposalPolicyInput,
) -> ApiResult<GetProposalPolicyResponse> {
    CONTROLLER.get_proposal_policy(input).await
}

#[query(name = "list_proposal_policies")]
async fn list_proposal_policies(
    input: ListProposalPoliciesInput,
) -> ApiResult<ListProposalPoliciesResponse> {
    CONTROLLER.list_proposal_policies(input).await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: ProposalPolicyController =
        ProposalPolicyController::new(Arc::clone(&PROPOSAL_POLICY_SERVICE));
}

#[derive(Debug)]
pub struct ProposalPolicyController {
    proposal_policy_service: Arc<ProposalPolicyService>,
}

impl ProposalPolicyController {
    fn new(proposal_policy_service: Arc<ProposalPolicyService>) -> Self {
        Self {
            proposal_policy_service,
        }
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::from(&input)]))]
    async fn get_proposal_policy(
        &self,
        input: GetProposalPolicyInput,
    ) -> ApiResult<GetProposalPolicyResponse> {
        let ctx = call_context();
        let proposal_policy = self
            .proposal_policy_service
            .get_proposal_policy(HelperMapper::to_uuid(input.id)?.as_bytes())?;
        let privileges = self
            .proposal_policy_service
            .get_caller_privileges_for_proposal_policy(&proposal_policy.id, &ctx)
            .await?;

        Ok(GetProposalPolicyResponse {
            policy: proposal_policy.to_dto(),
            privileges: privileges.into(),
        })
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::ProposalPolicy(ResourceAction::List)]))]
    async fn list_proposal_policies(
        &self,
        input: ListProposalPoliciesInput,
    ) -> ApiResult<ListProposalPoliciesResponse> {
        let ctx = call_context();
        let result = self
            .proposal_policy_service
            .list_proposal_policies(input, &ctx)
            .await?;

        let mut privileges = Vec::new();
        for policy in &result.items {
            let privilege = self
                .proposal_policy_service
                .get_caller_privileges_for_proposal_policy(&policy.id, &ctx)
                .await?;

            privileges.push(ProposalPolicyCallerPrivilegesDTO::from(privilege));
        }

        Ok(ListProposalPoliciesResponse {
            policies: result.items.into_iter().map(|p| p.to_dto()).collect(),
            next_offset: result.next_offset,
            total: result.total,
            privileges,
        })
    }
}
