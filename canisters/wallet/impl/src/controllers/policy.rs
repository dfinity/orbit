use crate::{
    core::middlewares::{authorize, call_context},
    mappers::HelperMapper,
    models::access_control::{
        AccessPolicyActionSpecifier, ProposalPolicyActionSpecifier, ResourceSpecifier, ResourceType,
    },
    services::{PolicyService, POLICY_SERVICE},
};
use ic_canister_core::api::ApiResult;
use ic_canister_macros::with_middleware;
use ic_cdk_macros::query;
use lazy_static::lazy_static;
use std::sync::Arc;
use wallet_api::{
    GetAccessPolicyInput, GetAccessPolicyResponse, GetProposalPolicyInput,
    GetProposalPolicyResponse, ListAccessPoliciesInput, ListAccessPoliciesResponse,
    ListProposalPoliciesInput, ListProposalPoliciesResponse,
};

// Canister entrypoints for the controller.
#[query(name = "get_access_policy")]
async fn get_access_policy(input: GetAccessPolicyInput) -> ApiResult<GetAccessPolicyResponse> {
    CONTROLLER.get_access_policy(input).await
}

#[query(name = "list_access_policies")]
async fn list_access_policies(
    input: ListAccessPoliciesInput,
) -> ApiResult<ListAccessPoliciesResponse> {
    CONTROLLER.list_access_policies(input).await
}

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
    static ref CONTROLLER: PolicyController = PolicyController::new(Arc::clone(&POLICY_SERVICE));
}

#[derive(Debug)]
pub struct PolicyController {
    policy_service: Arc<PolicyService>,
}

impl PolicyController {
    fn new(policy_service: Arc<PolicyService>) -> Self {
        Self { policy_service }
    }

    #[with_middleware(
        guard = "authorize",
        context = "call_context",
        args = [ResourceSpecifier::from(&input)],
        is_async = true
    )]
    async fn get_access_policy(
        &self,
        input: GetAccessPolicyInput,
    ) -> ApiResult<GetAccessPolicyResponse> {
        let access_policy = self
            .policy_service
            .get_access_policy(HelperMapper::to_uuid(input.id)?.as_bytes())?;

        Ok(GetAccessPolicyResponse {
            policy: access_policy.into(),
        })
    }

    #[with_middleware(
        guard = "authorize",
        context = "call_context",
        args = [ResourceSpecifier::Common(ResourceType::AccessPolicy, AccessPolicyActionSpecifier::List)],
        is_async = true
    )]
    async fn list_access_policies(
        &self,
        input: ListAccessPoliciesInput,
    ) -> ApiResult<ListAccessPoliciesResponse> {
        let list = self.policy_service.list_access_policies(input)?;
        let deps = self.policy_service.get_access_policies_dependencies(&list.items)?;

        Ok(ListAccessPoliciesResponse {
            policies: list.items.into_iter().map(Into::into).collect(),
            user_groups: deps.groups.into_iter().map(Into::into).collect(),
            users: deps.users.into_iter().map(Into::into).collect(),
            next_offset: list.next_offset,
            total: list.total,
        })
    }

    #[with_middleware(
        guard = "authorize",
        context = "call_context",
        args = [ResourceSpecifier::from(&input)],
        is_async = true
    )]
    async fn get_proposal_policy(
        &self,
        input: GetProposalPolicyInput,
    ) -> ApiResult<GetProposalPolicyResponse> {
        let proposal_policy = self
            .policy_service
            .get_proposal_policy(HelperMapper::to_uuid(input.id)?.as_bytes())?;

        Ok(GetProposalPolicyResponse {
            policy: proposal_policy.into(),
        })
    }

    #[with_middleware(
        guard = "authorize",
        context = "call_context",
        args = [ResourceSpecifier::Common(ResourceType::ProposalPolicy, ProposalPolicyActionSpecifier::List)],
        is_async = true
    )]
    async fn list_proposal_policies(
        &self,
        input: ListProposalPoliciesInput,
    ) -> ApiResult<ListProposalPoliciesResponse> {
        let list = self.policy_service.list_proposal_policies(input)?;

        Ok(ListProposalPoliciesResponse {
            policies: list.items.into_iter().map(Into::into).collect(),
            next_offset: list.next_offset,
            total: list.total,
        })
    }
}
