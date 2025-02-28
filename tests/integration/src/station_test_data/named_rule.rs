use super::next_unique_id;
use crate::utils::{submit_request, wait_for_request};
use candid::Principal;
use orbit_essentials::api::ApiResult;
use pocket_ic::{query_candid_as, PocketIc, RejectResponse};
use station_api::{
    GetNamedRuleInput, GetNamedRuleResponse, ListNamedRulesInput, ListNamedRulesResponse,
    RequestPolicyRuleDTO,
};

pub fn add_named_rule_with_input(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
    input: station_api::AddNamedRuleOperationInput,
) -> station_api::NamedRuleDTO {
    let add_named_rule_request = submit_request(
        env,
        requester,
        station_canister_id,
        station_api::RequestOperationInput::AddNamedRule(input),
    );

    let request = wait_for_request(env, requester, station_canister_id, add_named_rule_request)
        .expect("Failed to add named rule");

    match request.operation {
        station_api::RequestOperationDTO::AddNamedRule(add_named_rule) => {
            add_named_rule.named_rule.unwrap()
        }
        _ => panic!("invalid request operation"),
    }
}

#[allow(dead_code)]
pub fn add_named_rule(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
) -> station_api::NamedRuleDTO {
    let next_id = next_unique_id();

    add_named_rule_with_input(
        env,
        station_canister_id,
        requester,
        station_api::AddNamedRuleOperationInput {
            name: format!("rule-{}", next_id),
            description: Some(format!("Description for rule-{}", next_id)),
            rule: RequestPolicyRuleDTO::AutoApproved,
        },
    )
}

#[allow(dead_code)]
pub fn edit_named_rule(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
    named_rule_id: station_api::UuidDTO,
    name: Option<String>,
    description: Option<Option<String>>,
    rule: Option<RequestPolicyRuleDTO>,
) {
    let edit_named_rule_request = submit_request(
        env,
        requester,
        station_canister_id,
        station_api::RequestOperationInput::EditNamedRule(
            station_api::EditNamedRuleOperationInput {
                named_rule_id,
                name,
                description,
                rule,
            },
        ),
    );

    wait_for_request(env, requester, station_canister_id, edit_named_rule_request)
        .expect("Failed to edit named rule");
}

#[allow(dead_code)]
pub fn remove_named_rule(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
    named_rule_id: station_api::UuidDTO,
) {
    let remove_named_rule_request = submit_request(
        env,
        requester,
        station_canister_id,
        station_api::RequestOperationInput::RemoveNamedRule(
            station_api::RemoveNamedRuleOperationInput { named_rule_id },
        ),
    );

    wait_for_request(
        env,
        requester,
        station_canister_id,
        remove_named_rule_request,
    )
    .expect("Failed to remove named rule");
}

pub fn list_named_rules(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
) -> Result<(ApiResult<ListNamedRulesResponse>,), RejectResponse> {
    query_candid_as::<(ListNamedRulesInput,), (ApiResult<ListNamedRulesResponse>,)>(
        env,
        station_canister_id,
        requester,
        "list_named_rules",
        (ListNamedRulesInput { paginate: None },),
    )
}

#[allow(dead_code)]
pub fn get_named_rule(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
    named_rule_id: station_api::UuidDTO,
) -> Result<(ApiResult<GetNamedRuleResponse>,), RejectResponse> {
    query_candid_as::<(GetNamedRuleInput,), (ApiResult<GetNamedRuleResponse>,)>(
        env,
        station_canister_id,
        requester,
        "get_named_rule",
        (GetNamedRuleInput { named_rule_id },),
    )
}
