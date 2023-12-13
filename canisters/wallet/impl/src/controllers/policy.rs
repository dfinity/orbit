use crate::{
    mappers::HelperMapper,
    services::{PolicyService, POLICY_SERVICE},
};
use ic_canister_core::api::ApiResult;
use ic_cdk_macros::query;
use lazy_static::lazy_static;
use std::sync::Arc;
use wallet_api::{
    GetAccessPolicyInput, GetAccessPolicyResponse, ListAccessPoliciesInput,
    ListAccessPoliciesResponse,
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

    async fn list_access_policies(
        &self,
        input: ListAccessPoliciesInput,
    ) -> ApiResult<ListAccessPoliciesResponse> {
        let list = self.policy_service.list_access_policies(input)?;

        Ok(ListAccessPoliciesResponse {
            policies: list.items.into_iter().map(Into::into).collect(),
            next_offset: list.next_offset,
        })
    }
}
