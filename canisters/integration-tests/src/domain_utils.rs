use std::{collections::HashMap, process::Output, time::Duration};

use candid::{
    ser::IDLBuilder,
    utils::{ArgumentDecoder, ArgumentEncoder},
    Principal,
};
use ic_ledger_types::{
    AccountBalanceArgs, AccountIdentifier, Memo, Tokens, TransferArgs, TransferError,
    DEFAULT_SUBACCOUNT,
};
use num_format::{CustomFormat, ToFormattedString};
use pocket_ic::{query_candid_as, PocketIc};
use wallet_api::{
    AccountDTO, AddAccessPolicyOperationInput, AddAccountOperationInput,
    AddAddressBookEntryOperationInput, AddUserGroupOperationInput, AddUserOperationInput,
    AddressBookEntryDTO, ApiErrorDTO, CreateProposalInput, CreateProposalResponse,
    GetProposalInput, GetProposalResponse, GetUserGroupInput, GetUserGroupResponse,
    ListAccountsInput, ListAccountsResponse, ListAddressBookEntriesInputDTO,
    ListAddressBookEntriesResponseDTO, ListProposalsInput, ListProposalsResponse,
    ListUserGroupsInput, ListUserGroupsResponse, ListUsersInput, ListUsersResponse, MeResponse,
    PaginationInput, ProposalDTO, ProposalExecutionScheduleDTO, ProposalOperationInput,
    ProposalStatusDTO, TransferOperationInput, UserDTO, UserGroupDTO, UuidDTO, VoteOnProposalInput,
    VoteOnProposalResponse,
};

use crate::{
    setup::WALLET_ADMIN_USER,
    utils::{is_proposal_adopted, update_candid_as, user_test_id},
};

/// Performs an update call to the canister and based on the cycle consumption
/// estimates the number of instructions executed.
pub fn measure_instruction_count_of_call<Input, Output>(
    env: &PocketIc,
    canister_id: Principal,
    subnet_nodes: usize,
    sender: Principal,
    method: &str,
    input: Input,
) -> u128
where
    Input: ArgumentEncoder + Clone,
    Output: for<'a> ArgumentDecoder<'a>,
{
    let cycles_before = env.cycle_balance(canister_id);
    let mut ser = IDLBuilder::new();
    input
        .clone()
        .encode(&mut ser)
        .expect("Failed to encode input");
    let input_size = ser.serialize_to_vec().unwrap().len() as u128;

    let _ = update_candid_as::<Input, Output>(&env, canister_id, sender, method, input)
        .expect(&format!("Failed to call {}", method));

    let cycles_after = env.cycle_balance(canister_id);

    let mut cost = cycles_before - cycles_after;

    let subnet_nodes = subnet_nodes as u128;

    cost -= subnet_nodes * 590_000 / 13; // cost of update call
    cost -= subnet_nodes * 1_200_000 / 13; // cost of ingress message
    cost -= subnet_nodes * input_size * 2000 / 13; // cost of input message

    let cost_of_1b = subnet_nodes * 400_000_000 / 13;

    1_000_000_000 * cost / cost_of_1b
}

pub fn fetch_group(env: &PocketIc, wallet_canister_id: Principal, uuid: &UuidDTO) -> UserGroupDTO {
    let res: (Result<GetUserGroupResponse, ApiErrorDTO>,) =
        query_candid_as::<(GetUserGroupInput,), (Result<GetUserGroupResponse, ApiErrorDTO>,)>(
            &env,
            wallet_canister_id,
            WALLET_ADMIN_USER,
            "get_user_group",
            (GetUserGroupInput {
                user_group_id: uuid.to_owned(),
            },),
        )
        .unwrap();
    res.0.unwrap().user_group
}

pub fn fetch_groups(env: &PocketIc, wallet_canister_id: Principal) -> Vec<UserGroupDTO> {
    let res: (Result<ListUserGroupsResponse, ApiErrorDTO>,) =
        query_candid_as::<(ListUserGroupsInput,), (Result<ListUserGroupsResponse, ApiErrorDTO>,)>(
            env,
            wallet_canister_id,
            WALLET_ADMIN_USER,
            "list_user_groups",
            (ListUserGroupsInput {
                paginate: None,
                search_term: None,
            },),
        )
        .unwrap();
    res.0.unwrap().user_groups
}

pub fn fetch_users(env: &PocketIc, wallet_canister_id: Principal) -> Vec<UserDTO> {
    let mut all_users = Vec::new();
    let mut paginate: Option<PaginationInput> = None;
    loop {
        let res: (Result<ListUsersResponse, ApiErrorDTO>,) =
            query_candid_as::<(ListUsersInput,), (Result<ListUsersResponse, ApiErrorDTO>,)>(
                env,
                wallet_canister_id,
                WALLET_ADMIN_USER,
                "list_users",
                (ListUsersInput {
                    paginate,
                    search_term: None,
                    statuses: None,
                },),
            )
            .unwrap();

        let response = res.0.unwrap();

        all_users.extend(response.users.clone());

        if let Some(next_offset) = response.next_offset {
            paginate = Some(PaginationInput {
                offset: Some(next_offset),
                limit: None,
            });
        } else {
            break;
        }
        // res.0.unwrap().users
    }
    all_users
}

