use crate::setup::{create_canister, get_canister_wasm, WALLET_ADMIN_USER};
use crate::station_test_data::asset::list_assets;
use crate::system_upgrade_tests::STATION_UPGRADE_EXTRA_TICKS;
use candid::utils::ArgumentDecoder;
use candid::Principal;
use candid::{decode_args, CandidType, Encode};
use control_panel_api::UploadCanisterModulesInput;
use flate2::{write::GzEncoder, Compression};
use ic_certified_assets::types::{
    BatchOperation, CommitBatchArguments, CreateAssetArguments, CreateBatchResponse,
    CreateChunkArg, CreateChunkResponse, SetAssetContentArguments,
};
use ic_management_canister_types::CanisterStatusResult;
use orbit_essentials::api::ApiResult;
use orbit_essentials::cdk::api::management_canister::main::CanisterId;
use orbit_essentials::types::WasmModuleExtraChunks;
use orbit_essentials::utils::timestamp_to_rfc3339;
use pocket_ic::{query_candid_as, update_candid_as, PocketIc, RejectResponse};
use sha2::Digest;
use sha2::Sha256;
use station_api::{
    AccountDTO, AddAccountOperationInput, AddUserOperationInput, AllowDTO, ApiErrorDTO,
    CreateRequestInput, CreateRequestResponse, FetchAccountBalancesInput,
    FetchAccountBalancesResponse, GetPermissionResponse, GetRequestInput, GetRequestResponse,
    GetTransfersInput, GetTransfersResponse, HealthStatus, MeResponse, QuorumPercentageDTO,
    RequestApprovalStatusDTO, RequestDTO, RequestExecutionScheduleDTO, RequestOperationDTO,
    RequestOperationInput, RequestPolicyRuleDTO, RequestStatusDTO, ResourceIdDTO,
    SetDisasterRecoveryOperationDTO, SetDisasterRecoveryOperationInput, SubmitRequestApprovalInput,
    SubmitRequestApprovalResponse, SystemInfoDTO, SystemInfoResponse, SystemInstall, SystemUpgrade,
    SystemUpgradeOperationInput, SystemUpgradeTargetDTO, UserDTO, UserSpecifierDTO, UserStatusDTO,
    UuidDTO,
};
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;
use upgrader_api::{
    GetDisasterRecoveryStateResponse, GetLogsInput, GetLogsResponse, LogEntry, PaginationInput,
};
use uuid::Uuid;

pub const ADMIN_GROUP_ID: Uuid = Uuid::from_u128(302240678275694148452352); // 00000000-0000-4000-8000-000000000000
pub const OPERATOR_GROUP_ID: Uuid = Uuid::from_u128(302240678275694148452353); // 00000000-0000-4000-8000-000000000001
pub const NNS_ROOT_CANISTER_ID: Principal = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 3, 1, 1]);

pub const COUNTER_WAT: &str = r#"
    (module
        (import "ic0" "debug_print"
            (func $debug_print (param i32 i32)))
        (import "ic0" "msg_cycles_available"
            (func $ic0_msg_cycles_available (result i64)))
        (import "ic0" "msg_cycles_accept"
            (func $ic0_msg_cycles_accept (param $pages i64) (result i64)))
        (import "ic0" "msg_arg_data_copy"
            (func $msg_arg_data_copy (param i32 i32 i32)))
        (import "ic0" "msg_reply" (func $msg_reply))
        (import "ic0" "msg_reply_data_append"
            (func $msg_reply_data_append (param i32 i32)))
        (import "ic0" "stable_grow"
            (func $ic0_stable_grow (param $pages i32) (result i32)))
        (import "ic0" "stable_read"
            (func $ic0_stable_read (param $dst i32) (param $offset i32) (param $size i32)))
        (import "ic0" "stable_write"
            (func $ic0_stable_write (param $offset i32) (param $src i32) (param $size i32)))
        (func $init
            (drop (call $ic0_stable_grow (i32.const 1))))
        (func $set
            (call $msg_arg_data_copy (i32.const 0) (i32.const 0) (i32.const 4))
            (call $ic0_stable_write (i32.const 0) (i32.const 0) (i32.const 4))
            (drop (call $ic0_msg_cycles_accept (call $ic0_msg_cycles_available)))
            (call $msg_reply_data_append
                (i32.const 100) ;; the value at heap[100] encoding '(variant {Ok = "good"})' in candid
                (i32.const 19)) ;; length
            (call $msg_reply))
        (func $bad
            (call $doinc)
            (drop (call $ic0_msg_cycles_accept (call $ic0_msg_cycles_available)))
            (call $msg_reply_data_append
                (i32.const 200) ;; the value at heap[200] encoding '(variant {Err = "bad"})' in candid
                (i32.const 19)) ;; length
            (call $msg_reply))
        (func $inc
            (call $doinc)
            (drop (call $ic0_msg_cycles_accept (call $ic0_msg_cycles_available)))
            (call $msg_reply_data_append
                (i32.const 300) ;; the value at heap[300] encoding '(variant {Ok = "valid"})' in candid
                (i32.const 20)) ;; length
            (call $msg_reply))
        (func $doinc
            (call $ic0_stable_read (i32.const 0) (i32.const 0) (i32.const 4))
            (i32.store
                (i32.const 0)
                (i32.add (i32.load (i32.const 0)) (i32.const 2)))
            (call $ic0_stable_write (i32.const 0) (i32.const 0) (i32.const 4)))
        (func $read
            (call $ic0_stable_read (i32.const 0) (i32.const 0) (i32.const 4))
            (call $msg_reply_data_append
                (i32.const 0) ;; the counter from heap[0]
                (i32.const 4)) ;; length
            (call $msg_reply))
        (memory $memory 1)
        (data (i32.const 100) "\44\49\44\4c\01\6b\01\bc\8a\01\71\01\00\00\04\67\6f\6f\64")
        (data (i32.const 200) "\44\49\44\4c\01\6b\01\c5\fe\d2\01\71\01\00\00\03\62\61\64")
        (data (i32.const 300) "\44\49\44\4c\01\6b\01\bc\8a\01\71\01\00\00\05\76\61\6c\69\64")
        (export "canister_init" (func $init))
        (export "canister_post_upgrade" (func $doinc))
        (export "canister_query read" (func $read))
        (export "canister_update set" (func $set))
        (export "canister_update bad" (func $bad))
        (export "canister_update inc" (func $inc))
    )"#;

