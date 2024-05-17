use crate::setup::WALLET_ADMIN_USER;
use candid::Principal;
use ic_cdk::api::management_canister::main::CanisterStatusResponse;
use orbit_essentials::api::ApiResult;
use orbit_essentials::cdk::api::management_canister::main::CanisterId;
use pocket_ic::{update_candid_as, PocketIc, UserError, WasmResult};
use station_api::{
    AddUserOperationInput, ApiErrorDTO, CreateRequestInput, CreateRequestResponse, GetRequestInput,
    GetRequestResponse, HealthStatus, MeResponse, RequestApprovalStatusDTO, RequestDTO,
    RequestExecutionScheduleDTO, RequestOperationDTO, RequestOperationInput, RequestStatusDTO,
    SubmitRequestApprovalInput, SubmitRequestApprovalResponse, SystemInfoDTO, SystemInfoResponse,
    UserDTO, UserStatusDTO,
};
use std::time::Duration;

pub const NNS_ROOT_CANISTER_ID: Principal = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 3, 1, 1]);

pub const COUNTER_WAT: &str = r#"
    (module
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
            (drop (call $ic0_stable_grow (i32.const 1)))
            (call $msg_reply))
        (func $inc
            (call $ic0_stable_read (i32.const 0) (i32.const 0) (i32.const 4))
            (i32.store
                (i32.const 0)
                (i32.add (i32.load (i32.const 0)) (i32.const 2)))
            (call $ic0_stable_write (i32.const 0) (i32.const 0) (i32.const 4))
            (call $msg_reply))
        (func $read
            (call $ic0_stable_read (i32.const 0) (i32.const 0) (i32.const 4))
            (call $msg_reply_data_append
                (i32.const 0) ;; the counter from heap[0]
                (i32.const 4)) ;; length
            (call $msg_reply))
        (memory $memory 1)
        (export "canister_query read" (func $read))
        (export "canister_update inc" (func $inc))
        (export "canister_update init" (func $init))
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

pub fn get_request(
    env: &PocketIc,
    user_id: Principal,
    station_canister_id: CanisterId,
    request: RequestDTO,
) -> RequestDTO {
    let get_request_args = GetRequestInput {
        request_id: request.id,
    };
    let res: (Result<GetRequestResponse, ApiErrorDTO>,) = update_candid_as(
        env,
        station_canister_id,
        user_id,
        "get_request",
        (get_request_args,),
    )
    .unwrap();
    res.0.unwrap().request
}

fn is_request_completed(request: RequestDTO) -> bool {
    match request.status {
        RequestStatusDTO::Completed { .. } => true,
        RequestStatusDTO::Rejected { .. }
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
        | RequestStatusDTO::Rejected { .. }
        | RequestStatusDTO::Cancelled { .. }
        | RequestStatusDTO::Failed { .. } => true,
        RequestStatusDTO::Created
        | RequestStatusDTO::Approved
        | RequestStatusDTO::Scheduled { .. }
        | RequestStatusDTO::Processing { .. } => false,
    }
}

pub fn submit_request(
    env: &PocketIc,
    user_id: Principal,
    station_canister_id: CanisterId,
    request_operation_input: RequestOperationInput,
) -> RequestDTO {
    let create_request_input = CreateRequestInput {
        operation: request_operation_input,
        title: None,
        summary: None,
        execution_plan: Some(RequestExecutionScheduleDTO::Immediate),
    };
    let res: (Result<CreateRequestResponse, ApiErrorDTO>,) = update_candid_as(
        env,
        station_canister_id,
        user_id,
        "create_request",
        (create_request_input,),
    )
    .unwrap();
    res.0.unwrap().request
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
    // wait for the request to be approved (timer's period is 5 seconds)
    env.advance_time(Duration::from_secs(5));
    env.tick();
    // wait for the request to be processing (timer's period is 5 seconds)
    env.advance_time(Duration::from_secs(5));
    env.tick();
    for _ in 0..extra_ticks {
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
    let res: (ApiResult<SystemInfoResponse>,) =
        update_candid_as(env, station_canister_id, user_id, "system_info", ()).unwrap();
    res.0.unwrap().system
}

pub fn add_user(
    env: &PocketIc,
    user_id: Principal,
    group_ids: Vec<String>,
    station_canister_id: Principal,
) -> UserDTO {
    let add_user = RequestOperationInput::AddUser(AddUserOperationInput {
        name: user_id.to_text().to_string(),
        identities: vec![user_id],
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
) -> CanisterStatusResponse {
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
    let advance_times_to_burn_cycles = (cycles_to_burn + burned_cycles - 1) / burned_cycles;
    let burn_duration = Duration::from_secs(jump_secs * advance_times_to_burn_cycles as u64);
    env.advance_time(burn_duration);
    env.tick();

    // restart the canister
    env.start_canister(canister_id, Some(sender)).unwrap();
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
) -> Result<Vec<u8>, UserError> {
    env.update_call(canister_id, sender, method, payload)
        .map(|res| match res {
            WasmResult::Reply(bytes) => bytes,
            WasmResult::Reject(message) => panic!("Unexpected reject: {}", message),
        })
}
