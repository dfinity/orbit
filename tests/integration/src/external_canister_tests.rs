use crate::setup::{create_canister, setup_new_env, WALLET_ADMIN_USER};
use crate::utils::{
    add_user, bump_time_to_avoid_ratelimit, canister_status, deploy_test_canister, execute_request,
    get_core_canister_health_status, get_request, get_system_info, hash, submit_request,
    submit_request_approval, submit_request_raw, submit_request_with_expected_trap, update_raw,
    upload_canister_chunks_to_asset_canister, user_test_id, wait_for_request, COUNTER_WAT,
};
use crate::TestEnv;
use candid::{Encode, Principal};
use ic_cdk::api::management_canister::main::CanisterStatusResponse;
use ic_management_canister_types::{CanisterIdRecord, CanisterStatusType};
use orbit_essentials::api::ApiResult;
use orbit_essentials::cmc::{SubnetFilter, SubnetSelection};
use orbit_essentials::utils::timestamp_to_rfc3339;
use pocket_ic::update_candid_as;
use sha2::{Digest, Sha256};
use station_api::{
    AddRequestPolicyOperationInput, AllowDTO, CallExternalCanisterOperationInput,
    CallExternalCanisterResourceTargetDTO, CanisterInstallMode, CanisterMethodDTO,
    ChangeExternalCanisterOperationInput, CreateExternalCanisterOperationInput,
    CreateExternalCanisterOperationKindCreateNewDTO, CreateExternalCanisterOperationKindDTO,
    EditPermissionOperationInput, ExecutionMethodResourceTargetDTO, ExternalCanisterIdDTO,
    ExternalCanisterPermissionsCreateInput, ExternalCanisterRequestPoliciesCreateInput,
    HealthStatus, ListRequestsInput, ListRequestsOperationTypeDTO, ListRequestsResponse,
    PruneExternalCanisterOperationInput, PruneExternalCanisterResourceDTO, QuorumDTO,
    RequestApprovalStatusDTO, RequestOperationDTO, RequestOperationInput, RequestPolicyRuleDTO,
    RequestSpecifierDTO, RequestStatusDTO, RestoreExternalCanisterOperationInput, Snapshot,
    SnapshotExternalCanisterOperationInput, UserSpecifierDTO, ValidationMethodResourceTargetDTO,
};
use std::str::FromStr;
use std::time::Duration;

const MAX_CANISTER_SNAPSHOTS: usize = 10;