pub fn controller_test_id() -> Principal {
    let mut bytes = 0_u64.to_le_bytes().to_vec();
    bytes.push(0xfd); // internal marker for controller test id
    bytes.push(0x01); // marker for opaque ids
    Principal::from_slice(&bytes)
}

pub fn minter_test_id() -> Principal {
    let mut bytes = 0_u64.to_le_bytes().to_vec();
    bytes.push(0xfc); // internal marker for minter test id
    bytes.push(0x01); // marker for opaque ids
    Principal::from_slice(&bytes)
}

pub fn user_test_id(n: u64) -> Principal {
    let mut bytes = n.to_le_bytes().to_vec();
    bytes.push(0xfe); // internal marker for user test ids
    bytes.push(0x01); // marker for opaque ids
    Principal::from_slice(&bytes)
}

pub fn try_get_request(
    env: &PocketIc,
    user_id: Principal,
    station_canister_id: CanisterId,
    request: RequestDTO,
) -> Result<Result<RequestDTO, ApiErrorDTO>, RejectResponse> {
    let get_request_args = GetRequestInput {
        request_id: request.id,
        with_full_info: Some(false),
    };
    update_candid_as::<_, (Result<GetRequestResponse, ApiErrorDTO>,)>(
        env,
        station_canister_id,
        user_id,
        "get_request",
        (get_request_args,),
    )
    .map(|resp| resp.0.map(|resp| resp.request))
}

pub fn get_request(
    env: &PocketIc,
    user_id: Principal,
    station_canister_id: CanisterId,
    request: RequestDTO,
) -> RequestDTO {
    try_get_request(env, user_id, station_canister_id, request)
        .unwrap()
        .unwrap()
}

fn is_request_completed(request: RequestDTO) -> bool {
    match request.status {
        RequestStatusDTO::Completed { .. } => true,
        RequestStatusDTO::Rejected
        | RequestStatusDTO::Cancelled { .. }
        | RequestStatusDTO::Failed { .. }
        | RequestStatusDTO::Created
        | RequestStatusDTO::Approved
        | RequestStatusDTO::Scheduled { .. }
        | RequestStatusDTO::Processing { .. } => false,
    }
}

fn is_request_evaluated(request: RequestDTO) -> bool {
    match request.status {
        RequestStatusDTO::Completed { .. }
        | RequestStatusDTO::Rejected
        | RequestStatusDTO::Cancelled { .. }
        | RequestStatusDTO::Failed { .. } => true,
        RequestStatusDTO::Created
        | RequestStatusDTO::Approved
        | RequestStatusDTO::Scheduled { .. }
        | RequestStatusDTO::Processing { .. } => false,
    }
}

pub fn submit_delayed_request_raw(
    env: &PocketIc,
    user_id: Principal,
    station_canister_id: CanisterId,
    request_operation_input: RequestOperationInput,
    delay: Duration,
) -> Result<(Result<CreateRequestResponse, ApiErrorDTO>,), RejectResponse> {
    let execution_time = env.get_time() + delay;
    let execution_time_nanos = timestamp_to_rfc3339(&execution_time.as_nanos_since_unix_epoch());

    let create_request_input = CreateRequestInput {
        operation: request_operation_input,
        title: None,
        summary: None,
        execution_plan: Some(RequestExecutionScheduleDTO::Scheduled {
            execution_time: execution_time_nanos,
        }),
        expiration_dt: None,
    };
    update_candid_as(
        env,
        station_canister_id,
        user_id,
        "create_request",
        (create_request_input,),
    )
}

pub fn submit_request_raw(
    env: &PocketIc,
    user_id: Principal,
    station_canister_id: CanisterId,
    request_operation_input: RequestOperationInput,
) -> Result<(Result<CreateRequestResponse, ApiErrorDTO>,), RejectResponse> {
    let create_request_input = CreateRequestInput {
        operation: request_operation_input,
        title: None,
        summary: None,
        execution_plan: Some(RequestExecutionScheduleDTO::Immediate),
        expiration_dt: None,
    };
    update_candid_as(
        env,
        station_canister_id,
        user_id,
        "create_request",
        (create_request_input,),
    )
}

pub fn submit_request(
    env: &PocketIc,
    user_id: Principal,
    station_canister_id: CanisterId,
    request_operation_input: RequestOperationInput,
) -> RequestDTO {
    let res = submit_request_raw(env, user_id, station_canister_id, request_operation_input);
    res.unwrap().0.unwrap().request
}

pub fn submit_request_with_expected_trap(
    env: &PocketIc,
    user_id: Principal,
    station_canister_id: CanisterId,
    request_operation_input: RequestOperationInput,
) -> String {
    submit_request_raw(env, user_id, station_canister_id, request_operation_input)
        .unwrap_err()
        .reject_message
}

