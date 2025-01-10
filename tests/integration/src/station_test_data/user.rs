use super::next_unique_id;
use crate::utils::{submit_request, wait_for_request};
use candid::Principal;
use pocket_ic::PocketIc;

pub fn add_user(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
    group_ids: Vec<String>,
) -> station_api::UserDTO {
    let next_id = next_unique_id();
    let user_name = format!("user-{}", next_id);
    let next_id = next_id.to_be_bytes();
    let identity = Principal::from_slice(next_id.as_ref());
    let add_user =
        station_api::RequestOperationInput::AddUser(station_api::AddUserOperationInput {
            name: user_name,
            identities: vec![identity],
            groups: group_ids,
            status: station_api::UserStatusDTO::Active,
        });
    let add_user_request = submit_request(env, requester, station_canister_id, add_user);
    let new_request = wait_for_request(env, requester, station_canister_id, add_user_request)
        .expect("Failed to add user");

    match new_request.operation {
        station_api::RequestOperationDTO::AddUser(add_user) => add_user.user.unwrap(),
        _ => panic!("invalid request operation"),
    }
}

pub fn edit_user_name(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
    user_id: station_api::UuidDTO,
    name: String,
) {
    let edit_user =
        station_api::RequestOperationInput::EditUser(station_api::EditUserOperationInput {
            id: user_id,
            name: Some(name),
            identities: None,
            groups: None,
            status: None,
            cancel_pending_requests: None,
        });

    let edit_user_request = submit_request(env, requester, station_canister_id, edit_user);
    wait_for_request(env, requester, station_canister_id, edit_user_request)
        .expect("Failed to edit user");
}
