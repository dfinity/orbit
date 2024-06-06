use crate::setup::{create_canister, setup_new_env, WALLET_ADMIN_USER};
use crate::utils::{
    add_user, canister_status, execute_request, get_request, submit_request,
    submit_request_approval, submit_request_with_expected_trap, update_raw, user_test_id,
    wait_for_request, COUNTER_WAT,
};
use crate::TestEnv;
use candid::{Encode, Principal};
use ic_cdk::api::management_canister::main::{CanisterIdRecord, CanisterStatusResponse};
use orbit_essentials::api::ApiResult;
use pocket_ic::update_candid_as;
use sha2::{Digest, Sha256};
use station_api::{
    AddRequestPolicyOperationInput, CanisterInstallMode, ChangeManagedCanisterOperationInput,
    ChangeManagedCanisterResourceTargetDTO, CreateManagedCanisterOperationInput,
    CreateManagedCanisterResourceTargetDTO, EditPermissionOperationInput, ListRequestsInput,
    ListRequestsOperationTypeDTO, ListRequestsResponse, QuorumDTO,
    ReadManagedCanisterResourceTargetDTO, RequestApprovalStatusDTO, RequestOperationDTO,
    RequestOperationInput, RequestPolicyRuleDTO, RequestSpecifierDTO, RequestStatusDTO,
    UserSpecifierDTO,
};