pub fn wait_for_request(
    env: &PocketIc,
    user_id: Principal,
    station_canister_id: CanisterId,
    request: RequestDTO,
) -> Result<RequestDTO, Option<RequestStatusDTO>> {
    wait_for_request_with_extra_ticks(env, user_id, station_canister_id, request, 0)
}

pub fn wait_for_request_with_extra_ticks(
    env: &PocketIc,
    user_id: Principal,
    station_canister_id: CanisterId,
    request: RequestDTO,
    extra_ticks: u64,
) -> Result<RequestDTO, Option<RequestStatusDTO>> {
    // wait for the request to be approved
    env.advance_time(Duration::from_secs(2));
    env.tick();
    // wait for the request to be processing
    env.advance_time(Duration::from_secs(2));
    env.tick();
    // wait in case the request calls out to other canisters
    env.advance_time(Duration::from_secs(2));
    env.tick();

    for _ in 0..extra_ticks {
        // timer's period for processing requests is 5 seconds
        env.advance_time(Duration::from_secs(5));
        env.tick();
    }
    // wait for the request to be completed
    for _ in 0..100 {
        let new_request = get_request(env, user_id, station_canister_id, request.clone());
        if is_request_completed(new_request.clone()) {
            return Ok(new_request);
        }
        if is_request_evaluated(new_request.clone()) {
            return Err(Some(new_request.status));
        }
        // timer's period for processing requests is 5 seconds
        env.advance_time(Duration::from_secs(5));
        env.tick();
    }
    Err(None)
}

pub fn execute_request(
    env: &PocketIc,
    user_id: Principal,
    station_canister_id: CanisterId,
    request_operation_input: RequestOperationInput,
) -> Result<RequestDTO, Option<RequestStatusDTO>> {
    execute_request_with_extra_ticks(
        env,
        user_id,
        station_canister_id,
        request_operation_input,
        0,
    )
}

pub fn execute_request_with_extra_ticks(
    env: &PocketIc,
    user_id: Principal,
    station_canister_id: CanisterId,
    request_operation_input: RequestOperationInput,
    extra_ticks: u64,
) -> Result<RequestDTO, Option<RequestStatusDTO>> {
    let request = submit_request(env, user_id, station_canister_id, request_operation_input);
    wait_for_request_with_extra_ticks(env, user_id, station_canister_id, request, extra_ticks)
}

pub fn submit_request_approval(
    env: &PocketIc,
    user_id: Principal,
    station_canister_id: CanisterId,
    request: RequestDTO,
    decision: RequestApprovalStatusDTO,
) {
    let submit_request_approval_input = SubmitRequestApprovalInput {
        request_id: request.id,
        decision,
        reason: None,
    };
    let res: (Result<SubmitRequestApprovalResponse, ApiErrorDTO>,) = update_candid_as(
        env,
        station_canister_id,
        user_id,
        "submit_request_approval",
        (submit_request_approval_input,),
    )
    .unwrap();
    res.0.unwrap();
}

pub fn get_system_info(
    env: &PocketIc,
    user_id: Principal,
    station_canister_id: CanisterId,
) -> SystemInfoDTO {
    await_station_healthy(env, station_canister_id, user_id);
    let res: (ApiResult<SystemInfoResponse>,) =
        update_candid_as(env, station_canister_id, user_id, "system_info", ()).unwrap();
    res.0.unwrap().system
}

pub fn add_user(
    env: &PocketIc,
    identity: Principal,
    group_ids: Vec<String>,
    station_canister_id: Principal,
) -> UserDTO {
    add_user_with_name(
        env,
        identity.to_text().to_string(),
        identity,
        group_ids,
        station_canister_id,
    )
}

pub fn add_user_with_name(
    env: &PocketIc,
    user_name: String,
    identity: Principal,
    group_ids: Vec<String>,
    station_canister_id: Principal,
) -> UserDTO {
    let add_user = RequestOperationInput::AddUser(AddUserOperationInput {
        name: user_name,
        identities: vec![identity],
        groups: group_ids,
        status: UserStatusDTO::Active,
    });
    let add_user_request = submit_request(env, WALLET_ADMIN_USER, station_canister_id, add_user);
    let new_request = wait_for_request(
        env,
        WALLET_ADMIN_USER,
        station_canister_id,
        add_user_request,
    )
    .unwrap();
    match new_request.operation {
        RequestOperationDTO::AddUser(add_user) => add_user.user.unwrap(),
        _ => panic!("invalid request operation"),
    }
}

pub fn get_user(env: &PocketIc, user_id: Principal, station_canister_id: Principal) -> UserDTO {
    let res: (ApiResult<MeResponse>,) =
        update_candid_as(env, station_canister_id, user_id, "me", ()).unwrap();
    res.0.unwrap().me
}

pub fn canister_status(
    env: &PocketIc,
    sender: Option<Principal>,
    canister_id: Principal,
) -> CanisterStatusResult {
    env.canister_status(canister_id, sender).unwrap()
}

pub fn set_controllers(
    env: &PocketIc,
    sender: Option<Principal>,
    canister_id: Principal,
    new_controllers: Vec<Principal>,
) {
    env.set_controllers(canister_id, sender, new_controllers)
        .unwrap();
}

pub fn get_core_canister_health_status(
    env: &PocketIc,
    user_id: Principal,
    canister_id: Principal,
) -> HealthStatus {
    let res: (HealthStatus,) =
        update_candid_as(env, canister_id, user_id, "health_status", ((),)).unwrap();

    res.0
}

