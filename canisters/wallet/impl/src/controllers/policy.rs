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
        let privileges = self
            .policy_service
            .get_caller_privileges_for_access_policy(&access_policy.id, &call_context())
            .await?;

        Ok(GetAccessPolicyResponse {
            policy: access_policy.to_dto(),
            privileges: privileges.into(),
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
        let ctx = &call_context();
        let result = self.policy_service.list_access_policies(input, ctx).await?;
        let deps = self
            .policy_service
            .get_access_policies_dependencies(&result.items)?;

        let mut privileges = Vec::new();
        for policy in &result.items {
            let privilege = self
                .policy_service
                .get_caller_privileges_for_access_policy(&policy.id, &call_context())
                .await?;

            privileges.push(privilege);
        }

        Ok(ListAccessPoliciesResponse {
            policies: result.items.into_iter().map(|p| p.to_dto()).collect(),
            user_groups: deps.groups.into_iter().map(Into::into).collect(),
            users: deps.users.into_iter().map(Into::into).collect(),
            next_offset: result.next_offset,
            total: result.total,
            privileges: privileges.into_iter().map(Into::into).collect(),
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
        let ctx = call_context();
        let proposal_policy = self
            .policy_service
            .get_proposal_policy(HelperMapper::to_uuid(input.id)?.as_bytes())?;
        let privileges = self
            .policy_service
            .get_caller_privileges_for_proposal_policy(&proposal_policy.id, &ctx)
            .await?;

        Ok(GetProposalPolicyResponse {
            policy: proposal_policy.to_dto(),
            privileges: privileges.into(),
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
        let ctx = call_context();
        let result = self
            .policy_service
            .list_proposal_policies(input, &ctx)
            .await?;

        let mut privileges = Vec::new();
        for policy in &result.items {
            let privilege = self
                .policy_service
                .get_caller_privileges_for_proposal_policy(&policy.id, &ctx)
                .await?;

            privileges.push(privilege);
        }

        Ok(ListProposalPoliciesResponse {
            policies: result.items.into_iter().map(|p| p.to_dto()).collect(),
            next_offset: result.next_offset,
            total: result.total,
            privileges: privileges.into_iter().map(Into::into).collect(),
        })
    }
}
