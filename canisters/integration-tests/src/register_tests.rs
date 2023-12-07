use crate::setup::{setup_new_env, WALLET_ADMIN_USER};
use crate::utils::{update_candid_as, user_test_id};
use crate::TestEnv;
use std::time::Duration;
use wallet_api::{
    AddUserOperationInput, ApiErrorDTO, CreateProposalInput, CreateProposalResponse,
    GetProposalInput, GetProposalResponse, ProposalExecutionScheduleDTO, ProposalOperationDTO,
    ProposalOperationInput, ProposalStatusDTO,
};

#[test]
fn register_user_successful() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let user_id = user_test_id(0);

    // add a user through a proposal
    let add_user = AddUserOperationInput {
        name: Some("test".to_string()),
        identities: vec![user_id],
        groups: vec![],
        status: wallet_api::UserStatusDTO::Active,
        unconfirmed_identities: vec![],
    };
    let add_user_proposal = CreateProposalInput {
        operation: ProposalOperationInput::AddUser(add_user),
        title: None,
        summary: None,
        execution_plan: Some(ProposalExecutionScheduleDTO::Immediate),
    };

    let res: (Result<CreateProposalResponse, ApiErrorDTO>,) = update_candid_as(
        &env,
        canister_ids.wallet,
        WALLET_ADMIN_USER,
        "create_proposal",
        (add_user_proposal,),
    )
    .unwrap();
    let proposal_dto = res.0.unwrap().proposal;

    // wait for the proposal to be adopted and scheduled (timer's period is 5 seconds)
    env.advance_time(Duration::from_secs(5));
    env.tick();
    // wait for the proposal to be executed (timer's period is 5 seconds)
    env.advance_time(Duration::from_secs(5));
    env.tick();

    // check transfer proposal status
    let get_proposal_args = GetProposalInput {
        proposal_id: proposal_dto.id,
    };
    let res: (Result<GetProposalResponse, ApiErrorDTO>,) = update_candid_as(
        &env,
        canister_ids.wallet,
        WALLET_ADMIN_USER,
        "get_proposal",
        (get_proposal_args,),
    )
    .unwrap();
    let new_proposal_dto = res.0.unwrap().proposal;
    match new_proposal_dto.status {
        ProposalStatusDTO::Completed { .. } => {}
        _ => {
            panic!(
                "proposal must be completed by now but instead is {:?}",
                new_proposal_dto.status
            );
        }
    };

    if let ProposalOperationDTO::AddUser(add_user) = new_proposal_dto.operation {
        assert_eq!(add_user.user.unwrap().name, Some("test".to_string()));
    } else {
        panic!("proposal operation must be AddUser");
    }
}
