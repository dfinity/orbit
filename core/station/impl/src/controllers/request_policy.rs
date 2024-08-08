use crate::{
    core::middlewares::{authorize, call_context},
    mappers::HelperMapper,
    models::resource::{Resource, ResourceAction},
    services::{RequestPolicyService, REQUEST_POLICY_SERVICE},
};
use ic_cdk_macros::query;
use lazy_static::lazy_static;
use orbit_essentials::api::ApiResult;
use orbit_essentials::with_middleware;
use station_api::{
    GetRequestPolicyInput, GetRequestPolicyResponse, ListRequestPoliciesInput,
    ListRequestPoliciesResponse, RequestPolicyCallerPrivilegesDTO,
};
use std::sync::Arc;

// Canister entrypoints for the controller.
#[query(name = "get_request_policy")]
async fn get_request_policy(input: GetRequestPolicyInput) -> ApiResult<GetRequestPolicyResponse> {
    CONTROLLER.get_request_policy(input).await
}

#[query(name = "list_request_policies")]
async fn list_request_policies(
    input: ListRequestPoliciesInput,
) -> ApiResult<ListRequestPoliciesResponse> {
    CONTROLLER.list_request_policies(input).await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: RequestPolicyController =
        RequestPolicyController::new(Arc::clone(&REQUEST_POLICY_SERVICE));
}

#[derive(Debug)]
pub struct RequestPolicyController {
    request_policy_service: Arc<RequestPolicyService>,
}

impl RequestPolicyController {
    fn new(request_policy_service: Arc<RequestPolicyService>) -> Self {
        Self {
            request_policy_service,
        }
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::from(&input)]))]
    async fn get_request_policy(
        &self,
        input: GetRequestPolicyInput,
    ) -> ApiResult<GetRequestPolicyResponse> {
        let ctx = call_context();
        let request_policy = self
            .request_policy_service
            .get_request_policy(HelperMapper::to_uuid(input.id)?.as_bytes())?;
        let privileges = self
            .request_policy_service
            .get_caller_privileges_for_request_policy(&request_policy.id, &ctx)?;

        Ok(GetRequestPolicyResponse {
            policy: request_policy.to_dto(),
            privileges: privileges.into(),
        })
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::RequestPolicy(ResourceAction::List)]))]
    async fn list_request_policies(
        &self,
        input: ListRequestPoliciesInput,
    ) -> ApiResult<ListRequestPoliciesResponse> {
        let ctx = call_context();
        let result = self
            .request_policy_service
            .list_request_policies(input, &ctx)?;

        let mut privileges = Vec::new();
        for policy in &result.items {
            let privilege = self
                .request_policy_service
                .get_caller_privileges_for_request_policy(&policy.id, &ctx)?;

            privileges.push(RequestPolicyCallerPrivilegesDTO::from(privilege));
        }

        Ok(ListRequestPoliciesResponse {
            policies: result.items.into_iter().map(|p| p.to_dto()).collect(),
            next_offset: result.next_offset,
            total: result.total,
            privileges,
        })
    }
}
