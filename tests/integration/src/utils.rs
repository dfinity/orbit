use crate::setup::WALLET_ADMIN_USER;
use candid::Principal;
use ic_cdk::api::management_canister::main::{
    CanisterIdRecord, CanisterSettings, CanisterStatusResponse, UpdateSettingsArgument,
};
use orbit_essentials::api::ApiResult;
use orbit_essentials::cdk::api::management_canister::main::CanisterId;
use pocket_ic::{update_candid_as, PocketIc};
use station_api::{
    AddUserOperationInput, ApiErrorDTO, CreateProposalInput, CreateProposalResponse,
    GetProposalInput, GetProposalResponse, HealthStatus, MeResponse, ProposalDTO,
    ProposalExecutionScheduleDTO, ProposalOperationDTO, ProposalOperationInput, ProposalStatusDTO,
    SystemInfoDTO, SystemInfoResponse, UserDTO, UserStatusDTO, VoteOnProposalInput,
    VoteOnProposalResponse,
};
use std::time::Duration;

pub const NNS_ROOT_CANISTER_ID: Principal = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 3, 1, 1]);

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

pub fn get_proposal(
    env: &PocketIc,
    user_id: Principal,
    station_canister_id: CanisterId,
    proposal: ProposalDTO,
) -> ProposalDTO {
    let get_proposal_args = GetProposalInput {
        proposal_id: proposal.id,
    };
    let res: (Result<GetProposalResponse, ApiErrorDTO>,) = update_candid_as(
        env,
        station_canister_id,
        user_id,
        "get_proposal",
        (get_proposal_args,),
    )
    .unwrap();
    res.0.unwrap().proposal
}

fn is_proposal_completed(proposal: ProposalDTO) -> bool {
    match proposal.status {
        ProposalStatusDTO::Completed { .. } => true,
        ProposalStatusDTO::Rejected { .. }
        | ProposalStatusDTO::Cancelled { .. }
        | ProposalStatusDTO::Failed { .. }
        | ProposalStatusDTO::Created
        | ProposalStatusDTO::Adopted
        | ProposalStatusDTO::Scheduled { .. }
        | ProposalStatusDTO::Processing { .. } => false,
    }
}

fn is_proposal_evaluated(proposal: ProposalDTO) -> bool {
    match proposal.status {
        ProposalStatusDTO::Completed { .. }
        | ProposalStatusDTO::Rejected { .. }
        | ProposalStatusDTO::Cancelled { .. }
        | ProposalStatusDTO::Failed { .. } => true,
        ProposalStatusDTO::Created
        | ProposalStatusDTO::Adopted
        | ProposalStatusDTO::Scheduled { .. }
        | ProposalStatusDTO::Processing { .. } => false,
    }
}

pub fn submit_proposal(
    env: &PocketIc,
    user_id: Principal,
    station_canister_id: CanisterId,
    proposal_operation_input: ProposalOperationInput,
) -> ProposalDTO {
    let create_proposal_input = CreateProposalInput {
        operation: proposal_operation_input,
        title: None,
        summary: None,
        execution_plan: Some(ProposalExecutionScheduleDTO::Immediate),
    };
    let res: (Result<CreateProposalResponse, ApiErrorDTO>,) = update_candid_as(
        env,
        station_canister_id,
        user_id,
        "create_proposal",
        (create_proposal_input,),
    )
    .unwrap();
    res.0.unwrap().proposal
}

pub fn wait_for_proposal(
    env: &PocketIc,
    user_id: Principal,
    station_canister_id: CanisterId,
    proposal: ProposalDTO,
) -> Result<ProposalDTO, Option<ProposalStatusDTO>> {
    wait_for_proposal_with_extra_ticks(env, user_id, station_canister_id, proposal, 0)
}