pub fn advance_time_to_burn_cycles(
    env: &PocketIc,
    sender: Principal,
    canister_id: Principal,
    target_cycles: u128,
) {
    if env.cycle_balance(canister_id) < target_cycles {
        return;
    }

    // Stops to prevent side effects like timers or heartbeats
    env.stop_canister(canister_id, Some(sender)).unwrap();
    let canister_cycles = env.cycle_balance(canister_id);
    let jump_secs = 10;
    let cycles_to_burn = canister_cycles.saturating_sub(target_cycles);

    // advance time one step to get an estimate of the burned cycles per advance
    env.advance_time(Duration::from_secs(jump_secs));
    env.tick();

    let burned_cycles = canister_cycles.saturating_sub(env.cycle_balance(canister_id));
    if burned_cycles == 0 {
        panic!("Canister did not burn any cycles, this should not happen.");
    }

    // advance time to burn the remaining cycles
    let advance_times_to_burn_cycles = cycles_to_burn.div_ceil(burned_cycles);
    let burn_duration = Duration::from_secs(jump_secs * advance_times_to_burn_cycles as u64);
    env.advance_time(burn_duration);
    env.tick();

    if target_cycles > 0 {
        // restart the canister if it has some cycles remaining
        env.start_canister(canister_id, Some(sender)).unwrap();
    }

    // need at least 2 ticks
    env.tick();
    env.tick();

    // adds cycles to be as close as possible to the target
    let canister_cycles = env.cycle_balance(canister_id);
    let add_cycles = target_cycles.saturating_sub(canister_cycles);
    if add_cycles > 0 {
        env.add_cycles(canister_id, add_cycles);
    }
}

pub fn update_raw(
    env: &PocketIc,
    canister_id: CanisterId,
    sender: Principal,
    method: &str,
    payload: Vec<u8>,
) -> Result<Vec<u8>, RejectResponse> {
    env.update_call(canister_id, sender, method, payload)
}

pub fn get_upgrader_disaster_recovery(
    env: &PocketIc,
    upgrader_id: &Principal,
    station_canister_id: &Principal,
) -> upgrader_api::GetDisasterRecoveryStateResponse {
    let res: (ApiResult<GetDisasterRecoveryStateResponse>,) = query_candid_as(
        env,
        upgrader_id.to_owned(),
        station_canister_id.to_owned(),
        "get_disaster_recovery_state",
        ((),),
    )
    .expect("Failed query call to get disaster recovery state");

    res.0.expect("Failed to get disaster recovery state")
}

pub fn set_disaster_recovery(
    env: &PocketIc,
    station_canister_id: Principal,
    input: SetDisasterRecoveryOperationInput,
) -> SetDisasterRecoveryOperationDTO {
    let request = RequestOperationInput::SetDisasterRecovery(input);
    let request_response = submit_request(env, WALLET_ADMIN_USER, station_canister_id, request);
    let new_request = wait_for_request(
        env,
        WALLET_ADMIN_USER,
        station_canister_id,
        request_response,
    )
    .unwrap();
    match new_request.operation {
        RequestOperationDTO::SetDisasterRecovery(response) => *response,
        _ => panic!("invalid request operation"),
    }
}

pub fn get_upgrader_logs(
    env: &PocketIc,
    upgrader_id: &Principal,
    sender: &Principal,
) -> GetLogsResponse {
    let res: (ApiResult<GetLogsResponse>,) = query_candid_as(
        env,
        upgrader_id.to_owned(),
        *sender,
        "get_logs",
        (GetLogsInput { pagination: None },),
    )
    .expect("Failed query call to get disaster recovery logs");

    res.0.expect("Failed to get disaster recovery logs")
}

pub fn get_all_upgrader_logs(
    env: &PocketIc,
    upgrader_id: &Principal,
    sender: &Principal,
) -> Vec<LogEntry> {
    let pagination = PaginationInput {
        offset: Some(0),
        limit: Some(100),
    };
    let res: (ApiResult<GetLogsResponse>,) = query_candid_as(
        env,
        upgrader_id.to_owned(),
        *sender,
        "get_logs",
        (GetLogsInput {
            pagination: Some(pagination),
        },),
    )
    .expect("Failed query call to get disaster recovery logs");

    let logs = res.0.expect("Failed to get disaster recovery logs");
    assert_eq!(logs.logs.len(), logs.total as usize);
    assert!(logs.next_offset.is_none());
    logs.logs
}

pub fn get_account_read_permission(
    env: &PocketIc,
    sender: Principal,
    station_canister_id: Principal,
    account_id: String,
) -> AllowDTO {
    let res: (ApiResult<GetPermissionResponse>,) = update_candid_as(
        env,
        station_canister_id,
        sender,
        "get_permission",
        (station_api::GetPermissionInput {
            resource: station_api::ResourceDTO::Account(
                station_api::AccountResourceActionDTO::Read(ResourceIdDTO::Id(account_id)),
            ),
        },),
    )
    .expect("Failed to get account read permission");

    res.0
        .expect("Failed to get account read permission")
        .permission
        .allow
}

pub fn get_account_update_permission(
    env: &PocketIc,
    sender: Principal,
    station_canister_id: Principal,
    account_id: String,
) -> AllowDTO {
    let res: (ApiResult<GetPermissionResponse>,) = update_candid_as(
        env,
        station_canister_id,
        sender,
        "get_permission",
        (station_api::GetPermissionInput {
            resource: station_api::ResourceDTO::Account(
                station_api::AccountResourceActionDTO::Update(ResourceIdDTO::Id(account_id)),
            ),
        },),
    )
    .expect("Failed to get account update permission");

    res.0
        .expect("Failed to get account update permission")
        .permission
        .allow
}

