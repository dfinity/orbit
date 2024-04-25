use crate::{
    core::{
        authorization::Authorization,
        middlewares::{authorize, call_context},
    },
    models::resource::{AccessPolicyResourceAction, Resource},
    services::access_policy::{AccessPolicyService, ACCESS_POLICY_SERVICE},
};
use ic_canister_core::api::ApiResult;
use ic_canister_macros::with_middleware;
use ic_cdk_macros::query;
use lazy_static::lazy_static;
use station_api::{
    AccessPolicyCallerPrivilegesDTO, GetAccessPolicyInput, GetAccessPolicyResponse,
    ListAccessPoliciesInput, ListAccessPoliciesResponse,
};
use std::sync::Arc;

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

    #[with_middleware(guard = authorize(&call_context(), &[Resource::AccessPolicy(AccessPolicyResourceAction::Read)]))]
    async fn get_access_policy(
        &self,
        input: GetAccessPolicyInput,
    ) -> ApiResult<GetAccessPolicyResponse> {
        let ctx = call_context();
        let policy = self
            .access_policy_service
            .get_access_policy(&Resource::from(input.resource));

        Ok(GetAccessPolicyResponse {
            policy: policy.clone().into(),
            privileges: station_api::AccessPolicyCallerPrivilegesDTO {
                can_edit: Authorization::is_allowed(
                    &ctx,
                    &Resource::AccessPolicy(AccessPolicyResourceAction::Update),
                ),
                resource: policy.resource.into(),
            },
        })
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::AccessPolicy(AccessPolicyResourceAction::Read)]))]
    async fn list_access_policies(
        &self,
        input: ListAccessPoliciesInput,
    ) -> ApiResult<ListAccessPoliciesResponse> {
        let ctx = call_context();
        let result = self
            .access_policy_service
            .list_access_policies(input)
            .await?;
        let deps = self
            .access_policy_service
            .get_access_policies_dependencies(&result.items)?;

        let can_edit = Authorization::is_allowed(
            &ctx,
            &Resource::AccessPolicy(AccessPolicyResourceAction::Update),
        );
        let mut privileges = Vec::new();
        for policy in &result.items {
            privileges.push(AccessPolicyCallerPrivilegesDTO {
                can_edit,
                resource: policy.resource.clone().into(),
            });
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
