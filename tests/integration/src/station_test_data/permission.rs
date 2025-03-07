use crate::utils::{submit_request, wait_for_request};
use candid::Principal;
use orbit_essentials::api::ApiResult;
use pocket_ic::PocketIc;
use pocket_ic::{query_candid_as, RejectResponse};
use station_api::{ListPermissionsInput, ListPermissionsResponse};

pub fn edit_permission(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
    resource: station_api::ResourceDTO,
) {
    let edit_account_request = submit_request(
        env,
        requester,
        station_canister_id,
        station_api::RequestOperationInput::EditPermission(
            station_api::EditPermissionOperationInput {
                resource,
                auth_scope: Some(station_api::AuthScopeDTO::Authenticated),
                user_groups: None,
                users: None,
            },
        ),
    );

    wait_for_request(env, requester, station_canister_id, edit_account_request)
        .expect("Failed to edit permission");
}

pub fn list_permissions(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
) -> Result<(ApiResult<ListPermissionsResponse>,), RejectResponse> {
    query_candid_as::<(ListPermissionsInput,), (ApiResult<ListPermissionsResponse>,)>(
        env,
        station_canister_id,
        requester,
        "list_permissions",
        (ListPermissionsInput {
            resources: None,
            paginate: None,
        },),
    )
}