pub fn get_account_transfer_permission(
    env: &PocketIc,
    sender: Principal,
    station_canister_id: Principal,
    account_id: String,
) -> AllowDTO {
    let res: (ApiResult<GetPermissionResponse>,) = update_candid_as(
        env,
        station_canister_id,
        sender,
        "get_permission",
        (station_api::GetPermissionInput {
            resource: station_api::ResourceDTO::Account(
                station_api::AccountResourceActionDTO::Transfer(ResourceIdDTO::Id(account_id)),
            ),
        },),
    )
    .expect("Failed to get account transfer permission");

    res.0
        .expect("Failed to get account transfer permission")
        .permission
        .allow
}

pub fn create_icp_account(env: &PocketIc, station_id: Principal, user_id: UuidDTO) -> AccountDTO {
    let icp = get_icp_asset(env, station_id, WALLET_ADMIN_USER);

    // create account
    let create_account_args = AddAccountOperationInput {
        name: "test".to_string(),
        assets: vec![icp.id.clone()],
        read_permission: AllowDTO {
            auth_scope: station_api::AuthScopeDTO::Restricted,
            user_groups: vec![],
            users: vec![user_id.clone()],
        },
        configs_permission: AllowDTO {
            auth_scope: station_api::AuthScopeDTO::Restricted,
            user_groups: vec![],
            users: vec![user_id.clone()],
        },
        transfer_permission: AllowDTO {
            auth_scope: station_api::AuthScopeDTO::Restricted,
            user_groups: vec![],
            users: vec![user_id.clone()],
        },
        transfer_request_policy: Some(RequestPolicyRuleDTO::QuorumPercentage(
            QuorumPercentageDTO {
                approvers: UserSpecifierDTO::Id(vec![user_id.clone()]),
                min_approved: 100,
            },
        )),
        configs_request_policy: Some(RequestPolicyRuleDTO::QuorumPercentage(
            QuorumPercentageDTO {
                approvers: UserSpecifierDTO::Id(vec![user_id.clone()]),
                min_approved: 100,
            },
        )),
        metadata: vec![],
    };

    create_account(env, station_id, WALLET_ADMIN_USER, create_account_args)
}

pub fn create_account(
    env: &PocketIc,
    station_id: Principal,
    requester: Principal,
    input: AddAccountOperationInput,
) -> AccountDTO {
    let add_account_request = CreateRequestInput {
        operation: RequestOperationInput::AddAccount(input),
        title: None,
        summary: None,
        execution_plan: Some(RequestExecutionScheduleDTO::Immediate),
        expiration_dt: None,
    };
    let res: (ApiResult<CreateRequestResponse>,) = update_candid_as(
        env,
        station_id,
        requester,
        "create_request",
        (add_account_request,),
    )
    .unwrap();

    // wait for the request to be approved (timer's period is 5 seconds)
    env.advance_time(Duration::from_secs(5));
    env.tick();

    let account_creation_request_dto = res.0.unwrap().request;
    match account_creation_request_dto.status {
        RequestStatusDTO::Approved => {}
        _ => {
            panic!("request must be approved by now");
        }
    };

    // wait for the request to be executed (timer's period is 5 seconds)
    env.advance_time(Duration::from_secs(5));
    env.tick();

    // fetch the created account id from the request
    let get_request_args = GetRequestInput {
        request_id: account_creation_request_dto.id,
        with_full_info: Some(false),
    };
    let res: (ApiResult<CreateRequestResponse>,) = update_candid_as(
        env,
        station_id,
        requester,
        "get_request",
        (get_request_args,),
    )
    .unwrap();
    let finalized_request = res.0.unwrap().request;
    match finalized_request.status {
        RequestStatusDTO::Completed { .. } => {}
        _ => {
            panic!(
                "request must be completed by now but instead is {:?}",
                finalized_request.status
            );
        }
    };

    match finalized_request.operation {
        RequestOperationDTO::AddAccount(add_account) => {
            add_account.account.expect("no account in result")
        }
        _ => {
            panic!("request must be AddAccount");
        }
    }
}

pub fn create_transfer(
    env: &PocketIc,
    station_id: Principal,
    requester: Principal,
    input: station_api::TransferOperationInput,
) -> station_api::TransferDTO {
    // make transfer request to beneficiary

    let transfer_request = CreateRequestInput {
        operation: RequestOperationInput::Transfer(input),
        title: None,
        summary: None,
        expiration_dt: None,
        execution_plan: Some(RequestExecutionScheduleDTO::Immediate),
    };
    let res: (Result<CreateRequestResponse, ApiErrorDTO>,) = update_candid_as(
        env,
        station_id,
        requester,
        "create_request",
        (transfer_request,),
    )
    .unwrap();
    let request_dto = res.0.unwrap().request;

    // wait for the request to be approved (timer's period is 5 seconds)
    env.advance_time(Duration::from_secs(5));
    env.tick();
    // wait for the request to be processing (timer's period is 5 seconds) and first is set to processing
    env.advance_time(Duration::from_secs(5));
    env.tick();
    env.tick();
    env.tick();
    env.advance_time(Duration::from_secs(5));
    env.tick();
    env.tick();
    env.tick();

    // check transfer request status
    let get_request_args = GetRequestInput {
        request_id: request_dto.id.clone(),
        with_full_info: Some(false),
    };
    let res: (Result<GetRequestResponse, ApiErrorDTO>,) = update_candid_as(
        env,
        station_id,
        requester,
        "get_request",
        (get_request_args,),
    )
    .unwrap();
    let new_request_dto = res.0.unwrap().request;
    match new_request_dto.status {
        RequestStatusDTO::Completed { .. } => {}
        _ => {
            panic!(
                "request must be completed by now but instead is {:?}",
                new_request_dto.status
            );
        }
    };

    // request has the transfer id filled out
    let transfer_id = match new_request_dto.operation {
        RequestOperationDTO::Transfer(transfer) => transfer
            .transfer_id
            .expect("transfer id must be set for completed transfer"),
        _ => {
            panic!("request must be Transfer");
        }
    };

    // fetch the transfer and check if its request id matches the request id that created it
    let res: (Result<GetTransfersResponse, ApiErrorDTO>,) = query_candid_as(
        env,
        station_id,
        requester,
        "get_transfers",
        (GetTransfersInput {
            transfer_ids: vec![transfer_id],
        },),
    )
    .expect("Failed to send query call");

    res.0
        .expect("Failed to get transfers")
        .transfers
        .first()
        .expect("no transfer in result")
        .clone()
}

