use crate::{
    core::middlewares::{authorize, call_context},
    models::access_policy::{AccessPolicyResourceAction, Resource},
    services::access_policy::{AccessPolicyService, ACCESS_POLICY_SERVICE},
};
use ic_canister_core::api::ApiResult;
use ic_canister_macros::with_middleware;
use ic_cdk_macros::query;
use lazy_static::lazy_static;
use std::sync::Arc;
use wallet_api::{
    AccessPolicyCallerPrivilegesDTO, GetAccessPolicyInput, GetAccessPolicyResponse,
    ListAccessPoliciesInput, ListAccessPoliciesResponse,
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

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: AccessPolicyController =
        AccessPolicyController::new(Arc::clone(&ACCESS_POLICY_SERVICE));
}

#[derive(Debug)]
pub struct AccessPolicyController {
    access_policy_service: Arc<AccessPolicyService>,
}

impl AccessPolicyController {
    fn new(access_policy_service: Arc<AccessPolicyService>) -> Self {
        Self {
            access_policy_service,
        }
    }

    #[with_middleware(guard = "authorize", context = "call_context", args = [Resource::from(&input)])]
    async fn get_access_policy(
        &self,
        input: GetAccessPolicyInput,
    ) -> ApiResult<GetAccessPolicyResponse> {
        let ctx = call_context();
        let policies = self
            .access_policy_service
            .get_access_policy(&Resource::from(input.resource))?;

        let mut privileges = Vec::new();
        for policy in &policies {
            let privilege = self
                .access_policy_service
                .get_caller_privileges_for_access_policy(&policy.resource.to_type(), &ctx)
                .await?;

            privileges.push(AccessPolicyCallerPrivilegesDTO::from(privilege));
        }

        Ok(GetAccessPolicyResponse {
            policies: policies.into_iter().map(|p| p.into()).collect(),
            privileges,
        })
    }

    #[with_middleware(
        guard = "authorize",
        context = "call_context",
        args = [Resource::AccessPolicy(AccessPolicyResourceAction::List)],
    )]
    async fn list_access_policies(
        &self,
        input: ListAccessPoliciesInput,
    ) -> ApiResult<ListAccessPoliciesResponse> {
        let ctx = call_context();
        let result = self
            .access_policy_service
            .list_access_policies(input, &ctx)
            .await?;
        let deps = self
            .access_policy_service
            .get_access_policies_dependencies(&result.items)?;

        let mut privileges = Vec::new();
        for policy in &result.items {
            let privilege = self
                .access_policy_service
                .get_caller_privileges_for_access_policy(&policy.resource.to_type(), &ctx)
                .await?;

            privileges.push(AccessPolicyCallerPrivilegesDTO::from(privilege));
        }

        Ok(ListAccessPoliciesResponse {
            policies: result.items.into_iter().map(|p| p.into()).collect(),
            user_groups: deps.groups.into_iter().map(Into::into).collect(),
            users: deps.users.into_iter().map(Into::into).collect(),
            next_offset: result.next_offset,
            total: result.total,
            privileges,
        })
    }
}
