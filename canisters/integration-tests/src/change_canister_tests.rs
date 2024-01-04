use crate::setup::{create_canister, setup_new_env, WALLET_ADMIN_USER};
use crate::utils::{
    add_user, canister_status, execute_proposal, submit_proposal, user_test_id, vote_on_proposal,
    wait_for_proposal_completed,
};
use crate::TestEnv;
use wallet_api::{
    AddAccessPolicyOperationInput, AddProposalPolicyOperationInput,
    ChangeCanisterActionSpecifierDTO, ChangeCanisterOperationInput, ChangeCanisterTargetDTO,
    CommonSpecifierDTO, CriteriaDTO, ProposalOperationInput, ProposalSpecifierDTO,
    ResourceSpecifierDTO, UserSpecifierDTO,
};

#[test]
fn successful_four_eyes_upgrade() {
    let TestEnv {
        mut env,
        canister_ids,
        ..
    } = setup_new_env();

    // set four eyes principle for canister changes
    let add_proposal_policy =
        ProposalOperationInput::AddProposalPolicy(AddProposalPolicyOperationInput {
            specifier: ProposalSpecifierDTO::ChangeCanister,
            criteria: CriteriaDTO::MinimumVotes(UserSpecifierDTO::Any, 2),
        });
    execute_proposal(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.wallet,
        add_proposal_policy,
    );

    // allow anyone to create change canister proposals
    let add_access_policy =
        ProposalOperationInput::AddAccessPolicy(AddAccessPolicyOperationInput {
            user: CommonSpecifierDTO::Any,
            resource: ResourceSpecifierDTO::ChangeCanister(
                ChangeCanisterActionSpecifierDTO::Create,
            ),
        });
    execute_proposal(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.wallet,
        add_access_policy,
    );

    // create new user identities and add them to the wallet
    let user_a = user_test_id(0);
    add_user(&env, user_a, vec![], canister_ids.wallet);
    let user_b = user_test_id(1);
    add_user(&env, user_b, vec![], canister_ids.wallet);

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

    // new canister WASM
    let new_module_bytes = hex::decode("0061736d010000000503010001").unwrap();
    let new_module_hash =
        hex::decode("d7f602df8d1cb581cc5c886a4ff8809793c50627e305ef45f6d770f27e0261cc").unwrap();

    // make canister upgrade proposal
    let change_canister_operation =
        ProposalOperationInput::ChangeCanister(ChangeCanisterOperationInput {
            target: ChangeCanisterTargetDTO::UpgradeCanister(canister_id),
            module: new_module_bytes,
            arg: None,
            checksum: new_module_hash.clone(),
        });
    let change_canister_operation_proposal =
        submit_proposal(&env, user_a, canister_ids.wallet, change_canister_operation);
    vote_on_proposal(
        &env,
        user_b,
        canister_ids.wallet,
        change_canister_operation_proposal.clone(),
        true,
    );
    wait_for_proposal_completed(
        &env,
        user_a,
        canister_ids.wallet,
        change_canister_operation_proposal.clone(),
    );

    // check canister status
    let status = canister_status(&env, Some(canister_ids.wallet), canister_id);
    assert_eq!(status.module_hash, Some(new_module_hash));
}
