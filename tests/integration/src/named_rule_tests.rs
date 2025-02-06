use std::time::Duration;

use station_api::{
    AddAddressBookEntryOperationInput, AddNamedRuleOperationInput, EditRequestPolicyOperationInput,
    QuorumDTO, RequestApprovalStatusDTO, RequestOperationInput, RequestPolicyRuleDTO,
    RequestSpecifierDTO, RequestStatusDTO,
};

use crate::{
    setup::{setup_new_env, WALLET_ADMIN_USER},
    station_test_data::{
        named_rule::{add_named_rule_with_input, list_named_rules},
        request_policy::{edit_request_policy, list_request_policies},
        user::add_user,
    },
    utils::{get_request, submit_request, submit_request_approval},
    TestEnv,
};

#[test]
fn no_named_rules_by_default() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let named_rules = list_named_rules(&env, canister_ids.station, WALLET_ADMIN_USER)
        .expect("Failed to call list named rules");

    let response = named_rules.0.expect("Failed to get response");
    assert_eq!(response.named_rules.len(), 0);
}

#[test]
fn can_replace_initial_policy_with_named_rule() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let other_user = add_user(&env, canister_ids.station, WALLET_ADMIN_USER, vec![]);

    let policies_result = list_request_policies(&env, canister_ids.station, WALLET_ADMIN_USER)
        .expect("Failed to call list request policies");

    let response = policies_result.0.expect("Failed to get response");

    let add_address_book_entry_policy = response
        .policies
        .into_iter()
        .find(|policy| matches!(policy.specifier, RequestSpecifierDTO::AddAddressBookEntry))
        .expect("Failed to find policy");

    // add named rule that requires the other user to approve
    let named_rule1 = add_named_rule_with_input(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        AddNamedRuleOperationInput {
            name: "test".to_string(),
            description: Some("test".to_string()),
            rule: RequestPolicyRuleDTO::Quorum(QuorumDTO {
                approvers: station_api::UserSpecifierDTO::Id(vec![other_user.id]),
                min_approved: 1,
            }),
        },
    );

    // add another named rule that proxies the first one
    let named_rule2 = add_named_rule_with_input(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        AddNamedRuleOperationInput {
            name: "proxy".to_string(),
            description: Some("test".to_string()),
            rule: RequestPolicyRuleDTO::NamedRule(named_rule1.id),
        },
    );

    // edit the request policy to use the proxy named rule
    edit_request_policy(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        EditRequestPolicyOperationInput {
            policy_id: add_address_book_entry_policy.id,
            specifier: Some(RequestSpecifierDTO::AddAddressBookEntry),
            rule: Some(RequestPolicyRuleDTO::NamedRule(named_rule2.id)),
        },
    )
    .expect("Failed to edit request policy");

    let request = submit_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        RequestOperationInput::AddAddressBookEntry(AddAddressBookEntryOperationInput {
            address: "f2ca1bb6c7e907d06dafe4687e579fce76b37e4e93b7605022da52e6ccc26fd2".to_string(),
            address_owner: "test".to_string(),
            address_format: "icp_account_identifier".to_string(),
            blockchain: "icp".to_string(),
            metadata: vec![],
            labels: vec![],
        }),
    );

    // other user needs to approve the request
    assert!(matches!(request.status, RequestStatusDTO::Created));

    submit_request_approval(
        &env,
        other_user.identities[0],
        canister_ids.station,
        request.clone(),
        RequestApprovalStatusDTO::Approved,
    );

    env.advance_time(Duration::from_secs(5));
    env.tick();

    // other user has approved the request and it should be completed
    let request = get_request(&env, WALLET_ADMIN_USER, canister_ids.station, request);
    assert!(matches!(request.status, RequestStatusDTO::Completed { .. }));
}
