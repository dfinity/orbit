use candid::Principal;
use pocket_ic::PocketIc;
use station_api::{
    AuthScopeDTO, CallExternalCanisterOperationInput, CanisterMethodDTO,
    EditPermissionOperationInput, ListRequestsInput, RequestOperationInput,
    RequestResourceActionDTO, ResourceDTO,
};

use crate::{
    cli::{
        canister_call::{permit_call_operation, set_four_eyes_on_call},
        dfx_orbit_test, setup_agent, setup_counter_canister, TEST_PRINCIPAL,
    },
    setup::{setup_new_env, WALLET_ADMIN_USER},
    utils::{add_user, add_user_with_name, execute_request, submit_request, user_test_id},
    CanisterIds, TestEnv,
};

#[test]
fn review() {
    let TestEnv {
        mut env,
        canister_ids,
        ..
    } = setup_new_env();

    let canister_id = setup_counter_canister(&mut env, &canister_ids);

    // create new user identities and add them to the station
    let dfx_user = Principal::from_text(TEST_PRINCIPAL).unwrap();
    add_user_with_name(
        &env,
        String::from("dfx_user"),
        dfx_user,
        vec![],
        canister_ids.station,
    );
    let other_user = user_test_id(1);
    add_user(&env, other_user, vec![], canister_ids.station);

    permit_list_reads(&env, &canister_ids);
    permit_call_operation(&env, &canister_ids);
    set_four_eyes_on_call(&env, &canister_ids);

    // The other user submits a canister call request
    let request_counter_canister_inc =
        RequestOperationInput::CallExternalCanister(CallExternalCanisterOperationInput {
            validation_method: None,
            execution_method: CanisterMethodDTO {
                canister_id,
                method_name: String::from("set"),
            },
            arg: Some(42_u32.to_le_bytes().to_vec()),
            execution_method_cycles: None,
        });

    let submitted_request = submit_request(
        &env,
        other_user,
        canister_ids.station,
        request_counter_canister_inc,
    );

    dfx_orbit_test(&mut env, async {
        // Setup the station agent
        let mut station_agent = setup_agent(canister_ids.station).await;

        let list_request_response = station_agent
            .review_list(ListRequestsInput {
                requester_ids: None,
                approver_ids: None,
                statuses: None,
                operation_types: None,
                expiration_from_dt: None,
                expiration_to_dt: None,
                created_from_dt: None,
                created_to_dt: None,
                paginate: None,
                sort_by: None,
                only_approvable: true,
                with_evaluation_results: false,
            })
            .await
            .unwrap();

        dbg!(&list_request_response);

        // Check that there is only one entry in the response and that its the one that was submitted
        assert_eq!(list_request_response.requests.len(), 1);
        assert_eq!(list_request_response.requests[0].id, submitted_request.id);
    });

    todo!()
}

/// Allow anyone to read request list
pub(crate) fn permit_list_reads(env: &PocketIc, canister_ids: &CanisterIds) {
    let add_permission = RequestOperationInput::EditPermission(EditPermissionOperationInput {
        resource: ResourceDTO::Request(RequestResourceActionDTO::List),
        auth_scope: Some(AuthScopeDTO::Authenticated),
        user_groups: None,
        users: None,
    });
    execute_request(env, WALLET_ADMIN_USER, canister_ids.station, add_permission).unwrap();
}