#[test]
fn successful_four_eyes_upgrade() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    // create and install the counter canister
    let canister_id = create_canister(&env, canister_ids.station);
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

    // submit canister upgrade request
    let chunk_len = module_bytes.len() / 3;
    assert!(0 < chunk_len && chunk_len < 1_000_000);
    let (base_chunk, module_extra_chunks) =
        upload_canister_chunks_to_asset_canister(&env, module_bytes, chunk_len);
    let change_canister_operation =
        RequestOperationInput::ChangeExternalCanister(ChangeExternalCanisterOperationInput {
            canister_id,
            mode: CanisterInstallMode::Upgrade,
            module: base_chunk,
            module_extra_chunks: Some(module_extra_chunks),
            arg: None,
        });

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
        RequestStatusDTO::Rejected => (),
        _ => panic!("Request should have been rejected."),
    };

    // set four eyes principle for canister changes
    let add_request_policy =
        RequestOperationInput::AddRequestPolicy(AddRequestPolicyOperationInput {
            specifier: RequestSpecifierDTO::ChangeExternalCanister(
                ExternalCanisterIdDTO::Canister(canister_id),
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
        env, canister_ids, ..
    } = setup_new_env();

    // create and install the counter canister
    let canister_id = create_canister(&env, canister_ids.station);
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
    let chunk_len = module_bytes.len() / 3;
    assert!(0 < chunk_len && chunk_len < 1_000_000);
    let (base_chunk, module_extra_chunks) =
        upload_canister_chunks_to_asset_canister(&env, module_bytes, chunk_len);
    let change_canister_operation =
        RequestOperationInput::ChangeExternalCanister(ChangeExternalCanisterOperationInput {
            canister_id,
            mode: CanisterInstallMode::Upgrade,
            module: base_chunk.clone(),
            module_extra_chunks: Some(module_extra_chunks.clone()),
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
        RequestOperationInput::ChangeExternalCanister(ChangeExternalCanisterOperationInput {
            canister_id,
            mode: CanisterInstallMode::Reinstall,
            module: base_chunk,
            module_extra_chunks: Some(module_extra_chunks),
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
        ListRequestsOperationTypeDTO::ChangeExternalCanister(Some(canister_id));
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
    let list_requests_operation_type = ListRequestsOperationTypeDTO::ChangeExternalCanister(Some(
        Principal::management_canister(),
    ));
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
    let list_requests_operation_type = ListRequestsOperationTypeDTO::ChangeExternalCanister(None);
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
fn create_external_canister_and_check_status() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    // create new user identities and add them to the station
    let user_a = user_test_id(0);
    add_user(&env, user_a, vec![], canister_ids.station);
    let user_b = user_test_id(1);
    add_user(&env, user_b, vec![], canister_ids.station);

    // submit request to create an external canister
    let create_canister_operation =
        RequestOperationInput::CreateExternalCanister(CreateExternalCanisterOperationInput {
            kind: CreateExternalCanisterOperationKindDTO::CreateNew(
                CreateExternalCanisterOperationKindCreateNewDTO {
                    initial_cycles: None,
                    subnet_selection: None,
                },
            ),
            name: "test".to_string(),
            description: None,
            labels: None,
            metadata: None,
            permissions: ExternalCanisterPermissionsCreateInput {
                calls: vec![],
                read: AllowDTO {
                    auth_scope: station_api::AuthScopeDTO::Restricted,
                    user_groups: vec![],
                    users: vec![],
                },
                change: AllowDTO {
                    auth_scope: station_api::AuthScopeDTO::Restricted,
                    user_groups: vec![],
                    users: vec![],
                },
            },
            request_policies: ExternalCanisterRequestPoliciesCreateInput {
                change: vec![],
                calls: vec![],
            },
        });

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
        RequestStatusDTO::Rejected => (),
        _ => panic!("Request should have been rejected."),
    };
    match rejected_request.operation {
        RequestOperationDTO::CreateExternalCanister(operation) => {
            assert!(operation.canister_id.is_none())
        }
        _ => panic!(
            "Unexpected request operation type: {:?}",
            rejected_request.operation
        ),
    };

    // set four eyes principle for creating external canisters
    let add_request_policy =
        RequestOperationInput::AddRequestPolicy(AddRequestPolicyOperationInput {
            specifier: RequestSpecifierDTO::CreateExternalCanister,
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

    // submit the request to create a external canister again
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
        RequestOperationDTO::CreateExternalCanister(operation) => {
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
        RequestOperationDTO::CreateExternalCanister(operation) => operation.canister_id.unwrap(),
        _ => panic!(
            "Unexpected request operation type: {:?}",
            executed_request.operation
        ),
    };

    // check canister status on behalf of the station and ensure that the canister is empty
    let status = canister_status(&env, Some(canister_ids.station), canister_id);
    assert_eq!(status.module_hash, None);

    // checking canister status on behalf of the user_a
    let canister_id_record = CanisterIdRecord { canister_id };
    let status: (CanisterStatusResponse,) = update_candid_as(
        &env,
        canister_ids.station,
        user_a,
        "canister_status",
        (canister_id_record.clone(),),
    )
    .unwrap();
    assert_eq!(status.0.module_hash, None);

    // checking canister status on behalf of the user_c which has no permission to call canister_status
    let user_c = user_test_id(2);
    let err = update_candid_as::<_, (CanisterStatusResponse,)>(
        &env,
        canister_ids.station,
        user_c,
        "canister_status",
        (canister_id_record.clone(),),
    )
    .unwrap_err();
    assert!(err.reject_message.contains(&format!(
        "Unauthorized access to resources: ExternalCanister(Read(Canister({})))",
        canister_id
    )));
}

#[test]
fn call_external_canister_test() {
    const T: u128 = 1_000_000_000_000;

    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    // create and install the counter canister (validation)
    let validation_canister_id = create_canister(&env, Principal::anonymous());
    let module_bytes = wat::parse_str(COUNTER_WAT).unwrap();
    let mut sha256 = Sha256::new();
    sha256.update(module_bytes.clone());
    let module_hash = sha256.finalize().to_vec();
    env.install_canister(validation_canister_id, module_bytes.clone(), vec![], None);

    // create and install the counter canister (execution)
    let execution_canister_id = create_canister(&env, Principal::anonymous());
    let module_bytes = wat::parse_str(COUNTER_WAT).unwrap();
    env.install_canister(execution_canister_id, module_bytes.clone(), vec![], None);

    // check canister status and ensure that the WASM matches the counter canister module
    // for both the validation and execution canisters
    let status = canister_status(&env, None, validation_canister_id);
    assert_eq!(status.module_hash, Some(module_hash.clone()));
    let status = canister_status(&env, None, execution_canister_id);
    assert_eq!(status.module_hash, Some(module_hash.clone()));

    // the counters should initially be set at 0 and the cycles balance between 95T and 100T cycles
    let ctr = update_raw(
        &env,
        validation_canister_id,
        Principal::anonymous(),
        "read",
        vec![],
    )
    .unwrap();
    assert_eq!(ctr, 0_u32.to_le_bytes());
    let cycles = env.cycle_balance(validation_canister_id);
    assert!((95 * T..=100 * T).contains(&cycles));
    let ctr = update_raw(
        &env,
        execution_canister_id,
        Principal::anonymous(),
        "read",
        vec![],
    )
    .unwrap();
    assert_eq!(ctr, 0_u32.to_le_bytes());
    let cycles = env.cycle_balance(execution_canister_id);
    assert!((95 * T..=100 * T).contains(&cycles));

    // create new user identities and add them to the station
    let user_a = user_test_id(0);
    add_user(&env, user_a, vec![], canister_ids.station);
    let user_b = user_test_id(1);
    add_user(&env, user_b, vec![], canister_ids.station);

    // submitting call external canister request fails due to insufficient permissions to create such requests
    let failing_validation_method = CanisterMethodDTO {
        canister_id: validation_canister_id,
        method_name: "bad".to_string(),
    };
    let validation_method = CanisterMethodDTO {
        canister_id: validation_canister_id,
        method_name: "inc".to_string(),
    };
    let execution_method = CanisterMethodDTO {
        canister_id: execution_canister_id,
        method_name: "set".to_string(),
    };
    let call_canister_operation =
        RequestOperationInput::CallExternalCanister(CallExternalCanisterOperationInput {
            validation_method: Some(validation_method.clone()),
            execution_method: execution_method.clone(),
            arg: Some(42_u32.to_le_bytes().to_vec()),
            execution_method_cycles: Some(10_000_000_000_000),
        });
    let trap_message = submit_request_with_expected_trap(
        &env,
        user_a,
        canister_ids.station,
        call_canister_operation.clone(),
    );
    assert!(trap_message.contains(
        "Canister called `ic0.trap` with message: 'Unauthorized access to resources: ExternalCanister(Call(CallExternalCanister("
    ));

    // nothing should have changed so far
    let ctr = update_raw(
        &env,
        validation_canister_id,
        Principal::anonymous(),
        "read",
        vec![],
    )
    .unwrap();
    assert_eq!(ctr, 0_u32.to_le_bytes());
    let cycles = env.cycle_balance(validation_canister_id);
    assert!((95 * T..=100 * T).contains(&cycles));
    let ctr = update_raw(
        &env,
        execution_canister_id,
        Principal::anonymous(),
        "read",
        vec![],
    )
    .unwrap();
    assert_eq!(ctr, 0_u32.to_le_bytes());
    let cycles = env.cycle_balance(execution_canister_id);
    assert!((95 * T..=100 * T).contains(&cycles));

    // allow anyone to create call external canister requests with a given validation and execution method
    let add_permission = RequestOperationInput::EditPermission(EditPermissionOperationInput {
        resource: station_api::ResourceDTO::ExternalCanister(
            station_api::ExternalCanisterResourceActionDTO::Call(
                CallExternalCanisterResourceTargetDTO {
                    validation_method: ValidationMethodResourceTargetDTO::ValidationMethod(
                        validation_method.clone(),
                    ),
                    execution_method: ExecutionMethodResourceTargetDTO::ExecutionMethod(
                        execution_method.clone(),
                    ),
                },
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

    // also allow anyone to create call external canister requests with a validation method always failing validation
    let add_permission = RequestOperationInput::EditPermission(EditPermissionOperationInput {
        resource: station_api::ResourceDTO::ExternalCanister(
            station_api::ExternalCanisterResourceActionDTO::Call(
                CallExternalCanisterResourceTargetDTO {
                    validation_method: ValidationMethodResourceTargetDTO::ValidationMethod(
                        failing_validation_method.clone(),
                    ),
                    execution_method: ExecutionMethodResourceTargetDTO::ExecutionMethod(
                        execution_method.clone(),
                    ),
                },
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

    // now the request to call the counter canister can be successfully submitted
    // and it is immediately rejected because nobody can actually approve it
    let call_canister_operation_request = submit_request(
        &env,
        user_a,
        canister_ids.station,
        call_canister_operation.clone(),
    );
    let rejected_request = get_request(
        &env,
        user_a,
        canister_ids.station,
        call_canister_operation_request,
    );
    match rejected_request.status {
        RequestStatusDTO::Rejected => (),
        _ => panic!("Request should have been rejected."),
    };

    // the validation counter should increase now
    let ctr = update_raw(
        &env,
        validation_canister_id,
        Principal::anonymous(),
        "read",
        vec![],
    )
    .unwrap();
    assert_eq!(ctr, 2_u32.to_le_bytes());
    let cycles = env.cycle_balance(validation_canister_id);
    assert!((95 * T..=100 * T).contains(&cycles));
    let ctr = update_raw(
        &env,
        execution_canister_id,
        Principal::anonymous(),
        "read",
        vec![],
    )
    .unwrap();
    assert_eq!(ctr, 0_u32.to_le_bytes());
    let cycles = env.cycle_balance(execution_canister_id);
    assert!((95 * T..=100 * T).contains(&cycles));

    bump_time_to_avoid_ratelimit(&env);

    // submit a call external canister request with failing validation
    let failing_validation_call_canister_operation =
        RequestOperationInput::CallExternalCanister(CallExternalCanisterOperationInput {
            validation_method: Some(failing_validation_method.clone()),
            execution_method: execution_method.clone(),
            arg: Some(42_u32.to_le_bytes().to_vec()),
            execution_method_cycles: Some(10_000_000_000_000),
        });
    let request_error = submit_request_raw(
        &env,
        user_a,
        canister_ids.station,
        failing_validation_call_canister_operation,
    )
    .unwrap()
    .0
    .unwrap_err();
    assert_eq!(
        request_error.message,
        Some("The request has failed validation.".to_string())
    );
    assert_eq!(
        *request_error.details.clone().unwrap().get("info").unwrap(),
        "failed to validate call external canister request: bad".to_string()
    );

    // the validation counter should increase now since the validation was performed and returned a failure
    let ctr = update_raw(
        &env,
        validation_canister_id,
        Principal::anonymous(),
        "read",
        vec![],
    )
    .unwrap();
    assert_eq!(ctr, 4_u32.to_le_bytes());
    let cycles = env.cycle_balance(validation_canister_id);
    assert!((95 * T..=100 * T).contains(&cycles));
    let ctr = update_raw(
        &env,
        execution_canister_id,
        Principal::anonymous(),
        "read",
        vec![],
    )
    .unwrap();
    assert_eq!(ctr, 0_u32.to_le_bytes());
    let cycles = env.cycle_balance(execution_canister_id);
    assert!((95 * T..=100 * T).contains(&cycles));

    // submit a request with no validation method which is illegal given the permissions set so far
    let illegal_call_canister_operation =
        RequestOperationInput::CallExternalCanister(CallExternalCanisterOperationInput {
            validation_method: None,
            execution_method: execution_method.clone(),
            arg: None,
            execution_method_cycles: None,
        });
    let trap_message = submit_request_with_expected_trap(
        &env,
        user_a,
        canister_ids.station,
        illegal_call_canister_operation.clone(),
    );
    assert!(trap_message.contains(
        "Canister called `ic0.trap` with message: 'Unauthorized access to resources: ExternalCanister(Call(CallExternalCanister("
    ));

    bump_time_to_avoid_ratelimit(&env);

    // submit a request labeling the execution method as the validation method which is illegal given the permissions set so far
    let illegal_call_canister_operation =
        RequestOperationInput::CallExternalCanister(CallExternalCanisterOperationInput {
            validation_method: Some(execution_method.clone()),
            execution_method: execution_method.clone(),
            arg: None,
            execution_method_cycles: None,
        });
    let trap_message = submit_request_with_expected_trap(
        &env,
        user_a,
        canister_ids.station,
        illegal_call_canister_operation.clone(),
    );
    assert!(trap_message.contains(
        "Canister called `ic0.trap` with message: 'Unauthorized access to resources: ExternalCanister(Call(CallExternalCanister("
    ));

    // submit a request labeling the validation method as the execution method which is illegal given the permissions set so far
    let illegal_call_canister_operation =
        RequestOperationInput::CallExternalCanister(CallExternalCanisterOperationInput {
            validation_method: Some(validation_method.clone()),
            execution_method: validation_method.clone(),
            arg: None,
            execution_method_cycles: None,
        });
    let trap_message = submit_request_with_expected_trap(
        &env,
        user_a,
        canister_ids.station,
        illegal_call_canister_operation.clone(),
    );
    assert!(trap_message.contains(
        "Canister called `ic0.trap` with message: 'Unauthorized access to resources: ExternalCanister(Call(CallExternalCanister("
    ));

    // nothing should have changed
    let ctr = update_raw(
        &env,
        validation_canister_id,
        Principal::anonymous(),
        "read",
        vec![],
    )
    .unwrap();
    assert_eq!(ctr, 4_u32.to_le_bytes());
    let cycles = env.cycle_balance(validation_canister_id);
    assert!((95 * T..=100 * T).contains(&cycles));
    let ctr = update_raw(
        &env,
        execution_canister_id,
        Principal::anonymous(),
        "read",
        vec![],
    )
    .unwrap();
    assert_eq!(ctr, 0_u32.to_le_bytes());
    let cycles = env.cycle_balance(execution_canister_id);
    assert!((95 * T..=100 * T).contains(&cycles));

    // set four eyes principle (request approval policy) for calling external canisters
    let add_request_policy =
        RequestOperationInput::AddRequestPolicy(AddRequestPolicyOperationInput {
            specifier: RequestSpecifierDTO::CallExternalCanister(
                CallExternalCanisterResourceTargetDTO {
                    validation_method: ValidationMethodResourceTargetDTO::ValidationMethod(
                        validation_method,
                    ),
                    execution_method: ExecutionMethodResourceTargetDTO::ExecutionMethod(
                        execution_method,
                    ),
                },
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

    bump_time_to_avoid_ratelimit(&env);

    // submit the request to call the counter canister again
    let call_canister_operation_request = submit_request(
        &env,
        user_a,
        canister_ids.station,
        call_canister_operation.clone(),
    );
    let created_request = get_request(
        &env,
        user_a,
        canister_ids.station,
        call_canister_operation_request.clone(),
    );
    match created_request.status {
        RequestStatusDTO::Created => (),
        _ => panic!("Request should be created."),
    };
    let arg_rendering = match created_request.operation {
        RequestOperationDTO::CallExternalCanister(operation) => operation.arg_rendering,
        _ => panic!(
            "Unexpected request operation type: {:?}",
            created_request.operation
        ),
    };
    assert_eq!(arg_rendering, Some("valid".to_string()));

    // the validation canister counter should increase again now that one more request has been successfully created
    let ctr = update_raw(
        &env,
        validation_canister_id,
        Principal::anonymous(),
        "read",
        vec![],
    )
    .unwrap();
    assert_eq!(ctr, 6_u32.to_le_bytes());
    let cycles = env.cycle_balance(validation_canister_id);
    assert!((95 * T..=100 * T).contains(&cycles));
    let ctr = update_raw(
        &env,
        execution_canister_id,
        Principal::anonymous(),
        "read",
        vec![],
    )
    .unwrap();
    assert_eq!(ctr, 0_u32.to_le_bytes());
    let cycles = env.cycle_balance(execution_canister_id);
    assert!((95 * T..=100 * T).contains(&cycles));

    // let the admin user reject the request => the request stays open as the second user can also approve it
    submit_request_approval(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        call_canister_operation_request.clone(),
        RequestApprovalStatusDTO::Rejected,
    );
    let created_request = get_request(
        &env,
        user_a,
        canister_ids.station,
        call_canister_operation_request.clone(),
    );
    match created_request.status {
        RequestStatusDTO::Created => (),
        _ => panic!("Request should be created."),
    };

    // nothing should have changed
    let ctr = update_raw(
        &env,
        validation_canister_id,
        Principal::anonymous(),
        "read",
        vec![],
    )
    .unwrap();
    assert_eq!(ctr, 6_u32.to_le_bytes());
    let cycles = env.cycle_balance(validation_canister_id);
    assert!((95 * T..=100 * T).contains(&cycles));
    let ctr = update_raw(
        &env,
        execution_canister_id,
        Principal::anonymous(),
        "read",
        vec![],
    )
    .unwrap();
    assert_eq!(ctr, 0_u32.to_le_bytes());
    let cycles = env.cycle_balance(execution_canister_id);
    assert!((95 * T..=100 * T).contains(&cycles));

    // the second user approves and then the request will eventually become completed
    submit_request_approval(
        &env,
        user_b,
        canister_ids.station,
        call_canister_operation_request.clone(),
        RequestApprovalStatusDTO::Approved,
    );
    let executed_request = wait_for_request(
        &env,
        user_a,
        canister_ids.station,
        call_canister_operation_request.clone(),
    )
    .unwrap();

    // the execution method should have been called setting the counter to 42 on the execution canister
    // and the cycles transferred and accepted
    let ctr = update_raw(
        &env,
        validation_canister_id,
        Principal::anonymous(),
        "read",
        vec![],
    )
    .unwrap();
    assert_eq!(ctr, 6_u32.to_le_bytes());
    let cycles = env.cycle_balance(validation_canister_id);
    assert!((95 * T..=100 * T).contains(&cycles));
    let ctr = update_raw(
        &env,
        execution_canister_id,
        Principal::anonymous(),
        "read",
        vec![],
    )
    .unwrap();
    assert_eq!(ctr, 42_u32.to_le_bytes());
    let cycles = env.cycle_balance(execution_canister_id);
    assert!((105 * T..=110 * T).contains(&cycles));

    // check the execution method reply to match the candid encoding of '(variant {Ok = "good"})'
    let execution_method_reply = match executed_request.operation {
        RequestOperationDTO::CallExternalCanister(operation) => operation.execution_method_reply,
        _ => panic!(
            "Unexpected request operation type: {:?}",
            executed_request.operation
        ),
    };
    assert_eq!(
        execution_method_reply,
        Some(hex::decode("4449444c016b01bc8a017101000004676f6f64").unwrap())
    );
}

#[test]
fn create_external_canister_with_too_many_cycles() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let create_canister_operation = |name: &str, initial_cycles| {
        RequestOperationInput::CreateExternalCanister(CreateExternalCanisterOperationInput {
            kind: CreateExternalCanisterOperationKindDTO::CreateNew(
                CreateExternalCanisterOperationKindCreateNewDTO {
                    initial_cycles,
                    subnet_selection: None,
                },
            ),
            name: name.to_string(),
            description: None,
            labels: None,
            metadata: None,
            permissions: ExternalCanisterPermissionsCreateInput {
                calls: vec![],
                read: AllowDTO {
                    auth_scope: station_api::AuthScopeDTO::Restricted,
                    user_groups: vec![],
                    users: vec![],
                },
                change: AllowDTO {
                    auth_scope: station_api::AuthScopeDTO::Restricted,
                    user_groups: vec![],
                    users: vec![],
                },
            },
            request_policies: ExternalCanisterRequestPoliciesCreateInput {
                change: vec![],
                calls: vec![],
            },
        })
    };

    // request to create a test canister with 1T cycles (this should succeed)
    let create_test_canister_operation = create_canister_operation("test", Some(1_000_000_000_000));

    // request to create a canister with more cycles than the station has (this should fail)
    let station_cycles = env.cycle_balance(canister_ids.station);
    let create_rich_canister_operation =
        create_canister_operation("rich", Some(2 * station_cycles as u64));

    // submit requests for creating the test canister and a canister with too many cycles
    // to be executed concurrently
    let test_canister_request = submit_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        create_test_canister_operation,
    );
    let rich_canister_request = submit_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        create_rich_canister_operation,
    );

    // admin cannot trigger requests via the private `try_execute_request` endpoint
    let test_canister_request_id = uuid::Uuid::from_str(&test_canister_request.id).unwrap();
    let bytes = Encode!(&test_canister_request_id.as_bytes()).unwrap();
    let err = update_raw(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        "try_execute_request",
        bytes,
    )
    .unwrap_err();
    assert!(err
        .reject_message
        .contains("The method `try_execute_request` can only be called by the station canister."));

    // wait for the requests to be executed
    let test_canister_request = wait_for_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        test_canister_request,
    )
    .unwrap();
    let rich_canister_request_status = wait_for_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        rich_canister_request,
    )
    .unwrap_err()
    .unwrap();

    // the test canister with 1T cycles should be successfully created
    match test_canister_request.status {
        RequestStatusDTO::Completed { .. } => (),
        _ => panic!(
            "Unexpected request status: {:?}",
            test_canister_request.status
        ),
    };
    // check the test canister status on behalf of the station and ensure that the canister is empty
    let canister_id = match test_canister_request.operation {
        RequestOperationDTO::CreateExternalCanister(operation) => operation.canister_id.unwrap(),
        _ => panic!(
            "Unexpected request operation type: {:?}",
            test_canister_request.operation
        ),
    };
    let status = canister_status(&env, Some(canister_ids.station), canister_id);
    assert_eq!(status.module_hash, None);

    // the canister with too many cycles failed to be created because the station would be out of cycles
    match rich_canister_request_status {
        RequestStatusDTO::Failed { reason } => {
            assert_eq!(reason.unwrap(), format!("Request execution failed due to `failed to add external canister: FAILED: The external canister operation failed due to Canister {} has insufficient cycles balance to transfer {} cycles.`.", canister_ids.station, 2 * station_cycles));
        }
        _ => panic!(
            "Unexpected request status: {:?}",
            rich_canister_request_status
        ),
    };
    // the station should still be healthy
    let health_status =
        get_core_canister_health_status(&env, WALLET_ADMIN_USER, canister_ids.station);
    assert_eq!(health_status, HealthStatus::Healthy);
}

#[test]
fn create_external_canister_on_different_subnet() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    // create a canister on the fiduciary subnet
    let subnet_selection = Some(SubnetSelection::Filter(SubnetFilter {
        subnet_type: Some("fiduciary".to_string()),
    }));
    let create_canister_operation =
        RequestOperationInput::CreateExternalCanister(CreateExternalCanisterOperationInput {
            kind: CreateExternalCanisterOperationKindDTO::CreateNew(
                CreateExternalCanisterOperationKindCreateNewDTO {
                    initial_cycles: None,
                    subnet_selection,
                },
            ),
            name: "test".to_string(),
            description: None,
            labels: None,
            metadata: None,
            permissions: ExternalCanisterPermissionsCreateInput {
                calls: vec![],
                read: AllowDTO {
                    auth_scope: station_api::AuthScopeDTO::Restricted,
                    user_groups: vec![],
                    users: vec![],
                },
                change: AllowDTO {
                    auth_scope: station_api::AuthScopeDTO::Restricted,
                    user_groups: vec![],
                    users: vec![],
                },
            },
            request_policies: ExternalCanisterRequestPoliciesCreateInput {
                change: vec![],
                calls: vec![],
            },
        });
    let create_canister_request = execute_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        create_canister_operation,
    )
    .unwrap();
    let canister_id = match create_canister_request.operation {
        RequestOperationDTO::CreateExternalCanister(operation) => operation.canister_id.unwrap(),
        _ => panic!(
            "Unexpected request operation type: {:?}",
            create_canister_request.operation
        ),
    };

    // check canister status on behalf of the station and ensure that the canister is empty
    let status = canister_status(&env, Some(canister_ids.station), canister_id);
    assert_eq!(status.module_hash, None);

    // check that the canister has been deployed to the fiduciary subnet
    assert_eq!(
        env.get_subnet(canister_id).unwrap(),
        env.topology().get_fiduciary().unwrap()
    );
    // which is different from the subnet to which the station is deployed
    assert_ne!(
        env.get_subnet(canister_id).unwrap(),
        env.get_subnet(canister_ids.control_panel).unwrap()
    );
}

#[test]
fn snapshot_external_canister_test() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    // create and install the counter canister (external canister)
    let external_canister_id = create_canister(&env, canister_ids.station);
    let module_bytes = wat::parse_str(COUNTER_WAT).unwrap();
    let mut sha256 = Sha256::new();
    sha256.update(module_bytes.clone());
    let module_hash = sha256.finalize().to_vec();
    env.install_canister(
        external_canister_id,
        module_bytes,
        vec![],
        Some(canister_ids.station),
    );

    // check canister status and ensure that the WASM matches the counter canister module
    let status = canister_status(&env, Some(canister_ids.station), external_canister_id);
    assert_eq!(status.module_hash, Some(module_hash.clone()));

    // the counter should initially be set at 0
    let ctr = update_raw(
        &env,
        external_canister_id,
        Principal::anonymous(),
        "read",
        vec![],
    )
    .unwrap();
    assert_eq!(ctr, 0_u32.to_le_bytes());

    // bump the counter
    update_raw(
        &env,
        external_canister_id,
        Principal::anonymous(),
        "inc",
        vec![],
    )
    .unwrap();

    // the counter should now be equal to 2
    let ctr = update_raw(
        &env,
        external_canister_id,
        Principal::anonymous(),
        "read",
        vec![],
    )
    .unwrap();
    assert_eq!(ctr, 2_u32.to_le_bytes());

    // retrieve the existing snapshots from the management canister: there should be no snapshots yet
    let snapshots: Vec<_> = env
        .list_canister_snapshots(external_canister_id, Some(canister_ids.station))
        .unwrap();
    assert!(snapshots.is_empty());

    // execute `MAX_CANISTER_SNAPSHOTS` requests taking a snapshot (max. number of canister snapshots)
    let mut snapshot_ids = vec![];
    for _ in 0..MAX_CANISTER_SNAPSHOTS {
        let snapshot_canister_operation = RequestOperationInput::SnapshotExternalCanister(
            SnapshotExternalCanisterOperationInput {
                canister_id: external_canister_id,
                replace_snapshot: None,
                force: false,
            },
        );
        let request = execute_request(
            &env,
            WALLET_ADMIN_USER,
            canister_ids.station,
            snapshot_canister_operation,
        )
        .unwrap();

        // fetch the snapshot id from the executed request
        let snapshot_id = match request.operation {
            RequestOperationDTO::SnapshotExternalCanister(op) => op.snapshot_id.unwrap(),
            _ => panic!("Unexpected request operation: {:?}", request.operation),
        };
        snapshot_ids.push(snapshot_id);
    }

    // retrieve the existing snapshots from the management canister:
    // there should be `MAX_CANISTER_SNAPSHOTS` snapshots with snapshot ids from the requests
    let snapshots = env
        .list_canister_snapshots(external_canister_id, Some(canister_ids.station))
        .unwrap();
    assert_eq!(snapshots.len(), MAX_CANISTER_SNAPSHOTS);
    for i in 0..MAX_CANISTER_SNAPSHOTS {
        assert_eq!(snapshots[i].id, hex::decode(&snapshot_ids[i]).unwrap());
    }

    // retrieve the existing snapshots from a dedicated endpoint of the station:
    // the snapshots should match the snapshots from the management canister
    let res: (ApiResult<Vec<Snapshot>>,) = update_candid_as(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        "canister_snapshots",
        (CanisterIdRecord {
            canister_id: external_canister_id,
        },),
    )
    .unwrap();
    let snapshots_via_orbit = res.0.unwrap();
    assert_eq!(snapshots_via_orbit.len(), MAX_CANISTER_SNAPSHOTS);
    for i in 0..MAX_CANISTER_SNAPSHOTS {
        assert_eq!(
            snapshots_via_orbit[i].snapshot_id,
            hex::encode(&snapshots[i].id)
        );
        assert_eq!(
            snapshots_via_orbit[i].taken_at_timestamp,
            timestamp_to_rfc3339(&snapshots[i].taken_at_timestamp)
        );
        assert_eq!(snapshots_via_orbit[i].total_size, snapshots[i].total_size);
    }

    // bump the counter
    update_raw(
        &env,
        external_canister_id,
        Principal::anonymous(),
        "inc",
        vec![],
    )
    .unwrap();

    // the counter should now be equal to 4
    let ctr = update_raw(
        &env,
        external_canister_id,
        Principal::anonymous(),
        "read",
        vec![],
    )
    .unwrap();
    assert_eq!(ctr, 4_u32.to_le_bytes());

    // taking another snapshot without specifying a snapshot to replace should fail
    let snapshot_canister_operation =
        RequestOperationInput::SnapshotExternalCanister(SnapshotExternalCanisterOperationInput {
            canister_id: external_canister_id,
            replace_snapshot: None,
            force: false,
        });
    let failed_request_status = execute_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        snapshot_canister_operation,
    )
    .unwrap_err()
    .unwrap();
    match failed_request_status {
        RequestStatusDTO::Failed { reason } => assert!(reason.unwrap().contains(&format!(
            "Canister {} has reached the maximum number of snapshots allowed: {}.",
            external_canister_id, MAX_CANISTER_SNAPSHOTS
        ))),
        _ => panic!("Unexpected request status: {:?}", failed_request_status),
    };

    // restore the canister from the snapshot
    let restore_canister_operation =
        RequestOperationInput::RestoreExternalCanister(RestoreExternalCanisterOperationInput {
            canister_id: external_canister_id,
            snapshot_id: snapshot_ids[0].clone(),
        });
    execute_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        restore_canister_operation,
    )
    .unwrap();

    // the counter should now be back at 2
    let ctr = update_raw(
        &env,
        external_canister_id,
        Principal::anonymous(),
        "read",
        vec![],
    )
    .unwrap();
    assert_eq!(ctr, 2_u32.to_le_bytes());

    // taking another snapshot succeeds if we replace an existing snapshot
    let snapshot_canister_operation =
        RequestOperationInput::SnapshotExternalCanister(SnapshotExternalCanisterOperationInput {
            canister_id: external_canister_id,
            replace_snapshot: Some(snapshot_ids[0].clone()),
            force: false,
        });
    let request = execute_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        snapshot_canister_operation,
    )
    .unwrap();

    // fetch the new snapshot id from the executed request
    let new_snapshot_id = match request.operation {
        RequestOperationDTO::SnapshotExternalCanister(op) => op.snapshot_id.unwrap(),
        _ => panic!("Unexpected request operation: {:?}", request.operation),
    };
    for snapshot_id in &snapshot_ids {
        assert_ne!(new_snapshot_id, *snapshot_id);
    }

    let mut new_snapshot_ids = snapshot_ids[1..].to_vec();
    new_snapshot_ids.push(new_snapshot_id);

    // retrieve the existing snapshots from the management canister:
    // there should be `MAX_CANISTER_SNAPSHOTS` snapshots with the new snapshot id from the request
    // and without the snapshot specified to be replaced
    let snapshots: Vec<_> = env
        .list_canister_snapshots(external_canister_id, Some(canister_ids.station))
        .unwrap()
        .into_iter()
        .map(|snapshot| snapshot.id)
        .collect();
    assert_eq!(
        snapshots,
        new_snapshot_ids
            .iter()
            .map(|snapshot_id| hex::decode(snapshot_id).unwrap())
            .collect::<Vec<_>>()
    );

    // prune the snapshots
    for snapshot_id in new_snapshot_ids {
        let prune_canister_operation =
            RequestOperationInput::PruneExternalCanister(PruneExternalCanisterOperationInput {
                canister_id: external_canister_id,
                prune: PruneExternalCanisterResourceDTO::Snapshot(snapshot_id),
            });
        execute_request(
            &env,
            WALLET_ADMIN_USER,
            canister_ids.station,
            prune_canister_operation,
        )
        .unwrap();
    }

    // retrieve the existing snapshots from the management canister: there should be no snapshots anymore
    let snapshots: Vec<_> = env
        .list_canister_snapshots(external_canister_id, Some(canister_ids.station))
        .unwrap();
    assert!(snapshots.is_empty());
}

#[test]
fn prune_external_canister_chunk_store_test() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    // create and install the test canister (external canister)
    let external_canister_id = deploy_test_canister(&env, canister_ids.station);

    // check that the external canister is not empty
    let status = canister_status(&env, Some(canister_ids.station), external_canister_id);
    status.module_hash.unwrap();

    // check that the external canister has an empty chunk store
    let chunks = env
        .stored_chunks(external_canister_id, Some(canister_ids.station))
        .unwrap();
    assert!(chunks.is_empty());

    // upload a chunk
    let chunk = vec![1, 2, 3, 4];
    let chunk_hash = env
        .upload_chunk(
            external_canister_id,
            Some(canister_ids.station),
            chunk.clone(),
        )
        .unwrap();
    assert_eq!(chunk_hash, hash(&chunk));

    // check that the chunk is indeed in the external canister's chunk store
    let chunks = env
        .stored_chunks(external_canister_id, Some(canister_ids.station))
        .unwrap();
    assert_eq!(chunks, vec![chunk_hash]);

    // prune the external canister's chunk store
    let prune_canister_operation =
        RequestOperationInput::PruneExternalCanister(PruneExternalCanisterOperationInput {
            canister_id: external_canister_id,
            prune: PruneExternalCanisterResourceDTO::ChunkStore,
        });
    execute_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        prune_canister_operation,
    )
    .unwrap();

    // check that the external canister is still not empty
    let status = canister_status(&env, Some(canister_ids.station), external_canister_id);
    status.module_hash.unwrap();

    // check that the external canister has an empty chunk store again
    let chunks = env
        .stored_chunks(external_canister_id, Some(canister_ids.station))
        .unwrap();
    assert!(chunks.is_empty());
}

#[test]
fn prune_external_canister_state_test() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    // create and install the test canister (external canister)
    let external_canister_id = deploy_test_canister(&env, canister_ids.station);

    // check that the external canister is not empty
    let status = canister_status(&env, Some(canister_ids.station), external_canister_id);
    status.module_hash.unwrap();

    // prune the external canister
    let prune_canister_operation =
        RequestOperationInput::PruneExternalCanister(PruneExternalCanisterOperationInput {
            canister_id: external_canister_id,
            prune: PruneExternalCanisterResourceDTO::State,
        });
    execute_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        prune_canister_operation,
    )
    .unwrap();

    // check that the external canister is empty now
    let status = canister_status(&env, Some(canister_ids.station), external_canister_id);
    assert_eq!(status.module_hash, None);
}

#[test]
fn snapshot_unstoppable_external_canister_test() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    // create and install the test canister (external canister)
    let external_canister_id = deploy_test_canister(&env, canister_ids.station);

    // make the test canister unstoppable by submitting an update call to the method "unstoppable"
    // and executing a round to kick off the (indefinite) update call execution
    env.submit_call(
        external_canister_id,
        Principal::anonymous(),
        "unstoppable",
        Encode!(&()).unwrap(),
    )
    .unwrap();
    env.tick();

    // submit a request taking a snapshot without forcing the snapshot:
    // we expect such a request to fail because the canister is unstoppable
    let snapshot_canister_operation =
        RequestOperationInput::SnapshotExternalCanister(SnapshotExternalCanisterOperationInput {
            canister_id: external_canister_id,
            replace_snapshot: None,
            force: false,
        });
    let request = submit_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        snapshot_canister_operation,
    );
    // timer's period for processing requests is 5 seconds
    env.advance_time(Duration::from_secs(5));
    // wait until the canister becomes stopping
    loop {
        let status = env
            .canister_status(external_canister_id, Some(canister_ids.station))
            .unwrap();
        if let CanisterStatusType::Stopping = status.status {
            break;
        }
    }
    // advance time by 5mins to time out canister stopping
    env.advance_time(Duration::from_secs(5 * 60));
    // the request should fail now
    let failed_request_status =
        wait_for_request(&env, WALLET_ADMIN_USER, canister_ids.station, request)
            .unwrap_err()
            .unwrap();
    match failed_request_status {
        RequestStatusDTO::Failed { reason } => {
            assert!(reason.unwrap().contains("Stop canister request timed out"))
        }
        _ => panic!("Unexpected request status: {:?}", failed_request_status),
    };

    // restart the canister
    env.start_canister(external_canister_id, Some(canister_ids.station))
        .unwrap();

    // submit a request taking a snapshot and force taking the snapshot:
    // we expect such a request to succeed although the canister is unstoppable
    let snapshot_canister_operation =
        RequestOperationInput::SnapshotExternalCanister(SnapshotExternalCanisterOperationInput {
            canister_id: external_canister_id,
            replace_snapshot: None,
            force: true,
        });
    let request = submit_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        snapshot_canister_operation,
    );
    // timer's period for processing requests is 5 seconds
    env.advance_time(Duration::from_secs(5));
    // wait until the canister becomes stopping
    loop {
        let status = env
            .canister_status(external_canister_id, Some(canister_ids.station))
            .unwrap();
        if let CanisterStatusType::Stopping = status.status {
            break;
        }
    }
    // advance time by 5mins to time out canister stopping
    env.advance_time(Duration::from_secs(5 * 60));
    // the request should succeed now
    wait_for_request(&env, WALLET_ADMIN_USER, canister_ids.station, request).unwrap();
}

