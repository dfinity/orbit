use crate::setup::{get_canister_wasm, setup_new_env, WALLET_ADMIN_USER};
use crate::utils::{
    canister_status, execute_proposal_with_extra_ticks, get_core_canister_health_status,
    get_system_info, NNS_ROOT_CANISTER_ID,
};
use crate::TestEnv;
use candid::Encode;
use sha2::{Digest, Sha256};
use wallet_api::{
    ChangeCanisterOperationInput, ChangeCanisterTargetDTO, HealthStatus, ProposalOperationInput,
    SystemInstall, SystemUpgrade,
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

    // check if canister is healthy
    let health_status =
        get_core_canister_health_status(&env, WALLET_ADMIN_USER, canister_ids.wallet);
    assert_eq!(health_status, HealthStatus::Healthy);

    // submit wallet upgrade proposal
    let wallet_init_arg = SystemInstall::Upgrade(SystemUpgrade {});
    let wallet_init_arg_bytes = Encode!(&wallet_init_arg).unwrap();
    let wallet_upgrade_operation =
        ProposalOperationInput::ChangeCanister(ChangeCanisterOperationInput {
            target: ChangeCanisterTargetDTO::UpgradeWallet,
            module: wallet_wasm.clone(),
            arg: Some(wallet_init_arg_bytes),
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

    // check the status after the upgrade
    let health_status =
        get_core_canister_health_status(&env, WALLET_ADMIN_USER, canister_ids.wallet);
    assert_eq!(health_status, HealthStatus::Healthy);

    let system_info = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.wallet);
    assert!(system_info.raw_rand_successful);
    let last_uprade_timestamp = system_info.last_upgrade_timestamp;

    // submit one more wallet upgrade proposal with no changes
    let wallet_upgrade_operation =
        ProposalOperationInput::ChangeCanister(ChangeCanisterOperationInput {
            target: ChangeCanisterTargetDTO::UpgradeWallet,
            module: wallet_wasm.clone(),
            arg: None,
        });

    execute_proposal_with_extra_ticks(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.wallet,
        wallet_upgrade_operation,
        10,
    )
    .unwrap();

    // check the status after the upgrade
    let health_status =
        get_core_canister_health_status(&env, WALLET_ADMIN_USER, canister_ids.wallet);
    assert_eq!(health_status, HealthStatus::Healthy);

    let system_info = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.wallet);
    // check if the last upgrade timestamp is updated
    assert!(system_info.last_upgrade_timestamp > last_uprade_timestamp);

    let status = canister_status(&env, Some(NNS_ROOT_CANISTER_ID), canister_ids.wallet);
    assert_eq!(status.module_hash.unwrap(), wallet_wasm_hash);
}
