use std::time::Duration;

use candid::Principal;
use num_format::{CustomFormat, ToFormattedString};
use pocket_ic::PocketIc;
use wallet_api::{
    AddUserGroupOperationInput, AddUserOperationInput, ApiErrorDTO, CreateProposalInput,
    CreateProposalResponse, GetUserGroupInput, GetUserGroupResponse, ListUserGroupsInput,
    ListUserGroupsResponse, ListUsersInput, ListUsersResponse, ProposalDTO,
    ProposalExecutionScheduleDTO, ProposalOperationInput, UserDTO, UserGroupDTO, UuidDTO,
    VoteOnProposalInput, VoteOnProposalResponse,
};

use crate::{
    setup::WALLET_ADMIN_USER,
    utils::{update_candid_as, user_test_id},
};

pub struct TestOrg {
    pub env: PocketIc,
    pub next_user_id_seed: u64,
    // pub users: Vec<UserDTO>,
    // pub groups: Vec<UserGroupDTO>,
    // pub group_users_map: HashMap<String, Vec<UserDTO>>,
    pub wallet_canister_id: Principal,
    pub cycles: CyclesTracker,
}

impl TestOrg {
    pub fn new(env: PocketIc, wallet_canister_id: Principal) -> Self {
        // let groups = Self::fetch_groups(&env, wallet_canister_id);
        // let users = Self::fetch_users(&env, wallet_canister_id);

        // fn users_in_group(group: &UserGroupDTO, users: &Vec<UserDTO>) -> Vec<UserDTO> {
        //     users
        //         .iter()
        //         .filter(|user| {
        //             user.groups
        //                 .iter()
        //                 .any(|user_group| user_group.id == group.id)
        //         })
        //         .cloned()
        //         .collect()
        // }

        let mut cycles = CyclesTracker::new(&env, wallet_canister_id);

        TestOrg {
            env,
            wallet_canister_id,
            next_user_id_seed: 0,
            cycles,
            // group_users_map: groups
            //     .iter()
            //     .map(|g| (g.name.clone(), users_in_group(g, &users)))
            //     .collect(),
            // groups,
            // users,
        }
    }

    fn fetch_group(env: &PocketIc, uuid: &UuidDTO, wallet_canister_id: Principal) -> UserGroupDTO {
        let res: (Result<GetUserGroupResponse, ApiErrorDTO>,) =
            update_candid_as::<(GetUserGroupInput,), (Result<GetUserGroupResponse, ApiErrorDTO>,)>(
                env,
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

    fn fetch_groups(env: &PocketIc, wallet_canister_id: Principal) -> Vec<UserGroupDTO> {
        let res: (Result<ListUserGroupsResponse, ApiErrorDTO>,) = update_candid_as::<
            (ListUserGroupsInput,),
            (Result<ListUserGroupsResponse, ApiErrorDTO>,),
        >(
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

    fn fetch_users(env: &PocketIc, wallet_canister_id: Principal) -> Vec<UserDTO> {
        let res: (Result<ListUsersResponse, ApiErrorDTO>,) =
            update_candid_as::<(ListUsersInput,), (Result<ListUsersResponse, ApiErrorDTO>,)>(
                env,
                wallet_canister_id,
                WALLET_ADMIN_USER,
                "list_users",
                (ListUsersInput {
                    paginate: None,
                    search_term: None,
                    statuses: None,
                },),
            )
            .unwrap();
        res.0.unwrap().users
    }

    fn get_next_user_id(&mut self) -> Principal {
        let principal = user_test_id(self.next_user_id_seed);
        self.next_user_id_seed += 1;
        principal
    }

    pub fn vote_on_proposal(
        &mut self,
        vote_as: Principal,
        proposal_id: &str,
        approve: bool,
    ) -> Result<VoteOnProposalResponse, ApiErrorDTO> {
        let res: (Result<VoteOnProposalResponse, ApiErrorDTO>,) = update_candid_as(
            &self.env,
            self.wallet_canister_id,
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

    pub fn pass_proposal(&mut self, proposal_id: &str, voters: Vec<Principal>) -> ProposalDTO {
        for voter in voters {
            let res = self.vote_on_proposal(voter, proposal_id, true);

            match res {
                Ok(vote_res) => {
                    if let wallet_api::ProposalStatusDTO::Adopted = vote_res.proposal.status {
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

    pub fn create_user(
        &mut self,
        create_as: Principal,
        name: &str,
        groups: Vec<String>,
    ) -> (ProposalDTO, Principal) {
        let user_id = self.get_next_user_id();

        let res: (Result<CreateProposalResponse, ApiErrorDTO>,) = update_candid_as(
            &self.env,
            self.wallet_canister_id,
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

        (res.0.unwrap().proposal, user_id)
    }

    pub fn create_group(&mut self, create_as: Principal, name: &str) -> ProposalDTO {
        let res: (Result<CreateProposalResponse, ApiErrorDTO>,) = update_candid_as(
            &self.env,
            self.wallet_canister_id,
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

    pub fn get_group_by_name(&self, group_name: &str) -> UserGroupDTO {
        let groups = Self::fetch_groups(&self.env, self.wallet_canister_id);
        groups
            .iter()
            .find(|g| g.name == group_name)
            .unwrap()
            .clone()
    }

    pub fn get_group_users(&self, group_name: &str) -> Vec<UserDTO> {
        let group = self.get_group_by_name(group_name);
        let group_id = group.id.clone();

        let users = Self::fetch_users(&self.env, self.wallet_canister_id);

        users
            .iter()
            .filter(|user| {
                user.groups
                    .iter()
                    .any(|user_group| user_group.id == group_id)
            })
            .cloned()
            .collect()
    }

    pub fn advance_time(&mut self, duration: std::time::Duration, ticks: u64) {
        let duration_per_tick = duration.as_nanos() as u64 / ticks;
        for _ in 0..ticks {
            self.env
                .advance_time(Duration::from_nanos(duration_per_tick));
            self.env.tick();
        }
    }
}

// pub struct CycleCheckpoint {
//     pub checkpoint: Option<String>,
//     pub start_cycles: u128,
// }

pub struct CyclesTracker {
    canister_id: Principal,
    initial_cycles: u128,
    latest_cycles: u128,
    // pub final_cycles: u128,
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

    pub fn make_snapshot(&mut self, env: &PocketIc) {
        let current_cycles = env.cycle_balance(self.canister_id);

        let used = self.latest_cycles - current_cycles;
        let formatter = CustomFormat::builder().separator("_").build().unwrap();
        println!("Cycles: {}", used.to_formatted_string(&formatter));

        self.latest_cycles = current_cycles;
    }
}
