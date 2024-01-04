use crate::setup::{create_canister, setup_new_env, WALLET_ADMIN_USER};
use crate::utils::{canister_status, update_candid_as};
use crate::TestEnv;
use std::time::Duration;
use wallet_api::{
    ApiErrorDTO, ChangeCanisterOperationInput, ChangeCanisterTargetDTO, CreateProposalInput,
    CreateProposalResponse, GetProposalInput, GetProposalResponse, ProposalExecutionScheduleDTO,
    ProposalOperationInput, ProposalStatusDTO,
};

#[test]
fn make_install_successful() {
    let TestEnv {
        mut env,
        canister_ids,
        ..
    } = setup_new_env();

    // create the canister to be installed by a proposal
    let canister_id = create_canister(&mut env, canister_ids.wallet);
    // check canister status
    let status = canister_status(&env, Some(canister_ids.wallet), canister_id);
    assert_eq!(status.module_hash, None);

    // make canister install proposal
    let module_bytes = vec![0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
    let module_hash =
        hex::decode("93a44bbb96c751218e4c00d479e4c14358122a389acca16205b1e4d0dc5f9476").unwrap();
    let install_canister = ChangeCanisterOperationInput {
        target: ChangeCanisterTargetDTO::InstallCanister(canister_id),
        module: module_bytes,
        arg: None,
        checksum: module_hash.clone(),
    };
    let change_canister_proposal = CreateProposalInput {
        operation: ProposalOperationInput::ChangeCanister(install_canister),
        title: None,
        summary: None,
        execution_plan: Some(ProposalExecutionScheduleDTO::Immediate),
    };
    let res: (Result<CreateProposalResponse, ApiErrorDTO>,) = update_candid_as(
        &env,
        canister_ids.wallet,
        WALLET_ADMIN_USER,
        "create_proposal",
        (change_canister_proposal,),
    )
    .unwrap();
    let proposal_dto = res.0.unwrap().proposal;

    // wait for the proposal to be adopted (timer's period is 5 seconds)
    env.advance_time(Duration::from_secs(5));
    env.tick();
    // wait for the proposal to be processing (timer's period is 5 seconds) and then completed
    env.advance_time(Duration::from_secs(5));
    env.tick();
    env.tick();
    env.tick();
    env.tick();
    env.tick();
    env.tick();

    // check canister change proposal status
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

    // check canister status
    let status = canister_status(&env, Some(canister_ids.wallet), canister_id);
    assert_eq!(status.module_hash, Some(module_hash));
}

#[test]
fn make_upgrade_successful() {
    let TestEnv {
        mut env,
        canister_ids,
        ..
    } = setup_new_env();

    // create and install the canister to be upgraded by a proposal
    let canister_id = create_canister(&mut env, canister_ids.wallet);
    let module_bytes = vec![0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
    let module_hash =
        hex::decode("93a44bbb96c751218e4c00d479e4c14358122a389acca16205b1e4d0dc5f9476").unwrap();
    env.install_canister(
        canister_id,
        module_bytes.clone(),
        vec![],
        Some(canister_ids.wallet),
    );
    // check canister status
    let status = canister_status(&env, Some(canister_ids.wallet), canister_id);
    assert_eq!(status.module_hash, Some(module_hash.clone()));

    let new_module_bytes = hex::decode("0061736d010000000503010001").unwrap();
    let new_module_hash =
        hex::decode("d7f602df8d1cb581cc5c886a4ff8809793c50627e305ef45f6d770f27e0261cc").unwrap();

    // make canister upgrade proposal
    let upgrade_canister = ChangeCanisterOperationInput {
        target: ChangeCanisterTargetDTO::UpgradeCanister(canister_id),
        module: new_module_bytes,
        arg: None,
        checksum: new_module_hash.clone(),
    };
    let change_canister_proposal = CreateProposalInput {
        operation: ProposalOperationInput::ChangeCanister(upgrade_canister),
        title: None,
        summary: None,
        execution_plan: Some(ProposalExecutionScheduleDTO::Immediate),
    };
    let res: (Result<CreateProposalResponse, ApiErrorDTO>,) = update_candid_as(
        &env,
        canister_ids.wallet,
        WALLET_ADMIN_USER,
        "create_proposal",
        (change_canister_proposal,),
    )
    .unwrap();
    let proposal_dto = res.0.unwrap().proposal;

    // wait for the proposal to be adopted (timer's period is 5 seconds)
    env.advance_time(Duration::from_secs(5));
    env.tick();
    // wait for the proposal to be processing (timer's period is 5 seconds) and then completed
    env.advance_time(Duration::from_secs(5));
    env.tick();
    env.tick();
    env.tick();
    env.tick();
    env.tick();
    env.tick();

    // check upgrade proposal status
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

    // check canister status
    let status = canister_status(&env, Some(canister_ids.wallet), canister_id);
    assert_eq!(status.module_hash, Some(new_module_hash));
}
