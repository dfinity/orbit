use crate::interfaces::{
    default_account, get_icp_balance, send_icp, send_icp_to_account, ICP, ICP_FEE,
};
use crate::setup::setup_new_env;
use crate::utils::user_test_id;
use crate::TestEnv;
use ic_canister_core::api::ApiResult;
use ic_ledger_types::AccountIdentifier;
use pocket_ic::call_candid_as;
use pocket_ic::common::rest::RawEffectivePrincipal;
use std::time::Duration;
use wallet_api::{
    ApiErrorDTO, CreateAccountInput, CreateAccountResponse, CreateProposalInput,
    CreateProposalResponse, GetProposalInput, GetProposalResponse, ProposalExecutionScheduleDTO,
    ProposalOperationInput, ProposalStatusDTO, RegisterUserInput, RegisterUserResponse,
    TransferOperationInput,
};

#[test]
fn make_transfer_successful() {
    let TestEnv {
        env,
        canister_ids,
        controller,
        ..
    } = setup_new_env();

    let user_id = user_test_id(0);
    let beneficiary_id = user_test_id(1);

    // register user
    let register_args = RegisterUserInput {
        identities: vec![user_id],
    };
    let res: (ApiResult<RegisterUserResponse>,) = call_candid_as(
        &env,
        canister_ids.wallet,
        RawEffectivePrincipal::None,
        user_id,
        "register_user",
        (register_args,),
    )
    .unwrap();
    let user_dto = res.0.unwrap().user;

    // create account
    let create_account_args = CreateAccountInput {
        owners: vec![user_dto.id],
        name: None,
        blockchain: "icp".to_string(),
        standard: "native".to_string(),
        policies: vec![],
        metadata: None,
    };
    let res: (ApiResult<CreateAccountResponse>,) = call_candid_as(
        &env,
        canister_ids.wallet,
        RawEffectivePrincipal::None,
        user_id,
        "create_account",
        (create_account_args,),
    )
    .unwrap();
    let account_dto = res.0.unwrap().account;

    // send ICP to user
    send_icp(&env, controller, user_id, ICP + 2 * ICP_FEE, 0).unwrap();
    let user_balance = get_icp_balance(&env, user_id);
    assert_eq!(user_balance, ICP + 2 * ICP_FEE);

    // send ICP to orbit wallet account
    let account_address = AccountIdentifier::from_hex(&account_dto.address).unwrap();
    send_icp_to_account(&env, user_id, account_address, ICP + ICP_FEE, 0).unwrap();

    // check user balance after transfer to orbit wallet account
    let new_user_balance = get_icp_balance(&env, user_id);
    assert_eq!(new_user_balance, 0);

    // check beneficiary balance
    let old_beneficiary_balance = get_icp_balance(&env, beneficiary_id);
    assert_eq!(old_beneficiary_balance, 0);

    // make transfer proposal to beneficiary
    let transfer = TransferOperationInput {
        from_account_id: account_dto.id,
        to: default_account(beneficiary_id),
        amount: ICP.into(),
        fee: None,
        metadata: None,
        network: None,
    };
    let transfer_proposal = CreateProposalInput {
        operation: ProposalOperationInput::Transfer(transfer),
        title: None,
        summary: None,
        execution_plan: Some(ProposalExecutionScheduleDTO::Immediate),
    };
    let res: (Result<CreateProposalResponse, ApiErrorDTO>,) = call_candid_as(
        &env,
        canister_ids.wallet,
        RawEffectivePrincipal::None,
        user_id,
        "create_proposal",
        (transfer_proposal,),
    )
    .unwrap();
    let proposal_dto = res.0.unwrap().proposal;

    // wait for the proposal to be executed (timer's period is 5 seconds)
    env.set_time(env.get_time() + Duration::from_secs(5));
    env.tick();

    // check transfer proposal status
    let get_proposal_args = GetProposalInput {
        proposal_id: proposal_dto.id,
    };
    let res: (Result<GetProposalResponse, ApiErrorDTO>,) = call_candid_as(
        &env,
        canister_ids.wallet,
        RawEffectivePrincipal::None,
        user_id,
        "get_proposal",
        (get_proposal_args,),
    )
    .unwrap();
    let new_proposal_dto = res.0.unwrap().proposal;
    match new_proposal_dto.status {
        ProposalStatusDTO::Completed { .. } => {}
        _ => {
            panic!("proposal must be completed by now");
        }
    };

    // check beneficiary balance after completed transfer
    let new_beneficiary_balance = get_icp_balance(&env, beneficiary_id);
    assert_eq!(new_beneficiary_balance, ICP);
}
