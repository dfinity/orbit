use crate::interfaces::{
    default_account, get_icp_balance, send_icp, send_icp_to_account, ICP, ICP_FEE,
};
use crate::setup::{setup_new_env, WALLET_ADMIN_USER};
use crate::utils::user_test_id;
use crate::TestEnv;
use ic_canister_core::api::ApiResult;
use ic_ledger_types::AccountIdentifier;
use pocket_ic::{query_candid_as, update_candid_as};
use std::time::Duration;
use wallet_api::{
    AccountPoliciesDTO, AddAccountOperationInput, ApiErrorDTO, ApprovalThresholdDTO,
    CreateProposalInput, CreateProposalResponse, CriteriaDTO, GetProposalInput,
    GetProposalResponse, GetTransfersInput, GetTransfersResponse, ListAccountTransfersInput,
    ListAccountTransfersResponse, MeResponse, ProposalExecutionScheduleDTO, ProposalOperationDTO,
    ProposalOperationInput, ProposalStatusDTO, TransferOperationInput, UserSpecifierDTO,
};

#[test]
fn make_transfer_successful() {
    let TestEnv {
        env,
        canister_ids,
        controller,
        ..
    } = setup_new_env();

    let beneficiary_id = user_test_id(1);

    // register user
    let res: (ApiResult<MeResponse>,) =
        update_candid_as(&env, canister_ids.wallet, WALLET_ADMIN_USER, "me", ()).unwrap();
    let user_dto = res.0.unwrap().me;

    // create account
    let create_account_args = AddAccountOperationInput {
        owners: vec![user_dto.id],
        name: "test".to_string(),
        blockchain: "icp".to_string(),
        standard: "native".to_string(),
        policies: AccountPoliciesDTO {
            transfer: Some(CriteriaDTO::ApprovalThreshold(ApprovalThresholdDTO {
                voters: UserSpecifierDTO::Owner,
                threshold: 100,
            })),
            edit: Some(CriteriaDTO::ApprovalThreshold(ApprovalThresholdDTO {
                voters: UserSpecifierDTO::Owner,
                threshold: 100,
            })),
        },
        metadata: vec![],
    };
    let add_account_proposal = CreateProposalInput {
        operation: ProposalOperationInput::AddAccount(create_account_args),
        title: None,
        summary: None,
        execution_plan: Some(ProposalExecutionScheduleDTO::Immediate),
    };
    let res: (ApiResult<CreateProposalResponse>,) = update_candid_as(
        &env,
        canister_ids.wallet,
        WALLET_ADMIN_USER,
        "create_proposal",
        (add_account_proposal,),
    )
    .unwrap();

    // wait for the proposal to be adopted (timer's period is 5 seconds)
    env.advance_time(Duration::from_secs(5));
    env.tick();

    let account_creation_proposal_dto = res.0.unwrap().proposal;
    match account_creation_proposal_dto.status {
        ProposalStatusDTO::Adopted { .. } => {}
        _ => {
            panic!("proposal must be adopted by now");
        }
    };

    // wait for the proposal to be executed (timer's period is 5 seconds)
    env.advance_time(Duration::from_secs(5));
    env.tick();

    // fetch the created account id from the proposal
    let get_proposal_args = GetProposalInput {
        proposal_id: account_creation_proposal_dto.id,
    };
    let res: (ApiResult<CreateProposalResponse>,) = update_candid_as(
        &env,
        canister_ids.wallet,
        WALLET_ADMIN_USER,
        "get_proposal",
        (get_proposal_args,),
    )
    .unwrap();
    let finalized_proposal = res.0.unwrap().proposal;
    match finalized_proposal.status {
        ProposalStatusDTO::Completed { .. } => {}
        _ => {
            panic!(
                "proposal must be completed by now but instead is {:?}",
                finalized_proposal.status
            );
        }
    };

    let account_dto = match finalized_proposal.operation {
        ProposalOperationDTO::AddAccount(add_account) => add_account.account.unwrap(),
        _ => {
            panic!("proposal must be AddAccount");
        }
    };

    // send ICP to user
    send_icp(&env, controller, WALLET_ADMIN_USER, ICP + 2 * ICP_FEE, 0).unwrap();
    let user_balance = get_icp_balance(&env, WALLET_ADMIN_USER);
    assert_eq!(user_balance, ICP + 2 * ICP_FEE);

    // send ICP to orbit wallet account
    let account_address = AccountIdentifier::from_hex(&account_dto.address).unwrap();
    send_icp_to_account(&env, WALLET_ADMIN_USER, account_address, ICP + ICP_FEE, 0).unwrap();

    // check user balance after transfer to orbit wallet account
    let new_user_balance = get_icp_balance(&env, WALLET_ADMIN_USER);
    assert_eq!(new_user_balance, 0);

    // check beneficiary balance
    let old_beneficiary_balance = get_icp_balance(&env, beneficiary_id);
    assert_eq!(old_beneficiary_balance, 0);

    // make transfer proposal to beneficiary
    let transfer = TransferOperationInput {
        from_account_id: account_dto.id.clone(),
        to: default_account(beneficiary_id),
        amount: ICP.into(),
        fee: None,
        metadata: vec![],
        network: None,
    };
    let transfer_proposal = CreateProposalInput {
        operation: ProposalOperationInput::Transfer(transfer),
        title: None,
        summary: None,
        execution_plan: Some(ProposalExecutionScheduleDTO::Immediate),
    };
    let res: (Result<CreateProposalResponse, ApiErrorDTO>,) = update_candid_as(
        &env,
        canister_ids.wallet,
        WALLET_ADMIN_USER,
        "create_proposal",
        (transfer_proposal,),
    )
    .unwrap();
    let proposal_dto = res.0.unwrap().proposal;

    // wait for the proposal to be adopted (timer's period is 5 seconds)
    env.advance_time(Duration::from_secs(5));
    env.tick();
    // wait for the proposal to be processing (timer's period is 5 seconds) and first is set to processing
    env.advance_time(Duration::from_secs(5));
    env.tick();
    env.tick();

    // check transfer proposal status
    let get_proposal_args = GetProposalInput {
        proposal_id: proposal_dto.id.clone(),
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

    // proposal has the transfer id filled out
    let transfer_id = match new_proposal_dto.operation {
        ProposalOperationDTO::Transfer(transfer) => transfer
            .transfer_id
            .expect("transfer id must be set for completed transfer"),
        _ => {
            panic!("proposal must be Transfer");
        }
    };

    // fetch the transfer and check if its proposal id matches the proposal id that created it
    let res: (Result<GetTransfersResponse, ApiErrorDTO>,) = query_candid_as(
        &env,
        canister_ids.wallet,
        WALLET_ADMIN_USER,
        "get_transfers",
        (GetTransfersInput {
            transfer_ids: vec![transfer_id],
        },),
    )
    .unwrap();

    let proposal_id_in_transfer_dto = res
        .0
        .unwrap()
        .transfers
        .first()
        .expect("One transaction must be returned")
        .proposal_id
        .clone();

    assert_eq!(proposal_id_in_transfer_dto, proposal_dto.id);

    // check beneficiary balance after completed transfer
    let new_beneficiary_balance = get_icp_balance(&env, beneficiary_id);
    assert_eq!(new_beneficiary_balance, ICP);

    // load account transfers
    let res: (Result<ListAccountTransfersResponse, ApiErrorDTO>,) = query_candid_as(
        &env,
        canister_ids.wallet,
        WALLET_ADMIN_USER,
        "list_account_transfers",
        (ListAccountTransfersInput {
            account_id: account_dto.id,
            from_dt: None,
            to_dt: None,
            status: None,
        },),
    )
    .unwrap();

    // transactions should be completed and have transaction hash
    let all_have_transaction_hash = res.0.unwrap().transfers.iter().all(|t| match &t.status {
        wallet_api::TransferStatusDTO::Completed { hash, .. } => hash.is_some(),
        _ => {
            panic!("transfer should be completed");
        }
    });

    assert!(all_have_transaction_hash);
}