pub fn fetch_accounts(
    env: &PocketIc,
    wallet_canister_id: Principal,
    sender: Principal,
) -> Vec<AccountDTO> {
    let res: (Result<ListAccountsResponse, ApiErrorDTO>,) = query_candid_as(
        env,
        wallet_canister_id,
        sender,
        "list_accounts",
        (ListAccountsInput {
            paginate: None,
            search_term: None,
        },),
    )
    .unwrap();
    res.0.unwrap().accounts
}

pub fn fetch_proposals(
    env: &PocketIc,
    wallet_canister_id: Principal,
    sender: Principal,
    input: Option<ListProposalsInput>,
) -> Vec<ProposalDTO> {
    let res: (Result<ListProposalsResponse, ApiErrorDTO>,) = query_candid_as(
        env,
        wallet_canister_id,
        sender,
        "list_proposals",
        (input.unwrap_or(ListProposalsInput {
            created_from_dt: None,
            created_to_dt: None,
            expiration_from_dt: None,
            expiration_to_dt: None,
            operation_types: None,
            voter_ids: None,
            proposer_ids: None,
            statuses: None,
            paginate: Some(PaginationInput {
                limit: Some(100),
                offset: None,
            }),
            sort_by: None,
        }),),
    )
    .unwrap();
    res.0.unwrap().proposals
}

pub fn fetch_addressbook_entries(
    env: &PocketIc,
    wallet_canister_id: Principal,
    sender: Principal,
) -> Vec<AddressBookEntryDTO> {
    let mut all_addressbook_entries = Vec::new();
    let mut paginate: Option<PaginationInput> = None;
    loop {
        let res: (Result<ListAddressBookEntriesResponseDTO, ApiErrorDTO>,) = query_candid_as(
            env,
            wallet_canister_id,
            sender,
            "list_address_book_entries",
            (ListAddressBookEntriesInputDTO {
                paginate,
                address_chain: None,
                addresses: None,
                ids: None,
            },),
        )
        .unwrap();

        let response = res.0.unwrap();

        all_addressbook_entries.extend(response.address_book_entries.clone());

        if let Some(next_offset) = response.next_offset {
            paginate = Some(PaginationInput {
                offset: Some(next_offset),
                limit: None,
            });
        } else {
            break;
        }
    }

    all_addressbook_entries
}

pub fn get_user(env: &PocketIc, sender: Principal, wallet_canister_id: Principal) -> MeResponse {
    let res: (Result<MeResponse, ApiErrorDTO>,) =
        update_candid_as(env, wallet_canister_id, sender, "me", ()).unwrap();
    res.0.unwrap()
}

pub fn mint_icp(
    env: &PocketIc,
    minter_identity: Principal,
    to_account: AccountIdentifier,
    amount_e8s: u64,
) -> Result<u64, TransferError> {
    let transfer_args = TransferArgs {
        memo: Memo(0),
        amount: Tokens::from_e8s(amount_e8s),
        fee: Tokens::from_e8s(0),
        from_subaccount: None,
        to: to_account,
        created_at_time: None,
    };
    let res: (Result<u64, TransferError>,) = update_candid_as(
        env,
        Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(),
        minter_identity,
        "transfer",
        (transfer_args,),
    )
    .unwrap();
    res.0
}

pub fn get_icp_balance(env: &PocketIc, account: AccountIdentifier, sender: Principal) -> u64 {
    let ledger_canister_id = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
    let res: (Tokens,) = update_candid_as(
        env,
        ledger_canister_id,
        sender,
        "account_balance",
        (AccountBalanceArgs { account },),
    )
    .unwrap();
    res.0.e8s()
}

// fn get_next_user_id(env: &PocketIc, wallet_canister_id: Principal) -> Principal {
//     let principal = user_test_id(self.next_user_id_seed);
//     self.next_user_id_seed += 1;
//     principal
// }

pub fn vote_on_proposal(
    env: &PocketIc,
    wallet_canister_id: Principal,
    vote_as: Principal,
    proposal_id: &str,
    approve: bool,
) -> Result<VoteOnProposalResponse, ApiErrorDTO> {
    let res: (Result<VoteOnProposalResponse, ApiErrorDTO>,) = update_candid_as(
        env,
        wallet_canister_id,
        vote_as,
        "vote_on_proposal",
        (VoteOnProposalInput {
            proposal_id: proposal_id.to_owned(),
            approve,
            reason: None,
        },),
    )
    .unwrap();
    res.0
}

