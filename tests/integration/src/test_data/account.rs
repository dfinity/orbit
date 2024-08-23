use super::next_unique_id;
use crate::utils::{submit_request, wait_for_request};
use candid::Principal;
use pocket_ic::PocketIc;

pub fn add_account(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
) -> station_api::AccountDTO {
    let next_id = next_unique_id();
    let add_account_request = submit_request(
        env,
        requester,
        station_canister_id,
        station_api::RequestOperationInput::AddAccount(station_api::AddAccountOperationInput {
            name: format!("account-{}", next_id),
            blockchain: "icp".to_string(),
            standard: "native".to_string(),
            metadata: Vec::new(),
            configs_permission: station_api::AllowDTO {
                auth_scope: station_api::AuthScopeDTO::Authenticated,
                user_groups: Vec::new(),
                users: vec![],
            },
            read_permission: station_api::AllowDTO {
                auth_scope: station_api::AuthScopeDTO::Authenticated,
                user_groups: Vec::new(),
                users: vec![],
            },
            transfer_permission: station_api::AllowDTO {
                auth_scope: station_api::AuthScopeDTO::Authenticated,
                user_groups: Vec::new(),
                users: vec![],
            },
            configs_request_policy: Some(station_api::RequestPolicyRuleDTO::AutoApproved),
            transfer_request_policy: Some(station_api::RequestPolicyRuleDTO::QuorumPercentage(
                station_api::QuorumPercentageDTO {
                    min_approved: 1,
                    approvers: station_api::UserSpecifierDTO::Any,
                },
            )),
        }),
    );

    let request = wait_for_request(env, requester, station_canister_id, add_account_request)
        .expect("Failed to add account");

    match request.operation {
        station_api::RequestOperationDTO::AddAccount(add_account) => add_account.account.unwrap(),
        _ => panic!("invalid request operation"),
    }
}

pub fn edit_account_name(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
    account_id: station_api::UuidDTO,
    name: String,
) {
    let edit_account_request = submit_request(
        env,
        requester,
        station_canister_id,
        station_api::RequestOperationInput::EditAccount(station_api::EditAccountOperationInput {
            account_id,
            name: Some(name),
            configs_permission: None,
            read_permission: None,
            transfer_permission: None,
            configs_request_policy: None,
            transfer_request_policy: None,
        }),
    );

    wait_for_request(env, requester, station_canister_id, edit_account_request)
        .expect("Failed to edit account");
}