pub fn fetch_account_balances(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
    input: FetchAccountBalancesInput,
) -> station_api::FetchAccountBalancesResponse {
    update_candid_as::<(FetchAccountBalancesInput,), (ApiResult<FetchAccountBalancesResponse>,)>(
        env,
        station_canister_id,
        requester,
        "fetch_account_balances",
        (input,),
    )
    .expect("Failed to send query call")
    .0
    .expect("Failed to get account balances")
}

pub fn get_icp_asset(
    env: &PocketIc,
    station_canister_id: Principal,
    requester: Principal,
) -> station_api::AssetDTO {
    list_assets(env, station_canister_id, requester)
        .expect("Failed to query list_assets")
        .0
        .expect("Failed to list assets")
        .assets
        .into_iter()
        .find(|asset| asset.symbol == "ICP")
        .expect("Failed to find ICP asset")
}

pub fn get_icp_account_identifier(addresses: &[station_api::AccountAddressDTO]) -> Option<String> {
    addresses
        .iter()
        .find(|a| a.format == "icp_account_identifier")
        .map(|a| a.address.clone())
}

/// Compresses the given data to a gzip format.
pub fn compress_to_gzip(data: &[u8]) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(data).expect("Failed to write data");
    encoder.finish().expect("Failed to finish compression")
}

/// Creates a file in the `assets` folder with the given name and content.
pub fn create_file(name: &str, content: &[u8]) {
    let relative_path = std::path::Path::new("assets").join(name);
    let absolute_path = test_dir().join(relative_path);

    if let Some(parent_dir) = absolute_path.parent() {
        std::fs::create_dir_all(parent_dir).expect("Failed to create directories");
    }

    std::fs::write(&absolute_path, content).expect("Failed to write file");
}

/// Reads the content of a file in the `assets` folder with the given name.
pub fn read_file(name: &str) -> Option<Vec<u8>> {
    let relative_path = std::path::Path::new("assets").join(name);
    let absolute_path = test_dir().join(relative_path);

    if !absolute_path.exists() {
        return None;
    }

    std::fs::read(absolute_path).ok()
}

fn test_dir() -> PathBuf {
    PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("Failed to get CARGO_MANIFEST_DIR"))
}

/// Converts the given data to a SHA-256 hash and returns it as a hex string.
pub fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = sha2::Sha256::new();

    hasher.update(data);

    let result = hasher.finalize();

    hex::encode(result)
}

pub fn upload_canister_modules(env: &PocketIc, control_panel_id: Principal, controller: Principal) {
    // upload upgrader
    let upgrader_wasm = get_canister_wasm("upgrader").to_vec();
    let upload_canister_modules_args = UploadCanisterModulesInput {
        upgrader_wasm_module: Some(upgrader_wasm.to_owned()),
        station_wasm_module: None,
        station_wasm_module_extra_chunks: None,
    };
    let res: (ApiResult<()>,) = update_candid_as(
        env,
        control_panel_id,
        controller,
        "upload_canister_modules",
        (upload_canister_modules_args.clone(),),
    )
    .unwrap();
    res.0.unwrap();

    // upload station

    let station_wasm = get_canister_wasm("station");
    let (base_chunk, module_extra_chunks) =
        upload_canister_chunks_to_asset_canister(env, station_wasm, 500_000);
    let upload_canister_modules_args = UploadCanisterModulesInput {
        upgrader_wasm_module: None,
        station_wasm_module: Some(base_chunk),
        station_wasm_module_extra_chunks: Some(Some(module_extra_chunks)),
    };
    let res: (ApiResult<()>,) = update_candid_as(
        env,
        control_panel_id,
        controller,
        "upload_canister_modules",
        (upload_canister_modules_args.clone(),),
    )
    .unwrap();
    res.0.unwrap();
}

pub fn bump_time_to_avoid_ratelimit(env: &PocketIc) {
    // the rate limiter aggregation window is 300s and resolution is 10s
    env.advance_time(Duration::from_secs(300 + 10));
}

#[derive(CandidType)]
struct StoreArg {
    pub key: String,
    pub content: Vec<u8>,
    pub content_type: String,
    pub content_encoding: String,
    pub sha256: Option<Vec<u8>>,
}