pub fn wait_for_proposal_completion(
    env: &PocketIc,
    wallet_canister_id: Principal,
    proposal_id: &UuidDTO,
) -> ProposalDTO {
    let mut waits_left = 5;

    loop {
        let proposal = get_proposal(
            env,
            WALLET_ADMIN_USER,
            wallet_canister_id,
            proposal_id.to_owned(),
        );

        match proposal.status {
            ProposalStatusDTO::Completed { .. } => {
                return proposal;
            }
            ProposalStatusDTO::Failed { reason } => {
                panic!("Proposal failed: {:?}", reason);
            }
            _ => {
                if waits_left == 0 {
                    panic!(
                        "Proposal not completed in time, stuck at {:?}",
                        proposal.status
                    );
                }

                advance_time(env, Duration::from_secs(10), 1);

                waits_left -= 1;
            }
        }
    }
}

pub fn pass_proposal(
    env: &PocketIc,
    wallet_canister_id: Principal,
    proposal_id: &UuidDTO,
    voters: &Vec<UserDTO>,
    wait_for_completion: bool,
) -> ProposalDTO {
    let proposal = get_proposal(
        env,
        WALLET_ADMIN_USER,
        wallet_canister_id,
        proposal_id.to_owned(),
    );

    if is_proposal_adopted(&proposal.status) {
        return proposal;
    }

    for voter in voters {
        if proposal.votes.iter().any(|v| v.user_id == voter.id) {
            continue;
        }

        let res = vote_on_proposal(
            env,
            wallet_canister_id,
            *voter.identities.first().unwrap(),
            proposal_id,
            true,
        );

        match res {
            Ok(vote_res) => {
                if is_proposal_adopted(&vote_res.proposal.status) {
                    if wait_for_completion {
                        wait_for_proposal_completion(env, wallet_canister_id, proposal_id);
                    }
                    return vote_res.proposal;
                }
            }
            Err(e) => {
                println!("{:#?}", e);
            }
        };
    }

    panic!("Proposal not adopted");
}

// pub fn create_user(&mut self, name: &str, groups: Vec<UuidDTO>) -> UserDTO {
//     let principal = self.get_next_user_id();
//     let proposal = self.send_create_user_proposal(create_as, name, groups);

// }

pub fn create_add_user_proposal(
    env: &PocketIc,
    wallet_canister_id: Principal,
    user_id: Principal,
    create_as: Principal,
    name: &str,
    groups: Vec<String>,
) -> ProposalDTO {
    let res: (Result<CreateProposalResponse, ApiErrorDTO>,) = update_candid_as(
        env,
        wallet_canister_id,
        create_as,
        "create_proposal",
        (CreateProposalInput {
            operation: ProposalOperationInput::AddUser(AddUserOperationInput {
                name: Some(name.to_owned()),
                identities: vec![user_id],
                groups,
                status: wallet_api::UserStatusDTO::Active,
            }),
            title: None,
            summary: None,
            execution_plan: Some(ProposalExecutionScheduleDTO::Immediate),
        },),
    )
    .unwrap();

    res.0.unwrap().proposal
}

pub fn create_add_user_group_proposal(
    env: &PocketIc,
    wallet_canister_id: Principal,
    create_as: Principal,
    name: &str,
) -> ProposalDTO {
    let res: (Result<CreateProposalResponse, ApiErrorDTO>,) = update_candid_as(
        env,
        wallet_canister_id,
        create_as,
        "create_proposal",
        (CreateProposalInput {
            operation: ProposalOperationInput::AddUserGroup(AddUserGroupOperationInput {
                name: name.to_owned(),
            }),
            title: None,
            summary: None,
            execution_plan: Some(ProposalExecutionScheduleDTO::Immediate),
        },),
    )
    .unwrap();

    res.0.unwrap().proposal
}

pub fn create_add_access_policy_proposal(
    env: &PocketIc,
    wallet_canister_id: Principal,
    create_as: Principal,
    input: AddAccessPolicyOperationInput,
) -> ProposalDTO {
    let res: (Result<CreateProposalResponse, ApiErrorDTO>,) = update_candid_as(
        env,
        wallet_canister_id,
        create_as,
        "create_proposal",
        (CreateProposalInput {
            operation: ProposalOperationInput::AddAccessPolicy(input),
            title: None,
            summary: None,
            execution_plan: Some(ProposalExecutionScheduleDTO::Immediate),
        },),
    )
    .unwrap();

    res.0.unwrap().proposal
}

