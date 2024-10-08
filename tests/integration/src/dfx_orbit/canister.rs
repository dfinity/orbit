use crate::{
    dfx_orbit::{
        setup::{
            dfx_orbit_test, setup_counter_canister, setup_dfx_orbit, setup_dfx_user,
            DfxOrbitTestConfig,
        },
        util::{
            permit_call_operation, permit_change_operation, set_four_eyes_on_call,
            set_four_eyes_on_change,
        },
    },
    setup::setup_new_env,
    utils::{
        add_user, submit_request_approval, update_raw, user_test_id, wait_for_request, COUNTER_WAT,
    },
    TestEnv,
};
use candid::Principal;
use dfx_orbit::{
    args::{RequestArgs, RequestArgsActions, VerifyArgs, VerifyArgsAction},
    canister::{
        CanisterInstallModeArgs, RequestCanisterActionArgs, RequestCanisterArgs,
        RequestCanisterCallArgs, RequestCanisterInstallArgs, VerifyCanisterActionArgs,
        VerifyCanisterArgs,
    },
};
use station_api::{GetRequestInput, RequestApprovalStatusDTO};
use tempfile::env::temp_dir;

/// Test a canister call through orbit using the station agent
#[test]
fn canister_call() {
    let TestEnv {
        mut env,
        canister_ids,
        ..
    } = setup_new_env();

    let canister_id = setup_counter_canister(&mut env, &canister_ids);

    setup_dfx_user(&env, &canister_ids);
    let other_user = user_test_id(1);
    add_user(&env, other_user, vec![], canister_ids.station);

    permit_call_operation(&env, &canister_ids);
    set_four_eyes_on_call(&env, &canister_ids);

    let config = DfxOrbitTestConfig {
        canister_ids: vec![(String::from("counter"), canister_id)],
        ..Default::default()
    };

    let inner_args = RequestCanisterCallArgs {
        canister: String::from("counter"),
        method_name: String::from("set"),
        argument: None,
        arg_file: None,
        raw_arg: Some(String::from("2a000000")),
        with_cycles: None,
    };

    let request = dfx_orbit_test(&mut env, config, async {
        // Setup the station agent
        let dfx_orbit = setup_dfx_orbit(canister_ids.station).await;

        // Call the counter canister
        let request = RequestArgs {
            title: None,
            summary: None,
            action: RequestArgsActions::Canister(RequestCanisterArgs {
                action: RequestCanisterActionArgs::Call(inner_args.clone()),
            }),
        }
        .into_request(&dfx_orbit)
        .await
        .unwrap();

        let request = dfx_orbit.station.request(request.clone()).await.unwrap();

        // Check that the request verifies
        let req_response = dfx_orbit
            .station
            .review_id(GetRequestInput {
                request_id: request.request.id.clone(),
            })
            .await
            .unwrap();

        VerifyArgs {
            request_id: request.request.id.clone(),
            and_approve: false,
            or_reject: false,
            action: VerifyArgsAction::Canister(VerifyCanisterArgs {
                action: VerifyCanisterActionArgs::Call(inner_args),
            }),
        }
        .verify(&dfx_orbit, &req_response)
        .await
        .unwrap();

        request.request
    });

    // Check that the counter has not updated yet
    let ctr = update_raw(&env, canister_id, Principal::anonymous(), "read", vec![]).unwrap();
    assert_eq!(ctr, 0_u32.to_le_bytes());

    // The other user approves the request
    submit_request_approval(
        &env,
        other_user,
        canister_ids.station,
        request.clone(),
        RequestApprovalStatusDTO::Approved,
    );
    wait_for_request(&env, other_user, canister_ids.station, request).unwrap();

    let ctr = update_raw(&env, canister_id, Principal::anonymous(), "read", vec![]).unwrap();
    assert_eq!(ctr, 42_u32.to_le_bytes());
}

#[test]
fn canister_install() {
    let TestEnv {
        mut env,
        canister_ids,
        ..
    } = setup_new_env();

    let canister_id = setup_counter_canister(&mut env, &canister_ids);

    setup_dfx_user(&env, &canister_ids);
    let other_user = user_test_id(1);
    add_user(&env, other_user, vec![], canister_ids.station);

    permit_change_operation(&env, &canister_ids);
    set_four_eyes_on_change(&env, &canister_ids);

    // the counter value should be preserved across upgrade and incremented in post-upgrade hook
    update_raw(&env, canister_id, canister_ids.station, "inc", vec![]).unwrap();
    let ctr = update_raw(&env, canister_id, Principal::anonymous(), "read", vec![]).unwrap();
    assert_eq!(ctr, 2_u32.to_le_bytes());

    let wasm_dir = temp_dir();
    let wasm_path = wasm_dir.as_path().join("counter.wasm");
    let counter_wasm = wat::parse_str(COUNTER_WAT).unwrap();
    std::fs::write(&wasm_path, counter_wasm).unwrap();

    let config = DfxOrbitTestConfig {
        canister_ids: vec![(String::from("counter"), canister_id)],
        ..Default::default()
    };

    let inner_args = RequestCanisterInstallArgs {
        canister: String::from("counter"),
        mode: CanisterInstallModeArgs::Upgrade,
        wasm: wasm_path.into_os_string().into_string().unwrap(),
        argument: None,
        arg_file: None,
    };

    let request = dfx_orbit_test(&mut env, config, async {
        // Setup the station agent
        let dfx_orbit = setup_dfx_orbit(canister_ids.station).await;

        // Update the counter canister
        let request = RequestArgs {
            title: None,
            summary: None,
            action: RequestArgsActions::Canister(RequestCanisterArgs {
                action: RequestCanisterActionArgs::Install(inner_args.clone()),
            }),
        }
        .into_request(&dfx_orbit)
        .await
        .unwrap();
        let request = dfx_orbit.station.request(request.clone()).await.unwrap();

        // Check that the request verifies
        let req_response = dfx_orbit
            .station
            .review_id(GetRequestInput {
                request_id: request.request.id.clone(),
            })
            .await
            .unwrap();

        VerifyArgs {
            request_id: request.request.id.clone(),
            and_approve: false,
            or_reject: false,
            action: VerifyArgsAction::Canister(VerifyCanisterArgs {
                action: VerifyCanisterActionArgs::Install(inner_args),
            }),
        }
        .verify(&dfx_orbit, &req_response)
        .await
        .unwrap();

        request.request
    });

    // Check that the upgrade has not happened yet
    let ctr = update_raw(&env, canister_id, Principal::anonymous(), "read", vec![]).unwrap();
    assert_eq!(ctr, 2_u32.to_le_bytes());

    // The other user approves the request
    submit_request_approval(
        &env,
        other_user,
        canister_ids.station,
        request.clone(),
        RequestApprovalStatusDTO::Approved,
    );
    wait_for_request(&env, other_user, canister_ids.station, request).unwrap();

    // Check that the upgrade happened
    let ctr = update_raw(&env, canister_id, Principal::anonymous(), "read", vec![]).unwrap();
    assert_eq!(ctr, 4_u32.to_le_bytes());
}