pub(crate) fn hash(data: &Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

pub fn upload_canister_chunks_to_asset_canister(
    env: &PocketIc,
    canister_wasm: Vec<u8>,
    chunk_len: usize,
) -> (Vec<u8>, WasmModuleExtraChunks) {
    // create and install the asset canister
    let asset_canister_id = create_canister(env, Principal::anonymous());
    env.install_canister(
        asset_canister_id,
        get_canister_wasm("assetstorage"),
        Encode!(&()).unwrap(),
        None,
    );

    // get canister wasm hash
    let canister_wasm_hash = hash(&canister_wasm);

    // chunk canister
    let mut chunks = canister_wasm.chunks(chunk_len);
    let base_chunk: &[u8] = chunks.next().unwrap();
    assert!(!base_chunk.is_empty());
    let chunks: Vec<&[u8]> = chunks.collect();
    assert!(chunks.len() >= 2);

    // upload chunks to asset canister
    let batch_id = update_candid_as::<_, (CreateBatchResponse,)>(
        env,
        asset_canister_id,
        Principal::anonymous(),
        "create_batch",
        ((),),
    )
    .unwrap()
    .0
    .batch_id;
    let extra_chunks_key = "/extra_chunks".to_string();
    let create_asset = CreateAssetArguments {
        key: extra_chunks_key.clone(),
        content_type: "application/octet-stream".to_string(),
        max_age: None,
        headers: None,
        enable_aliasing: None,
        allow_raw_access: None,
    };
    let mut chunk_ids = vec![];
    for chunk in chunks {
        let create_chunk_arg = CreateChunkArg {
            batch_id: batch_id.clone(),
            content: chunk.to_vec().into(),
        };
        let create_chunk_response = update_candid_as::<_, (CreateChunkResponse,)>(
            env,
            asset_canister_id,
            Principal::anonymous(),
            "create_chunk",
            (create_chunk_arg,),
        )
        .unwrap()
        .0;
        chunk_ids.push(create_chunk_response.chunk_id);
    }
    let set_asset_content = SetAssetContentArguments {
        key: extra_chunks_key.clone(),
        content_encoding: "identity".to_string(),
        chunk_ids,
        sha256: None,
        last_chunk: None,
    };
    let operations = vec![
        BatchOperation::CreateAsset(create_asset),
        BatchOperation::SetAssetContent(set_asset_content),
    ];
    let commit_batch_args: CommitBatchArguments = CommitBatchArguments {
        batch_id,
        operations,
    };
    update_candid_as::<_, ((),)>(
        env,
        asset_canister_id,
        Principal::anonymous(),
        "commit_batch",
        (commit_batch_args,),
    )
    .unwrap();

    let module_extra_chunks = WasmModuleExtraChunks {
        store_canister: asset_canister_id,
        extra_chunks_key,
        wasm_module_hash: canister_wasm_hash,
    };

    (base_chunk.to_vec(), module_extra_chunks)
}

pub(crate) fn await_station_healthy(env: &PocketIc, station_id: Principal, user_id: Principal) {
    let max_rounds = 100;
    for _ in 0..max_rounds {
        env.tick();
        let res: (station_api::HealthStatus,) =
            query_candid_as(env, station_id, user_id, "health_status", ())
                .expect("Unexpected error calling Station health_status");
        if res.0 == station_api::HealthStatus::Healthy {
            return;
        }
    }
    panic!(
        "Station did not become healthy within {} rounds.",
        max_rounds
    );
}

pub(crate) fn add_external_canister_call_any_method_permission_and_approval(
    env: &PocketIc,
    station_id: Principal,
    admin_id: Principal,
    quorum: station_api::QuorumDTO,
) {
    // add the permissions for admins to call any external canister
    execute_request(
        env,
        admin_id,
        station_id,
        RequestOperationInput::EditPermission(station_api::EditPermissionOperationInput {
            auth_scope: Some(station_api::AuthScopeDTO::Authenticated),
            users: None,
            user_groups: None,
            resource: station_api::ResourceDTO::ExternalCanister(
                station_api::ExternalCanisterResourceActionDTO::Call(
                    station_api::CallExternalCanisterResourceTargetDTO {
                        execution_method: station_api::ExecutionMethodResourceTargetDTO::Any,
                        validation_method: station_api::ValidationMethodResourceTargetDTO::No,
                    },
                ),
            ),
        }),
    )
    .expect("Failed to add permission to call external canister");

    // automatically approve calls to external canisters
    execute_request(
        env,
        admin_id,
        station_id,
        RequestOperationInput::AddRequestPolicy(station_api::AddRequestPolicyOperationInput {
            specifier: station_api::RequestSpecifierDTO::CallExternalCanister(
                station_api::CallExternalCanisterResourceTargetDTO {
                    execution_method: station_api::ExecutionMethodResourceTargetDTO::Any,
                    validation_method: station_api::ValidationMethodResourceTargetDTO::No,
                },
            ),
            rule: station_api::RequestPolicyRuleDTO::Quorum(quorum),
        }),
    )
    .expect("Failed to add approval policy to call external canister");
}

pub(crate) fn deploy_test_canister(env: &PocketIc, controller: Principal) -> Principal {
    let test_canister = create_canister(env, controller);
    let test_canister_wasm = get_canister_wasm("test_canister");
    env.install_canister(test_canister, test_canister_wasm, vec![], Some(controller));
    test_canister
}

pub fn expect_await_call_result<T>(result: Vec<u8>) -> T
where
    T: for<'a> ArgumentDecoder<'a>,
{
    decode_args(&result).expect("Failed to decode result")
}

pub(crate) fn set_disaster_recovery_committee(
    env: &PocketIc,
    upgrader_id: Principal,
    station_id: Principal,
    committee: upgrader_api::DisasterRecoveryCommittee,
) -> ApiResult {
    let args = upgrader_api::SetDisasterRecoveryCommitteeInput { committee };
    let res: (ApiResult,) = update_candid_as(
        env,
        upgrader_id,
        station_id,
        "set_disaster_recovery_committee",
        (args,),
    )
    .expect("Failed update call to set disaster recovery committee");
    res.0
}

pub(crate) fn get_disaster_recovery_committee(
    env: &PocketIc,
    upgrader_id: Principal,
    station_id: Principal,
) -> Option<upgrader_api::DisasterRecoveryCommittee> {
    let res: (ApiResult<upgrader_api::GetDisasterRecoveryCommitteeResponse>,) = query_candid_as(
        env,
        upgrader_id,
        station_id,
        "get_disaster_recovery_committee",
        ((),),
    )
    .expect("Failed query call to get disaster recovery committee");
    res.0.unwrap().committee
}

pub(crate) fn set_disaster_recovery_accounts(
    env: &PocketIc,
    upgrader_id: Principal,
    station_id: Principal,
    accounts: Vec<upgrader_api::Account>,
) -> ApiResult {
    let args = upgrader_api::SetDisasterRecoveryAccountsInput { accounts };
    let res: (ApiResult,) = update_candid_as(
        env,
        upgrader_id,
        station_id,
        "set_disaster_recovery_accounts",
        (args,),
    )
    .expect("Failed update call to set disaster recovery accounts");
    res.0
}

pub(crate) fn get_disaster_recovery_accounts(
    env: &PocketIc,
    upgrader_id: Principal,
    station_id: Principal,
) -> Vec<upgrader_api::Account> {
    let res: (ApiResult<upgrader_api::GetDisasterRecoveryAccountsResponse>,) = query_candid_as(
        env,
        upgrader_id,
        station_id,
        "get_disaster_recovery_accounts",
        ((),),
    )
    .expect("Failed query call to get disaster recovery accounts");
    res.0.unwrap().accounts
}

pub(crate) fn set_disaster_recovery_accounts_and_assets(
    env: &PocketIc,
    upgrader_id: Principal,
    station_id: Principal,
    accounts: Vec<upgrader_api::MultiAssetAccount>,
    assets: Vec<upgrader_api::Asset>,
) -> ApiResult {
    let args = upgrader_api::SetDisasterRecoveryAccountsAndAssetsInput { accounts, assets };
    let res: (ApiResult,) = update_candid_as(
        env,
        upgrader_id,
        station_id,
        "set_disaster_recovery_accounts_and_assets",
        (args,),
    )
    .expect("Failed update call to set disaster recovery accounts and assets");
    res.0
}

pub(crate) fn get_disaster_recovery_accounts_and_assets(
    env: &PocketIc,
    upgrader_id: Principal,
    station_id: Principal,
) -> (
    Vec<upgrader_api::MultiAssetAccount>,
    Vec<upgrader_api::Asset>,
) {
    let res: (ApiResult<upgrader_api::GetDisasterRecoveryAccountsAndAssetsResponse>,) =
        query_candid_as(
            env,
            upgrader_id,
            station_id,
            "get_disaster_recovery_accounts_and_assets",
            ((),),
        )
        .expect("Failed query call to get disaster recovery accounts and assets");
    let resp = res.0.unwrap();
    (resp.accounts, resp.assets)
}

pub(crate) fn request_disaster_recovery(
    env: &PocketIc,
    upgrader_id: Principal,
    user: Principal,
    request: upgrader_api::RequestDisasterRecoveryInput,
) -> ApiResult {
    let res: (ApiResult<()>,) = update_candid_as(
        env,
        upgrader_id,
        user,
        "request_disaster_recovery",
        (request,),
    )
    .expect("Failed update call to request disaster recovery");
    res.0
}

pub(crate) fn get_disaster_recovery_state(
    env: &PocketIc,
    upgrader_id: Principal,
    user: Principal,
) -> upgrader_api::GetDisasterRecoveryStateResponse {
    let res: (ApiResult<upgrader_api::GetDisasterRecoveryStateResponse>,) =
        query_candid_as(env, upgrader_id, user, "get_disaster_recovery_state", ((),))
            .expect("Failed query call to get disaster recovery state");
    res.0.unwrap()
}

pub(crate) fn is_committee_member(
    env: &PocketIc,
    upgrader_id: Principal,
    user: Principal,
) -> ApiResult<bool> {
    let res: (ApiResult<upgrader_api::IsCommitteeMemberResponse>,) =
        query_candid_as(env, upgrader_id, user, "is_committee_member", ((),))
            .expect("Failed query call to check committee membership");
    res.0.map(|resp| resp.is_committee_member)
}

pub(crate) fn upgrade_station(
    env: &PocketIc,
    user: Principal,
    station_id: Principal,
    name: Option<String>,
) {
    // upload chunks to asset canister
    let station_wasm = get_canister_wasm("station").to_vec();
    let (base_chunk, module_extra_chunks) =
        upload_canister_chunks_to_asset_canister(env, station_wasm, 200_000);

    // create system upgrade request from chunks
    let station_init_arg = SystemInstall::Upgrade(SystemUpgrade { name });
    let station_init_arg_bytes = Encode!(&station_init_arg).unwrap();
    let system_upgrade_operation =
        RequestOperationInput::SystemUpgrade(SystemUpgradeOperationInput {
            target: SystemUpgradeTargetDTO::UpgradeStation,
            module: base_chunk,
            module_extra_chunks: Some(module_extra_chunks),
            arg: Some(station_init_arg_bytes),
            take_backup_snapshot: None,
        });

    execute_request_with_extra_ticks(
        env,
        user,
        station_id,
        system_upgrade_operation,
        STATION_UPGRADE_EXTRA_TICKS,
    )
    .unwrap();
}