#[test]
fn read_system_canister_info() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let system_info = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.station);
    let upgrader_id = system_info.upgrader_id;

    // checking canister snapshots/status of the upgrader on behalf of the admin succeeds
    let canister_id_record = CanisterIdRecord {
        canister_id: upgrader_id,
    };
    update_candid_as::<_, (ApiResult<Vec<Snapshot>>,)>(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        "canister_snapshots",
        (canister_id_record.clone(),),
    )
    .unwrap()
    .0
    .unwrap();
    update_candid_as::<_, (CanisterStatusResponse,)>(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        "canister_status",
        (canister_id_record.clone(),),
    )
    .unwrap();

    // checking canister snapshots/status of the upgrader on behalf of an unauthenticated user fails
    let user = user_test_id(0);
    let err = update_candid_as::<_, (ApiResult<Vec<Snapshot>>,)>(
        &env,
        canister_ids.station,
        user,
        "canister_snapshots",
        (canister_id_record.clone(),),
    )
    .unwrap_err();
    assert!(err.reject_message.contains("System(SystemInfo)"));
    let err = update_candid_as::<_, (CanisterStatusResponse,)>(
        &env,
        canister_ids.station,
        user,
        "canister_status",
        (canister_id_record.clone(),),
    )
    .unwrap_err();
    assert!(err.reject_message.contains("System(SystemInfo)"));
}
