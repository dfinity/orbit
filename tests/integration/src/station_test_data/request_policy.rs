use crate::utils::{submit_request, wait_for_request};
use candid::Principal;
use orbit_essentials::api::ApiResult;
use pocket_ic::{query_candid_as, PocketIc, RejectResponse};
use station_api::{
    EditRequestPolicyOperationInput, ListRequestPoliciesInput, ListRequestPoliciesResponse,
};

pub fn add_request_policy(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
    specifier: station_api::RequestSpecifierDTO,
) {
    let edit_account_request = submit_request(
        env,
        requester,
        station_canister_id,
        station_api::RequestOperationInput::AddRequestPolicy(
            station_api::AddRequestPolicyOperationInput {
                specifier,
                rule: station_api::RequestPolicyRuleDTO::AutoApproved,
            },
        ),
    );

    wait_for_request(env, requester, station_canister_id, edit_account_request)
        .expect("Failed to add new request policy");
}

pub fn list_request_policies(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
) -> Result<(ApiResult<ListRequestPoliciesResponse>,), RejectResponse> {
    query_candid_as::<(ListRequestPoliciesInput,), (ApiResult<ListRequestPoliciesResponse>,)>(
        env,
        station_canister_id,
        requester,
        "list_request_policies",
        (ListRequestPoliciesInput {
            offset: Some(0),
            limit: Some(25),
        },),
    )
}

pub fn edit_request_policy(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
    input: EditRequestPolicyOperationInput,
) -> Result<(), RejectResponse> {
    let edit_request_policy_request = submit_request(
        env,
        requester,
        station_canister_id,
        station_api::RequestOperationInput::EditRequestPolicy(input),
    );

    wait_for_request(
        env,
        requester,
        station_canister_id,
        edit_request_policy_request,
    )
    .expect("Failed to edit request policy");

    Ok(())
}
