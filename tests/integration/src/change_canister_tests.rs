use crate::setup::{create_canister, setup_new_env, WALLET_ADMIN_USER};
use crate::utils::{
    add_user, canister_status, execute_request, submit_request, submit_request_approval,
    update_raw, user_test_id, wait_for_request, COUNTER_WAT,
};
use crate::TestEnv;
use candid::Principal;
use orbit_essentials::cdk::api::management_canister::main::CanisterInstallMode;
use sha2::{Digest, Sha256};
use station_api::{
    AddRequestPolicyOperationInput, ChangeCanisterOperationInput, ChangeCanisterTargetDTO,
    EditPermissionOperationInput, InstallCanisterInputDTO, QuorumDTO, RequestApprovalStatusDTO,
    RequestOperationInput, RequestPolicyRuleDTO, RequestSpecifierDTO, UserSpecifierDTO,
};

#[test]
fn successful_four_eyes_upgrade() {
    let TestEnv {
        mut env,
        canister_ids,
        ..
    } = setup_new_env();

    // set four eyes principle for canister changes
    let add_request_policy =
        RequestOperationInput::AddRequestPolicy(AddRequestPolicyOperationInput {
            specifier: RequestSpecifierDTO::ChangeCanister,
            rule: RequestPolicyRuleDTO::Quorum(QuorumDTO {
                approvers: UserSpecifierDTO::Any,
                min_approved: 2,
            }),
        });
    execute_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        add_request_policy,
    )
    .unwrap();

    // allow anyone to create change canister requests
    let add_permission = RequestOperationInput::EditPermission(EditPermissionOperationInput {
        resource: station_api::ResourceDTO::ChangeCanister(
            station_api::ChangeCanisterResourceActionDTO::Create,
        ),
        auth_scope: Some(station_api::AuthScopeDTO::Authenticated),
        user_groups: None,
        users: None,
    });
    execute_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        add_permission,
    )
    .unwrap();

    // create new user identities and add them to the station
    let user_a = user_test_id(0);
    add_user(&env, user_a, vec![], canister_ids.station);
    let user_b = user_test_id(1);
    add_user(&env, user_b, vec![], canister_ids.station);

    // create and install the canister to be upgraded by a request
    let canister_id = create_canister(&mut env, canister_ids.station);
    let module_bytes = vec![0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
    let module_hash =
        hex::decode("93a44bbb96c751218e4c00d479e4c14358122a389acca16205b1e4d0dc5f9476").unwrap();
    env.install_canister(
        canister_id,
        module_bytes.clone(),
        vec![],
        Some(canister_ids.station),
    );

    // check canister status and ensure that the WASM matches the old canister module
    let status = canister_status(&env, Some(canister_ids.station), canister_id);
    assert_eq!(status.module_hash, Some(module_hash.clone()));

    // new canister WASM
    let new_module_bytes = hex::decode("0061736d010000000503010001").unwrap();
    let new_module_hash =
        hex::decode("d7f602df8d1cb581cc5c886a4ff8809793c50627e305ef45f6d770f27e0261cc").unwrap();

    // submit canister upgrade request
    let install_canister_input = InstallCanisterInputDTO {
        mode: CanisterInstallMode::Upgrade(None),
        canister_id,
    };
    let change_canister_operation =
        RequestOperationInput::ChangeCanister(ChangeCanisterOperationInput {
            target: ChangeCanisterTargetDTO::InstallCanister(install_canister_input),
            module: new_module_bytes,
            arg: None,
        });
    let change_canister_operation_request = submit_request(
        &env,
        user_a,
        canister_ids.station,
        change_canister_operation,
    );

    // the request should not be completed before the second user approves on it
    assert!(wait_for_request(
        &env,
        user_a,
        canister_ids.station,
        change_canister_operation_request.clone(),
    )
    .is_err());

    // the second user approves and then the request will eventually become completed
    submit_request_approval(
        &env,
        user_b,
        canister_ids.station,
        change_canister_operation_request.clone(),
        RequestApprovalStatusDTO::Approved,
    );
    wait_for_request(
        &env,
        user_a,
        canister_ids.station,
        change_canister_operation_request.clone(),
    )
    .unwrap();

    // check canister status and ensure that the WASM matches the new canister module
    let status = canister_status(&env, Some(canister_ids.station), canister_id);
    assert_eq!(status.module_hash, Some(new_module_hash));
}

#[test]
fn reinstall_canister() {
    let TestEnv {
        mut env,
        canister_ids,
        ..
    } = setup_new_env();

    // create and install the canister to be reinstalled by a request
    let canister_id = create_canister(&mut env, canister_ids.station);
    let module_bytes = wat::parse_str(COUNTER_WAT).unwrap();
    let mut sha256 = Sha256::new();
    sha256.update(module_bytes.clone());
    let module_hash = sha256.finalize().to_vec();
    env.install_canister(
        canister_id,
        module_bytes.clone(),
        vec![],
        Some(canister_ids.station),
    );

    // check canister status and ensure that the WASM matches the counter canister module
    let status = canister_status(&env, Some(canister_ids.station), canister_id);
    assert_eq!(status.module_hash, Some(module_hash.clone()));

    // initialize stable memory and increment the counter in stable memory
    update_raw(&env, canister_id, Principal::anonymous(), "init", vec![]).unwrap();
    update_raw(&env, canister_id, Principal::anonymous(), "inc", vec![]).unwrap();

    // reading the counter should yield 2 after the increment
    let ctr = update_raw(&env, canister_id, Principal::anonymous(), "read", vec![]).unwrap();
    assert_eq!(ctr, 2_u32.to_le_bytes());

    // submit canister upgrade request
    let install_canister_input = InstallCanisterInputDTO {
        mode: CanisterInstallMode::Upgrade(None),
        canister_id,
    };
    let change_canister_operation =
        RequestOperationInput::ChangeCanister(ChangeCanisterOperationInput {
            target: ChangeCanisterTargetDTO::InstallCanister(install_canister_input),
            module: module_bytes.clone(),
            arg: None,
        });
    execute_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        change_canister_operation,
    )
    .unwrap();

    // check canister status and ensure that the WASM matches the counter canister module
    let status = canister_status(&env, Some(canister_ids.station), canister_id);
    assert_eq!(status.module_hash, Some(module_hash.clone()));

    // the counter value should be preserved across upgrade
    let ctr = update_raw(&env, canister_id, Principal::anonymous(), "read", vec![]).unwrap();
    assert_eq!(ctr, 2_u32.to_le_bytes());

    // submit canister reinstall request
    let install_canister_input = InstallCanisterInputDTO {
        mode: CanisterInstallMode::Reinstall,
        canister_id,
    };
    let change_canister_operation =
        RequestOperationInput::ChangeCanister(ChangeCanisterOperationInput {
            target: ChangeCanisterTargetDTO::InstallCanister(install_canister_input),
            module: module_bytes,
            arg: None,
        });
    execute_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        change_canister_operation,
    )
    .unwrap();

    // check canister status and ensure that the WASM matches the counter canister module
    let status = canister_status(&env, Some(canister_ids.station), canister_id);
    assert_eq!(status.module_hash, Some(module_hash));

    // stable memory should be reset now and thus "read" will trap
    let err = update_raw(&env, canister_id, Principal::anonymous(), "read", vec![]).unwrap_err();
    assert!(err
        .description
        .contains("Canister trapped: stable memory out of bounds"));
}