#[test]
fn successful_four_eyes_upgrade() {
    let TestEnv {
        mut env,
        canister_ids,
        ..
    } = setup_new_env();

    // create and install the counter canister
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

    // the counter should initially be set at 0
    let ctr = update_raw(&env, canister_id, Principal::anonymous(), "read", vec![]).unwrap();
    assert_eq!(ctr, 0_u32.to_le_bytes());

    // create new user identities and add them to the station
    let user_a = user_test_id(0);
    add_user(&env, user_a, vec![], canister_ids.station);
    let user_b = user_test_id(1);
    add_user(&env, user_b, vec![], canister_ids.station);

    // submitting canister upgrade request fails due to insufficient permissions to create change canister requests
    let change_canister_operation =
        RequestOperationInput::ChangeManagedCanister(ChangeManagedCanisterOperationInput {
            canister_id,
            mode: CanisterInstallMode::Upgrade,
            module: module_bytes.clone(),
            arg: None,
        });
    let trap_message = submit_request_with_expected_trap(
        &env,
        user_a,
        canister_ids.station,
        change_canister_operation.clone(),
    );
    assert!(trap_message.contains(
        "Canister trapped explicitly: Unauthorized access to resources: ManagedCanister(Change"
    ));

    // allow anyone to create change canister requests
    let add_permission = RequestOperationInput::EditPermission(EditPermissionOperationInput {
        resource: station_api::ResourceDTO::ManagedCanister(
            station_api::ManagedCanisterResourceActionDTO::Change(
                ChangeManagedCanisterResourceTargetDTO::Canister(canister_id),
            ),
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

    // now the request to upgrade the counter canister can be successfully submitted
    let change_canister_operation_request = submit_request(
        &env,
        user_a,
        canister_ids.station,
        change_canister_operation.clone(),
    );

    // let the admin user reject the request => the request becomes rejected
    submit_request_approval(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        change_canister_operation_request.clone(),
        RequestApprovalStatusDTO::Rejected,
    );
    let rejected_request = get_request(
        &env,
        user_a,
        canister_ids.station,
        change_canister_operation_request,
    );
    match rejected_request.status {
        RequestStatusDTO::Rejected { .. } => (),
        _ => panic!("Request should have been rejected."),
    };

    // set four eyes principle for canister changes
    let add_request_policy =
        RequestOperationInput::AddRequestPolicy(AddRequestPolicyOperationInput {
            specifier: RequestSpecifierDTO::ChangeManagedCanister(
                ChangeManagedCanisterResourceTargetDTO::Canister(canister_id),
            ),
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

    // submit the request to upgrade the counter canister again
    let change_canister_operation_request = submit_request(
        &env,
        user_a,
        canister_ids.station,
        change_canister_operation,
    );

    // let the admin user reject the request => the request stays open as the second user can also approve it
    submit_request_approval(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        change_canister_operation_request.clone(),
        RequestApprovalStatusDTO::Rejected,
    );
    let created_request = get_request(
        &env,
        user_a,
        canister_ids.station,
        change_canister_operation_request.clone(),
    );
    match created_request.status {
        RequestStatusDTO::Created => (),
        _ => panic!("Request should be created."),
    };

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

    // check canister status and ensure that the WASM matches the counter canister module
    let status = canister_status(&env, Some(canister_ids.station), canister_id);
    assert_eq!(status.module_hash, Some(module_hash.clone()));

    // the counter value should be preserved across upgrade and incremented in post-upgrade hook
    let ctr = update_raw(&env, canister_id, Principal::anonymous(), "read", vec![]).unwrap();
    assert_eq!(ctr, 2_u32.to_le_bytes());
}

#[test]
fn upgrade_reinstall_list_test() {
    let TestEnv {
        mut env,
        canister_ids,
        ..
    } = setup_new_env();

    // create and install the counter canister
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

    // the counter should be initially be set at 0
    let ctr = update_raw(&env, canister_id, Principal::anonymous(), "read", vec![]).unwrap();
    assert_eq!(ctr, 0_u32.to_le_bytes());

    // increment the counter in stable memory
    update_raw(&env, canister_id, Principal::anonymous(), "inc", vec![]).unwrap();

    // reading the counter should yield 2 after the increment
    let ctr = update_raw(&env, canister_id, Principal::anonymous(), "read", vec![]).unwrap();
    assert_eq!(ctr, 2_u32.to_le_bytes());

    // submit canister upgrade request
    let change_canister_operation =
        RequestOperationInput::ChangeManagedCanister(ChangeManagedCanisterOperationInput {
            canister_id,
            mode: CanisterInstallMode::Upgrade,
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

    // the counter value should be preserved across upgrade and incremented in post-upgrade hook
    let ctr = update_raw(&env, canister_id, Principal::anonymous(), "read", vec![]).unwrap();
    assert_eq!(ctr, 4_u32.to_le_bytes());

    // submit canister reinstall request
    let change_canister_operation =
        RequestOperationInput::ChangeManagedCanister(ChangeManagedCanisterOperationInput {
            canister_id,
            mode: CanisterInstallMode::Reinstall,
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

    // stable memory should be reset across reinstall and thus the counter is back at 0
    let ctr = update_raw(&env, canister_id, Principal::anonymous(), "read", vec![]).unwrap();
    assert_eq!(ctr, 0_u32.to_le_bytes());

    // test listing requests for the specified canister ID
    let list_requests_operation_type =
        ListRequestsOperationTypeDTO::ChangeManagedCanister(Some(canister_id));
    let list_requests_input = ListRequestsInput {
        requester_ids: None,
        approver_ids: None,
        statuses: None,
        operation_types: Some(vec![list_requests_operation_type]),
        expiration_from_dt: None,
        expiration_to_dt: None,
        created_from_dt: None,
        created_to_dt: None,
        paginate: None,
        sort_by: None,
        only_approvable: false,
        with_evaluation_results: false,
    };
    let res: (ApiResult<ListRequestsResponse>,) = update_candid_as(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        "list_requests",
        (list_requests_input,),
    )
    .unwrap();
    assert_eq!(res.0.unwrap().total, 2);

    // test listing requests for a specified canister ID without any requests
    let list_requests_operation_type =
        ListRequestsOperationTypeDTO::ChangeManagedCanister(Some(Principal::management_canister()));
    let list_requests_input = ListRequestsInput {
        requester_ids: None,
        approver_ids: None,
        statuses: None,
        operation_types: Some(vec![list_requests_operation_type]),
        expiration_from_dt: None,
        expiration_to_dt: None,
        created_from_dt: None,
        created_to_dt: None,
        paginate: None,
        sort_by: None,
        only_approvable: false,
        with_evaluation_results: false,
    };
    let res: (ApiResult<ListRequestsResponse>,) = update_candid_as(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        "list_requests",
        (list_requests_input,),
    )
    .unwrap();
    assert_eq!(res.0.unwrap().total, 0);

    // test listing requests for no specified canister ID
    let list_requests_operation_type = ListRequestsOperationTypeDTO::ChangeManagedCanister(None);
    let list_requests_input = ListRequestsInput {
        requester_ids: None,
        approver_ids: None,
        statuses: None,
        operation_types: Some(vec![list_requests_operation_type]),
        expiration_from_dt: None,
        expiration_to_dt: None,
        created_from_dt: None,
        created_to_dt: None,
        paginate: None,
        sort_by: None,
        only_approvable: false,
        with_evaluation_results: false,
    };
    let res: (ApiResult<ListRequestsResponse>,) = update_candid_as(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        "list_requests",
        (list_requests_input,),
    )
    .unwrap();
    assert_eq!(res.0.unwrap().total, 2);
}

#[test]
fn create_managed_canister_and_check_status() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    // create new user identities and add them to the station
    let user_a = user_test_id(0);
    let user_a_dto = add_user(&env, user_a, vec![], canister_ids.station);
    let user_b = user_test_id(1);
    add_user(&env, user_b, vec![], canister_ids.station);

    // submitting request to create a managed canister fails due to insufficient permissions to create such requests
    let create_canister_operation =
        RequestOperationInput::CreateManagedCanister(CreateManagedCanisterOperationInput {});
    let trap_message = submit_request_with_expected_trap(
        &env,
        user_a,
        canister_ids.station,
        create_canister_operation.clone(),
    );
    assert!(trap_message.contains(
        "Canister trapped explicitly: Unauthorized access to resources: ManagedCanister(Create"
    ));

    // allow anyone to create requests to create a managed canister
    let add_permission = RequestOperationInput::EditPermission(EditPermissionOperationInput {
        resource: station_api::ResourceDTO::ManagedCanister(
            station_api::ManagedCanisterResourceActionDTO::Create(
                CreateManagedCanisterResourceTargetDTO::Any,
            ),
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

    // now the request to create a managed canister can be successfully submitted
    let create_canister_operation_request = submit_request(
        &env,
        user_a,
        canister_ids.station,
        create_canister_operation.clone(),
    );

    // let the admin user reject the request => the request becomes rejected
    submit_request_approval(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        create_canister_operation_request.clone(),
        RequestApprovalStatusDTO::Rejected,
    );
    let rejected_request = get_request(
        &env,
        user_a,
        canister_ids.station,
        create_canister_operation_request,
    );
    match rejected_request.status {
        RequestStatusDTO::Rejected { .. } => (),
        _ => panic!("Request should have been rejected."),
    };
    match rejected_request.operation {
        RequestOperationDTO::CreateManagedCanister(operation) => {
            assert!(operation.canister_id.is_none())
        }
        _ => panic!(
            "Unexpected request operation type: {:?}",
            rejected_request.operation
        ),
    };

    // set four eyes principle for creating managed canisters
    let add_request_policy =
        RequestOperationInput::AddRequestPolicy(AddRequestPolicyOperationInput {
            specifier: RequestSpecifierDTO::CreateManagedCanister(
                CreateManagedCanisterResourceTargetDTO::Any,
            ),
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

    // submit the request to create a managed canister again
    let create_canister_operation_request = submit_request(
        &env,
        user_a,
        canister_ids.station,
        create_canister_operation,
    );

    // let the admin user reject the request => the request stays open as the second user can also approve it
    submit_request_approval(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        create_canister_operation_request.clone(),
        RequestApprovalStatusDTO::Rejected,
    );
    let created_request = get_request(
        &env,
        user_a,
        canister_ids.station,
        create_canister_operation_request.clone(),
    );
    match created_request.status {
        RequestStatusDTO::Created => (),
        _ => panic!("Request should be created."),
    };
    match created_request.operation {
        RequestOperationDTO::CreateManagedCanister(operation) => {
            assert!(operation.canister_id.is_none())
        }
        _ => panic!(
            "Unexpected request operation type: {:?}",
            created_request.operation
        ),
    };

    // the second user approves and then the request will eventually become completed
    submit_request_approval(
        &env,
        user_b,
        canister_ids.station,
        create_canister_operation_request.clone(),
        RequestApprovalStatusDTO::Approved,
    );
    wait_for_request(
        &env,
        user_a,
        canister_ids.station,
        create_canister_operation_request.clone(),
    )
    .unwrap();

    let executed_request = get_request(
        &env,
        user_a,
        canister_ids.station,
        create_canister_operation_request.clone(),
    );
    match executed_request.status {
        RequestStatusDTO::Completed { .. } => (),
        _ => panic!("Request should be completed."),
    };
    let canister_id = match executed_request.operation {
        RequestOperationDTO::CreateManagedCanister(operation) => operation.canister_id.unwrap(),
        _ => panic!(
            "Unexpected request operation type: {:?}",
            executed_request.operation
        ),
    };

    // top up canister
    assert_eq!(env.cycle_balance(canister_id), 0);
    env.add_cycles(canister_id, 100_000_000_000_000);

    // check canister status on behalf of the station and ensure that the canister is empty
    let status = canister_status(&env, Some(canister_ids.station), canister_id);
    assert_eq!(status.module_hash, None);

    // checking canister status on behalf of the users fails due to insufficient permissions
    let canister_id_record = CanisterIdRecord { canister_id };
    let trap_message = update_raw(
        &env,
        canister_ids.station,
        user_a,
        "canister_status",
        Encode!(&canister_id_record).unwrap(),
    )
    .unwrap_err();
    assert!(trap_message.description.contains(
        "Canister trapped explicitly: Unauthorized access to resources: ManagedCanister(Read"
    ));
    let trap_message = update_raw(
        &env,
        canister_ids.station,
        user_b,
        "canister_status",
        Encode!(&canister_id_record).unwrap(),
    )
    .unwrap_err();
    assert!(trap_message.description.contains(
        "Canister trapped explicitly: Unauthorized access to resources: ManagedCanister(Read"
    ));

    // allow the first user to read the canister status of the managed canister created above
    let add_permission = RequestOperationInput::EditPermission(EditPermissionOperationInput {
        resource: station_api::ResourceDTO::ManagedCanister(
            station_api::ManagedCanisterResourceActionDTO::Read(
                ReadManagedCanisterResourceTargetDTO::Canister(canister_id),
            ),
        ),
        auth_scope: None,
        user_groups: None,
        users: Some(vec![user_a_dto.id.to_string()]),
    });
    execute_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        add_permission,
    )
    .unwrap();

    // checking canister status on behalf of the first user now succeeds
    let canister_id_record = CanisterIdRecord { canister_id };
    let status: (ApiResult<CanisterStatusResponse>,) = update_candid_as(
        &env,
        canister_ids.station,
        user_a,
        "canister_status",
        (canister_id_record,),
    )
    .unwrap();
    assert_eq!(status.0.unwrap().module_hash, None);
    let trap_message = update_raw(
        &env,
        canister_ids.station,
        user_b,
        "canister_status",
        Encode!(&canister_id_record).unwrap(),
    )
    .unwrap_err();
    assert!(trap_message.description.contains(
        "Canister trapped explicitly: Unauthorized access to resources: ManagedCanister(Read"
    ));
}
