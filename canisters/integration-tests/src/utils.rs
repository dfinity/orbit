use crate::setup::WALLET_ADMIN_USER;
use candid::utils::{ArgumentDecoder, ArgumentEncoder};
use candid::Principal;
use ic_canister_core::api::ApiResult;
use ic_canister_core::cdk::api::management_canister::main::CanisterId;
use ic_cdk::api::management_canister::main::{
    CanisterIdRecord, CanisterSettings, CanisterStatusResponse, UpdateSettingsArgument,
};
use pocket_ic::{with_candid, CallError, PocketIc};
use std::time::Duration;
use wallet_api::{
    AddUserOperationInput, ApiErrorDTO, CreateProposalInput, CreateProposalResponse,
    GetProposalInput, GetProposalResponse, MeResponse, ProposalDTO, ProposalExecutionScheduleDTO,
    ProposalOperationDTO, ProposalOperationInput, ProposalStatusDTO, UserDTO, UserStatusDTO,
    VoteOnProposalInput, VoteOnProposalResponse, WalletSettingsResponse,
};

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
    wallet_canister_id: CanisterId,
    proposal: ProposalDTO,
) -> ProposalDTO {
    let get_proposal_args = GetProposalInput {
        proposal_id: proposal.id,
    };
    let res: (Result<GetProposalResponse, ApiErrorDTO>,) = update_candid_as(
        env,
        wallet_canister_id,
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
    wallet_canister_id: CanisterId,
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
        wallet_canister_id,
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
    wallet_canister_id: CanisterId,
    proposal: ProposalDTO,
) -> Result<ProposalDTO, Option<ProposalStatusDTO>> {
    wait_for_proposal_with_extra_ticks(env, user_id, wallet_canister_id, proposal, 0)
}

pub fn wait_for_proposal_with_extra_ticks(
    env: &PocketIc,
    user_id: Principal,
    wallet_canister_id: CanisterId,
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
        let new_proposal = get_proposal(env, user_id, wallet_canister_id, proposal.clone());
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
    wallet_canister_id: CanisterId,
    proposal_operation_input: ProposalOperationInput,
) -> Result<ProposalDTO, Option<ProposalStatusDTO>> {
    execute_proposal_with_extra_ticks(
        env,
        user_id,
        wallet_canister_id,
        proposal_operation_input,
        0,
    )
}

pub fn execute_proposal_with_extra_ticks(
    env: &PocketIc,
    user_id: Principal,
    wallet_canister_id: CanisterId,
    proposal_operation_input: ProposalOperationInput,
    extra_ticks: u64,
) -> Result<ProposalDTO, Option<ProposalStatusDTO>> {
    let proposal = submit_proposal(env, user_id, wallet_canister_id, proposal_operation_input);
    wait_for_proposal_with_extra_ticks(env, user_id, wallet_canister_id, proposal, extra_ticks)
}

pub fn vote_on_proposal(
    env: &PocketIc,
    user_id: Principal,
    wallet_canister_id: CanisterId,
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
        wallet_canister_id,
        user_id,
        "vote_on_proposal",
        (vote_on_proposal_input,),
    )
    .unwrap();
    res.0.unwrap();
}

pub fn add_user(
    env: &PocketIc,
    user_id: Principal,
    group_ids: Vec<String>,
    wallet_canister_id: Principal,
) -> UserDTO {
    let add_user = ProposalOperationInput::AddUser(AddUserOperationInput {
        name: None,
        identities: vec![user_id],
        groups: group_ids,
        status: UserStatusDTO::Active,
    });
    let add_user_proposal = submit_proposal(env, WALLET_ADMIN_USER, wallet_canister_id, add_user);
    let new_proposal = wait_for_proposal(
        env,
        WALLET_ADMIN_USER,
        wallet_canister_id,
        add_user_proposal,
    )
    .unwrap();
    match new_proposal.operation {
        ProposalOperationDTO::AddUser(add_user) => add_user.user.unwrap(),
        _ => panic!("invalid proposal operation"),
    }
}

pub fn get_user(env: &PocketIc, user_id: Principal, wallet_canister_id: Principal) -> UserDTO {
    let res: (ApiResult<MeResponse>,) =
        update_candid_as(env, wallet_canister_id, user_id, "me", ()).unwrap();
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

pub fn get_wallet_owners(
    env: &PocketIc,
    user_id: Principal,
    wallet_canister_id: Principal,
) -> Vec<Principal> {
    let res: (ApiResult<WalletSettingsResponse>,) =
        update_candid_as(env, wallet_canister_id, user_id, "wallet_settings", ((),)).unwrap();
    let wallet_settings = res.0.unwrap().settings;
    wallet_settings
        .owners
        .into_iter()
        .flat_map(|u| u.identities)
        .collect()
}

/// Call a canister candid update method, authenticated. The sender can be impersonated (i.e., the
/// signature is not verified).
pub fn update_candid_as<Input, Output>(
    env: &PocketIc,
    canister_id: CanisterId,
    sender: Principal,
    method: &str,
    input: Input,
) -> Result<Output, CallError>
where
    Input: ArgumentEncoder,
    Output: for<'a> ArgumentDecoder<'a>,
{
    with_candid(input, |bytes| {
        env.update_call(canister_id, sender, method, bytes)
    })
}
