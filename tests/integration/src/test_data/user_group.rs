use super::next_unique_id;
use crate::utils::{submit_request, wait_for_request};
use candid::Principal;
use pocket_ic::PocketIc;

pub fn add_user_group(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
) -> station_api::UserGroupDTO {
    let add_user_group_request = submit_request(
        env,
        requester,
        station_canister_id,
        station_api::RequestOperationInput::AddUserGroup(station_api::AddUserGroupOperationInput {
            name: format!("group-{}", next_unique_id()),
        }),
    );

    let result = wait_for_request(env, requester, station_canister_id, add_user_group_request)
        .expect("Failed to add user group");

    match result.operation {
        station_api::RequestOperationDTO::AddUserGroup(add_user_group) => add_user_group
            .user_group
            .expect("Unexpected missing user group"),
        _ => panic!("unexpected request operation"),
    }
}

pub fn edit_user_group(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
    user_group_id: station_api::UuidDTO,
    name: String,
) {
    let edit_user_group_request = submit_request(
        env,
        requester,
        station_canister_id,
        station_api::RequestOperationInput::EditUserGroup(
            station_api::EditUserGroupOperationInput {
                user_group_id,
                name,
            },
        ),
    );

    wait_for_request(env, requester, station_canister_id, edit_user_group_request)
        .expect("Failed to edit user group");
}
