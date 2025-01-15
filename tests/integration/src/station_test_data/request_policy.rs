use crate::utils::{submit_request, wait_for_request};
use candid::Principal;
use pocket_ic::PocketIc;

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
