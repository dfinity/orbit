use crate::setup::{get_canister_wasm, setup_new_env, WALLET_ADMIN_USER};
use crate::utils::{
    execute_proposal_with_extra_ticks, get_wallet_owners, submit_proposal, user_test_id,
    vote_on_proposal, wait_for_proposal_with_extra_ticks,
};
use crate::TestEnv;
use candid::Encode;
use sha2::{Digest, Sha256};
use wallet_api::{
    ChangeCanisterOperationInput, ChangeCanisterTargetDTO, ProposalOperationInput, WalletInstall,
    WalletUpgrade,
};

#[test]
fn successful_wallet_upgrade() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    // get wallet wasm
    let wallet_wasm = get_canister_wasm("wallet").to_vec();
    let mut hasher = Sha256::new();
    hasher.update(&wallet_wasm);
    let wallet_wasm_hash = hasher.finalize().to_vec();

    // check initial wallet owners
    let wallet_owners = get_wallet_owners(&env, WALLET_ADMIN_USER, canister_ids.wallet);
    assert_eq!(wallet_owners.len(), 1);
    assert!(wallet_owners.contains(&WALLET_ADMIN_USER));

    // submit wallet upgrade proposal setting a new wallet owner
    let user_id = user_test_id(0);
    let wallet_init_arg = WalletInstall::Upgrade(WalletUpgrade {
        owners: Some(vec![WALLET_ADMIN_USER, user_id]),
    });
    let wallet_init_arg_bytes = Encode!(&wallet_init_arg).unwrap();
    let wallet_upgrade_operation =
        ProposalOperationInput::ChangeCanister(ChangeCanisterOperationInput {
            target: ChangeCanisterTargetDTO::UpgradeWallet,
            module: wallet_wasm.clone(),
            arg: Some(wallet_init_arg_bytes),
            checksum: wallet_wasm_hash.clone(),
        });
    // extra ticks are necessary to prevent polling on the proposal status
    // before the wallet canister is upgraded and running
    execute_proposal_with_extra_ticks(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.wallet,
        wallet_upgrade_operation,
        10,
    )
    .unwrap();

    // check the new wallet owners
    let new_wallet_owners = get_wallet_owners(&env, WALLET_ADMIN_USER, canister_ids.wallet);
    assert_eq!(new_wallet_owners.len(), 2);
    assert!(new_wallet_owners.contains(&WALLET_ADMIN_USER));
    assert!(new_wallet_owners.contains(&user_id));

    // submit one more wallet upgrade proposal unsetting the new wallet owner
    let wallet_init_arg = WalletInstall::Upgrade(WalletUpgrade {
        owners: Some(vec![WALLET_ADMIN_USER]),
    });
    let wallet_init_arg_bytes = Encode!(&wallet_init_arg).unwrap();
    let wallet_upgrade_operation =
        ProposalOperationInput::ChangeCanister(ChangeCanisterOperationInput {
            target: ChangeCanisterTargetDTO::UpgradeWallet,
            module: wallet_wasm,
            arg: Some(wallet_init_arg_bytes),
            checksum: wallet_wasm_hash,
        });
    let wallet_upgrade_proposal = submit_proposal(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.wallet,
        wallet_upgrade_operation,
    );
    vote_on_proposal(
        &env,
        user_id,
        canister_ids.wallet,
        wallet_upgrade_proposal.clone(),
        true,
    );
    // extra ticks are necessary to prevent polling on the proposal status
    // before the wallet canister is upgraded and running
    wait_for_proposal_with_extra_ticks(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.wallet,
        wallet_upgrade_proposal,
        10,
    )
    .unwrap();

    // check the new wallet owners
    let new_wallet_owners = get_wallet_owners(&env, WALLET_ADMIN_USER, canister_ids.wallet);
    assert_eq!(new_wallet_owners.len(), 1);
    assert!(new_wallet_owners.contains(&WALLET_ADMIN_USER));
}