pub fn wait_for_proposal_with_extra_ticks(
    env: &PocketIc,
    user_id: Principal,
    station_canister_id: CanisterId,
    proposal: ProposalDTO,
    extra_ticks: u64,
) -> Result<ProposalDTO, Option<ProposalStatusDTO>> {
    // wait for the proposal to be adopted (timer's period is 5 seconds)
    env.advance_time(Duration::from_secs(5));
    env.tick();
    // wait for the proposal to be processing (timer's period is 5 seconds)
    env.advance_time(Duration::from_secs(5));
    env.tick();
    for _ in 0..extra_ticks {
        env.tick();
    }
    // wait for the proposal to be completed
    for _ in 0..100 {
        let new_proposal = get_proposal(env, user_id, station_canister_id, proposal.clone());
        if is_proposal_completed(new_proposal.clone()) {
            return Ok(new_proposal);
        }
        if is_proposal_evaluated(new_proposal.clone()) {
            return Err(Some(new_proposal.status));
        }
    }
    Err(None)
}

pub fn execute_proposal(
    env: &PocketIc,
    user_id: Principal,
    station_canister_id: CanisterId,
    proposal_operation_input: ProposalOperationInput,
) -> Result<ProposalDTO, Option<ProposalStatusDTO>> {
    execute_proposal_with_extra_ticks(
        env,
        user_id,
        station_canister_id,
        proposal_operation_input,
        0,
    )
}

pub fn execute_proposal_with_extra_ticks(
    env: &PocketIc,
    user_id: Principal,
    station_canister_id: CanisterId,
    proposal_operation_input: ProposalOperationInput,
    extra_ticks: u64,
) -> Result<ProposalDTO, Option<ProposalStatusDTO>> {
    let proposal = submit_proposal(env, user_id, station_canister_id, proposal_operation_input);
    wait_for_proposal_with_extra_ticks(env, user_id, station_canister_id, proposal, extra_ticks)
}

pub fn vote_on_proposal(
    env: &PocketIc,
    user_id: Principal,
    station_canister_id: CanisterId,
    proposal: ProposalDTO,
    approve: bool,
) {
    let vote_on_proposal_input = VoteOnProposalInput {
        proposal_id: proposal.id,
        approve,
        reason: None,
    };
    let res: (Result<VoteOnProposalResponse, ApiErrorDTO>,) = update_candid_as(
        env,
        station_canister_id,
        user_id,
        "vote_on_proposal",
        (vote_on_proposal_input,),
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
    let add_user = ProposalOperationInput::AddUser(AddUserOperationInput {
        name: None,
        identities: vec![user_id],
        groups: group_ids,
        status: UserStatusDTO::Active,
    });
    let add_user_proposal = submit_proposal(env, WALLET_ADMIN_USER, station_canister_id, add_user);
    let new_proposal = wait_for_proposal(
        env,
        WALLET_ADMIN_USER,
        station_canister_id,
        add_user_proposal,
    )
    .unwrap();
    match new_proposal.operation {
        ProposalOperationDTO::AddUser(add_user) => add_user.user.unwrap(),
        _ => panic!("invalid proposal operation"),
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
    let args = CanisterIdRecord { canister_id };

    let res: (CanisterStatusResponse,) = update_candid_as(
        env,
        Principal::management_canister(),
        sender.unwrap_or(Principal::anonymous()),
        "canister_status",
        (args,),
    )
    .unwrap();
    res.0
}

pub fn update_canister_settings(
    env: &PocketIc,
    sender: Option<Principal>,
    canister_id: Principal,
    settings: CanisterSettings,
) {
    let args = UpdateSettingsArgument {
        settings,
        canister_id,
    };

    // the type () is required here due to rust not being able to infer the type of the return automatically
    let _: () = update_candid_as(
        env,
        Principal::management_canister(),
        sender.unwrap_or(Principal::anonymous()),
        "update_settings",
        (args,),
    )
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
    // wait for the canister to start
    for _ in 0..2 {
        env.tick();
    }

    // adds cycles to be as close as possible to the target
    let canister_cycles = env.cycle_balance(canister_id);
    let add_cycles = target_cycles.saturating_sub(canister_cycles);
    if add_cycles > 0 {
        env.add_cycles(canister_id, add_cycles);
    }
}