pub fn create_transfer_proposal(
    env: &PocketIc,
    wallet_canister_id: Principal,
    create_as: Principal,
    input: TransferOperationInput,
) -> ProposalDTO {
    let res: (Result<CreateProposalResponse, ApiErrorDTO>,) = update_candid_as(
        env,
        wallet_canister_id,
        create_as,
        "create_proposal",
        (CreateProposalInput {
            operation: ProposalOperationInput::Transfer(input),
            title: None,
            summary: None,
            execution_plan: Some(ProposalExecutionScheduleDTO::Immediate),
        },),
    )
    .unwrap();

    res.0.unwrap().proposal
}

pub fn get_group_by_name(
    env: &PocketIc,
    wallet_canister_id: Principal,
    group_name: &str,
) -> UserGroupDTO {
    let groups = fetch_groups(env, wallet_canister_id);
    groups
        .iter()
        .find(|g| g.name == group_name)
        .unwrap()
        .clone()
}

pub fn get_group_users(
    env: &PocketIc,
    wallet_canister_id: Principal,
    group_id: &UuidDTO,
) -> Vec<UserDTO> {
    let users = fetch_users(env, wallet_canister_id);

    users
        .iter()
        .filter(|user| {
            user.groups
                .iter()
                .any(|user_group| &user_group.id == group_id)
        })
        .cloned()
        .collect()
}

pub fn get_proposal(
    env: &PocketIc,
    user_id: Principal,
    wallet_canister_id: Principal,
    proposal_id: UuidDTO,
) -> ProposalDTO {
    let get_proposal_args = GetProposalInput { proposal_id };
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

pub fn create_add_address_book_entry_proposal(
    env: &PocketIc,
    wallet_canister_id: Principal,
    create_as: Principal,
    input: AddAddressBookEntryOperationInput,
) -> ProposalDTO {
    let res: (Result<CreateProposalResponse, ApiErrorDTO>,) = update_candid_as(
        env,
        wallet_canister_id,
        create_as,
        "create_proposal",
        (CreateProposalInput {
            operation: ProposalOperationInput::AddAddressBookEntry(input),
            title: None,
            summary: None,
            execution_plan: Some(ProposalExecutionScheduleDTO::Immediate),
        },),
    )
    .unwrap();

    res.0.unwrap().proposal
}

pub fn create_add_account_proposal(
    env: &PocketIc,
    wallet_canister_id: Principal,
    create_as: Principal,
    input: AddAccountOperationInput,
) -> ProposalDTO {
    let res: (Result<CreateProposalResponse, ApiErrorDTO>,) = update_candid_as(
        env,
        wallet_canister_id,
        create_as,
        "create_proposal",
        (CreateProposalInput {
            operation: ProposalOperationInput::AddAccount(input),
            title: None,
            summary: None,
            execution_plan: Some(ProposalExecutionScheduleDTO::Immediate),
        },),
    )
    .unwrap();

    res.0.unwrap().proposal
}

pub fn get_estimated_instructions_from_cycles(cycles: u128, subnet_nodes: usize) -> u128 {
    // for a 13 node application subnet it costs 400M cycles to run 1b instructions
    cycles * 10 * subnet_nodes as u128 / 4 / 13
}

pub fn advance_time(env: &PocketIc, duration: std::time::Duration, ticks: u64) {
    let duration_per_tick = duration.as_nanos() as u64 / ticks;
    for _ in 0..ticks {
        env.advance_time(Duration::from_nanos(duration_per_tick));
        env.tick();
    }
}

pub struct CyclesTracker {
    canister_id: Principal,
    initial_cycles: u128,
    latest_cycles: u128,
    // pub final_cycles: u128,
}

pub struct UnderscoreFormatter {
    format: CustomFormat,
}

impl UnderscoreFormatter {
    pub fn new() -> Self {
        let format = CustomFormat::builder().separator("_").build().unwrap();
        UnderscoreFormatter { format }
    }

    pub fn format(&self, value: u128) -> String {
        value.to_formatted_string(&self.format)
    }
}

impl CyclesTracker {
    pub fn new(env: &PocketIc, canister_id: Principal) -> Self {
        let initial_cycles = env.cycle_balance(canister_id);
        CyclesTracker {
            canister_id,
            initial_cycles,
            latest_cycles: initial_cycles,
            // final_cycles: initial_cycles,
        }
    }

    pub fn make_snapshot(&mut self, env: &PocketIc) -> u128 {
        let current_cycles = env.cycle_balance(self.canister_id);

        let used = self.latest_cycles - current_cycles;

        self.latest_cycles = current_cycles;

        used
    }
}

pub struct UserIdGenerator {
    next_user_id_seed: u64,
}

impl UserIdGenerator {
    pub fn new() -> Self {
        UserIdGenerator {
            next_user_id_seed: 0,
        }
    }

    pub fn get_next_user_id(&mut self) -> Principal {
        let principal = user_test_id(self.next_user_id_seed);
        self.next_user_id_seed += 1;
        principal
    }
}
