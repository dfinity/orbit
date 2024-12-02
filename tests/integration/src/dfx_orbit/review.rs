use candid::Principal;

use station_api::{
    CallExternalCanisterOperationInput, CanisterMethodDTO, GetNextApprovableRequestInput,
    GetRequestInput, ListRequestsInput, ListRequestsOperationTypeDTO, RequestOperationInput,
};

use crate::{
    dfx_orbit::{
        setup::{
            dfx_orbit_test, setup_counter_canister, setup_dfx_orbit, DfxOrbitTestConfig,
            TEST_PRINCIPAL,
        },
        util::{permit_call_operation, permit_list_reads, set_four_eyes_on_call},
    },
    setup::setup_new_env,
    utils::{
        add_user, add_user_with_name, submit_request, update_raw, user_test_id, wait_for_request,
    },
    TestEnv,
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
    let request_counter_canister_set =
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
        request_counter_canister_set,
    );

    let submitted_request_clone = submitted_request.clone();

    // Check that the counter has not updated yet
    let ctr = update_raw(&env, canister_id, Principal::anonymous(), "read", vec![]).unwrap();
    assert_eq!(ctr, 0_u32.to_le_bytes());

    dfx_orbit_test(&mut env, DfxOrbitTestConfig::default(), async {
        // Setup the station agent
        let dfx_orbit = setup_dfx_orbit(canister_ids.station).await;

        let list_request_response = dfx_orbit
            .station
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

        // Check that there is only one entry in the response and that its the one that was submitted
        assert_eq!(list_request_response.requests.len(), 1);
        assert_eq!(list_request_response.requests[0].id, submitted_request.id);

        // Check that this id also matches the values returned from review_next and review_id
        let next_request = dfx_orbit
            .station
            .review_next(GetNextApprovableRequestInput {
                excluded_request_ids: vec![],
                operation_types: Some(vec![ListRequestsOperationTypeDTO::CallExternalCanister(
                    Some(canister_id),
                )]),
                sort_by: None,
            })
            .await
            .unwrap()
            .unwrap();

        let id_request = dfx_orbit
            .station
            .review_id(GetRequestInput {
                request_id: submitted_request.id.clone(),
                with_full_info: Some(false),
            })
            .await
            .unwrap();

        assert_eq!(next_request.request.id, id_request.request.id);

        // Approve the request
        dfx_orbit
            .station
            .approve(submitted_request.id, None)
            .await
            .unwrap();
    });

    wait_for_request(
        &env,
        other_user,
        canister_ids.station,
        submitted_request_clone,
    )
    .unwrap();

    let ctr = update_raw(&env, canister_id, Principal::anonymous(), "read", vec![]).unwrap();
    assert_eq!(ctr, 42_u32.to_le_bytes());
}
