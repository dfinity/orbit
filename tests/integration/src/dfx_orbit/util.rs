use crate::{setup::WALLET_ADMIN_USER, utils::execute_request, CanisterIds};

use super::PORT;
use candid::Principal;
use pocket_ic::PocketIc;
use station_api::{
    AddRequestPolicyOperationInput, AuthScopeDTO, CallExternalCanisterResourceTargetDTO,
    EditPermissionOperationInput, ExecutionMethodResourceTargetDTO,
    ExternalCanisterResourceActionDTO, QuorumDTO, RequestOperationInput, RequestPolicyRuleDTO,
    RequestResourceActionDTO, RequestSpecifierDTO, ResourceDTO, UserSpecifierDTO,
    ValidationMethodResourceTargetDTO,
};

/// Fetches an asset from the local host and port
///
/// This is a bit tricky, as the boundary node uses the `Referer` header to determine the
/// resource being fetched.
pub(super) async fn fetch_asset(canister_id: Principal, path: &str) -> Vec<u8> {
    let port = PORT.with(|port| *port.borrow());
    let local_url = format!("http://localhost:{}{}", port, path);
    let referer = format!("http://localhost:{}?canisterId={}", port, canister_id);

    reqwest::Client::new()
        .get(local_url)
        .header("Referer", referer)
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap()
        .into()
}

/// Allow anyone to create change canister requests
pub(super) fn permit_call_operation(env: &PocketIc, canister_ids: &CanisterIds) {
    let add_permission = RequestOperationInput::EditPermission(EditPermissionOperationInput {
        resource: ResourceDTO::ExternalCanister(ExternalCanisterResourceActionDTO::Call(
            CallExternalCanisterResourceTargetDTO {
                validation_method: ValidationMethodResourceTargetDTO::No,
                execution_method: ExecutionMethodResourceTargetDTO::Any,
            },
        )),
        auth_scope: Some(AuthScopeDTO::Authenticated),
        user_groups: None,
        users: None,
    });
    execute_request(env, WALLET_ADMIN_USER, canister_ids.station, add_permission).unwrap();
}

/// Set four eyes principle for canister calls
pub(super) fn set_four_eyes_on_call(env: &PocketIc, canister_ids: &CanisterIds) {
    let add_request_policy =
        RequestOperationInput::AddRequestPolicy(AddRequestPolicyOperationInput {
            specifier: RequestSpecifierDTO::CallExternalCanister(
                CallExternalCanisterResourceTargetDTO {
                    validation_method: ValidationMethodResourceTargetDTO::No,
                    execution_method: ExecutionMethodResourceTargetDTO::Any,
                },
            ),
            rule: RequestPolicyRuleDTO::Quorum(QuorumDTO {
                approvers: UserSpecifierDTO::Any,
                min_approved: 2,
            }),
        });
    execute_request(
        env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        add_request_policy,
    )
    .unwrap();
}

/// Allow anyone to read request list
pub(super) fn permit_list_reads(env: &PocketIc, canister_ids: &CanisterIds) {
    let add_permission = RequestOperationInput::EditPermission(EditPermissionOperationInput {
        resource: ResourceDTO::Request(RequestResourceActionDTO::List),
        auth_scope: Some(AuthScopeDTO::Authenticated),
        user_groups: None,
        users: None,
    });
    execute_request(env, WALLET_ADMIN_USER, canister_ids.station, add_permission).unwrap();
}
