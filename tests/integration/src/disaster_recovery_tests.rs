use crate::setup::{
    get_canister_wasm, setup_new_env, setup_new_env_with_config, WALLET_ADMIN_USER,
};
use crate::utils::{
    add_user, advance_time_to_burn_cycles, execute_request, get_core_canister_health_status,
    get_system_info, get_upgrader_disaster_recovery, get_upgrader_logs, get_user,
    set_disaster_recovery, user_test_id, NNS_ROOT_CANISTER_ID,
};
use crate::TestEnv;
use candid::{Encode, Principal};
use orbit_essentials::api::ApiResult;
use orbit_essentials::utils::sha256_hash;
use pocket_ic::{query_candid_as, update_candid_as};
use station_api::{
    AddAccountOperationInput, AllowDTO, DisasterRecoveryCommitteeDTO, HealthStatus,
    RequestOperationInput, SetDisasterRecoveryOperationInput, SystemInit, SystemInstall,
    SystemUpgrade,
};
use upgrader_api::{
    Account, AdminUser, DisasterRecoveryCommittee, GetDisasterRecoveryAccountsResponse,
    GetDisasterRecoveryCommitteeResponse, SetDisasterRecoveryAccountsInput,
    SetDisasterRecoveryCommitteeInput,
};
use uuid::Uuid;

#[test]
fn successful_disaster_recovery_sync() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let health_status =
        get_core_canister_health_status(&env, WALLET_ADMIN_USER, canister_ids.station);
    assert_eq!(health_status, HealthStatus::Healthy);

    let system_info = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.station);

    let upgrader_id = system_info.upgrader_id;

    let args: SetDisasterRecoveryCommitteeInput = SetDisasterRecoveryCommitteeInput {
        committee: DisasterRecoveryCommittee {
            quorum_percentage: 50,
            users: vec![
                AdminUser {
                    id: Uuid::new_v4().hyphenated().to_string(),
                    name: "user_1".to_owned(),
                    identities: vec![Principal::from_slice(&[0])],
                },
                AdminUser {
                    id: Uuid::new_v4().hyphenated().to_string(),
                    name: "user_2".to_owned(),
                    identities: vec![Principal::from_slice(&[1])],
                },
            ],
        },
    };

    // non-controller can't set disaster recovery committee
    let res: (ApiResult,) = update_candid_as(
        &env,
        upgrader_id,
        Principal::from_slice(&[1]),
        "set_disaster_recovery_committee",
        (args.clone(),),
    )
    .expect("Failed update call to set disaster recovery committee");
    let err = res
        .0
        .expect_err("Non-controller should not be able to set disaster recovery committee");
    assert!(err.code == "NOT_CONTROLLER");

    // controller can set disaster recovery committee
    let res: (ApiResult,) = update_candid_as(
        &env,
        upgrader_id,
        canister_ids.station,
        "set_disaster_recovery_committee",
        (args,),
    )
    .expect("Failed update call to set disaster recovery committee");
    res.0.expect("Failed to set disaster recovery committee");

    let res: (ApiResult<GetDisasterRecoveryCommitteeResponse>,) = query_candid_as(
        &env,
        upgrader_id,
        canister_ids.station,
        "get_disaster_recovery_committee",
        ((),),
    )
    .expect("Failed query call to get disaster recovery committee");

    let maybe_admins = res
        .0
        .expect("Failed to get disaster recovery committee")
        .committee;

    let admins = maybe_admins.expect("No committee found");

    assert_eq!(admins.quorum_percentage, 50);
    assert_eq!(admins.users.len(), 2);
    assert_eq!(admins.users[0].name, "user_1");
    assert_eq!(admins.users[1].name, "user_2");

    let args = SetDisasterRecoveryAccountsInput {
        accounts: vec![
            Account {
                id: Uuid::from_bytes([0; 16]).hyphenated().to_string(),
                blockchain: "icp".to_owned(),
                address: "abc".to_owned(),
                standard: "native".to_owned(),
                symbol: "ICP".to_owned(),
                decimals: 8,
                name: "Main Account".to_owned(),
                metadata: vec![],
            },
            Account {
                id: Uuid::from_bytes([1; 16]).hyphenated().to_string(),
                blockchain: "icp".to_owned(),
                address: "def".to_owned(),
                standard: "native".to_owned(),
                symbol: "ICP".to_owned(),
                decimals: 8,
                name: "Another Account".to_owned(),
                metadata: vec![],
            },
        ],
    };

    // non-controller can't set disaster recovery accounts
    let res: (ApiResult,) = update_candid_as(
        &env,
        upgrader_id,
        Principal::from_slice(&[1]),
        "set_disaster_recovery_accounts",
        (args.clone(),),
    )
    .expect("Failed update call to set disaster recovery accounts");
    let err = res
        .0
        .expect_err("Non-controller should not be able to set disaster recovery accounts");
    assert!(err.code == "NOT_CONTROLLER");

    // controller can set disaster recovery accounts
    let res: (ApiResult,) = update_candid_as(
        &env,
        upgrader_id,
        canister_ids.station,
        "set_disaster_recovery_accounts",
        (args,),
    )
    .expect("Failed update call to set disaster recovery accounts");
    res.0.expect("Failed to set disaster recovery accounts");

    let res: (ApiResult<GetDisasterRecoveryAccountsResponse>,) = query_candid_as(
        &env,
        upgrader_id,
        canister_ids.station,
        "get_disaster_recovery_accounts",
        ((),),
    )
    .expect("Failed query call to get disaster recovery accounts");

    let accounts = res
        .0
        .expect("Failed to get disaster recovery accounts")
        .accounts;

    assert_eq!(accounts.len(), 2);
    assert_eq!(accounts[0].name, "Main Account");
    assert_eq!(accounts[1].name, "Another Account");
}

#[test]
fn auto_syncs_users_after_deploy() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let health_status =
        get_core_canister_health_status(&env, WALLET_ADMIN_USER, canister_ids.station);
    assert_eq!(health_status, HealthStatus::Healthy);

    let system_info = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.station);

    let upgrader_id = system_info.upgrader_id;

    let state = get_upgrader_disaster_recovery(&env, &upgrader_id, &canister_ids.station);

    let committee = state.committee.expect("No committee found");

    assert_eq!(committee.quorum_percentage, 51);
    assert_eq!(committee.users.len(), 1);
    assert_eq!(committee.users[0].name, "station-admin");

    assert!(state.accounts.is_empty());
}

#[test]
fn auto_syncs_on_account_creation() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let system_info = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.station);
    let upgrader_id = system_info.upgrader_id;
    let admin_user = get_user(&env, WALLET_ADMIN_USER, canister_ids.station);

    let state = get_upgrader_disaster_recovery(&env, &upgrader_id, &canister_ids.station);

    assert!(state.accounts.is_empty());

    // create account for admin user
    let add_account = RequestOperationInput::AddAccount(AddAccountOperationInput {
        name: "admin".to_string(),
        blockchain: "icp".to_string(),
        standard: "native".to_string(),
        read_permission: AllowDTO {
            auth_scope: station_api::AuthScopeDTO::Restricted,
            user_groups: vec![],
            users: vec![admin_user.id.clone()],
        },
        configs_permission: AllowDTO {
            auth_scope: station_api::AuthScopeDTO::Restricted,
            user_groups: vec![],
            users: vec![admin_user.id.clone()],
        },
        transfer_permission: AllowDTO {
            auth_scope: station_api::AuthScopeDTO::Restricted,
            user_groups: vec![],
            users: vec![admin_user.id.clone()],
        },
        configs_request_policy: None,
        transfer_request_policy: None,
        metadata: vec![],
    });

    execute_request(&env, WALLET_ADMIN_USER, canister_ids.station, add_account)
        .expect("Failed to create account");

    let state = get_upgrader_disaster_recovery(&env, &upgrader_id, &canister_ids.station);

    assert_eq!(state.accounts.len(), 1);
    assert_eq!(state.accounts[0].name, "admin");
}

/*
- create 2 more admin users
- create 1 user that is not part of the disaster recovery committee
- set disaster recovery committee with quorum of 2
- submit disaster recovery request with 1st user - assert it does not pass yet
- submit non-matching disaster recovery request with 2nd user - assert it does not pass yet still
- submit matching disaster recovery request with 4th user - assert it does not pass yet still
- submit matching disaster recovery request with 3rd user - assert it passes
- assert station is reinstalled with the new wasm
- assert disaster recovery requests are cleared
*/
#[test]
fn test_disaster_recovery_flow() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let system_info = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.station);
    let upgrader_id = system_info.upgrader_id;

    let admin_user = get_user(&env, WALLET_ADMIN_USER, canister_ids.station);

    let user2_identity = user_test_id(1);
    add_user(
        &env,
        user2_identity,
        admin_user.groups.iter().map(|g| g.id.clone()).collect(),
        canister_ids.station,
    );

    let user3_identity = user_test_id(2);
    add_user(
        &env,
        user3_identity,
        admin_user.groups.iter().map(|g| g.id.clone()).collect(),
        canister_ids.station,
    );

    // user 4 can't set disaster recovery committee
    let user4_identity = user_test_id(3);
    add_user(&env, user4_identity, vec![], canister_ids.station);

    set_disaster_recovery(
        &env,
        canister_ids.station,
        SetDisasterRecoveryOperationInput {
            committee: Some(DisasterRecoveryCommitteeDTO {
                quorum_percentage: 51,
                user_group_id: admin_user.groups[0].id.clone(),
            }),
        },
    );

    let new_wasm_module = get_canister_wasm("upgrader");
    let old_wasm_hash = env
        .canister_status(canister_ids.station, Some(upgrader_id))
        .expect("Failed to get canister status")
        .module_hash
        .expect("No module hash found");

    // install the upgrader wasm for the station as a test
    let good_request = upgrader_api::RequestDisasterRecoveryInput {
        module: new_wasm_module.clone(),
        arg: Encode!(&upgrader_api::InitArg {
            target_canister: canister_ids.station
        })
        .unwrap(),
        install_mode: upgrader_api::InstallMode::Reinstall,
    };

    let bad_request = upgrader_api::RequestDisasterRecoveryInput {
        module: new_wasm_module.clone(),
        arg: vec![1, 2, 3],
        install_mode: upgrader_api::InstallMode::Reinstall,
    };

    let res: (ApiResult<()>,) = update_candid_as(
        &env,
        upgrader_id,
        WALLET_ADMIN_USER,
        "request_disaster_recovery",
        (good_request.clone(),),
    )
    .expect("Failed update call to request disaster recovery");
    res.0.expect("Failed to request disaster recovery");

    let dr_status = get_upgrader_disaster_recovery(&env, &upgrader_id, &canister_ids.station);
    assert!(matches!(
        dr_status.recovery_status,
        upgrader_api::RecoveryStatus::Idle
    ));

    let res: (ApiResult<()>,) = update_candid_as(
        &env,
        upgrader_id,
        user2_identity,
        "request_disaster_recovery",
        (bad_request.clone(),),
    )
    .expect("Failed update call to request disaster recovery");
    res.0.expect("Failed to request disaster recovery");

    let dr_status = get_upgrader_disaster_recovery(&env, &upgrader_id, &canister_ids.station);
    assert!(matches!(
        dr_status.recovery_status,
        upgrader_api::RecoveryStatus::Idle
    ));

    assert!(dr_status.last_recovery_result.is_none());

    let res: (ApiResult<()>,) = update_candid_as(
        &env,
        upgrader_id,
        user4_identity,
        "request_disaster_recovery",
        (good_request.clone(),),
    )
    .expect("Failed update call to request disaster recovery");
    res.0
        .expect_err("User 4 should not be able to request disaster recovery");

    let dr_status = get_upgrader_disaster_recovery(&env, &upgrader_id, &canister_ids.station);
    assert!(matches!(
        dr_status.recovery_status,
        upgrader_api::RecoveryStatus::Idle
    ));

    assert!(dr_status.last_recovery_result.is_none());

    let res: (ApiResult<()>,) = update_candid_as(
        &env,
        upgrader_id,
        user3_identity,
        "request_disaster_recovery",
        (good_request.clone(),),
    )
    .expect("Failed update call to request disaster recovery");
    res.0.expect("Failed to request disaster recovery");

    let dr_status = get_upgrader_disaster_recovery(&env, &upgrader_id, &canister_ids.station);
    assert!(matches!(
        dr_status.recovery_status,
        upgrader_api::RecoveryStatus::InProgress { .. }
    ));

    env.tick();
    env.tick();
    env.tick();
    env.tick();
    env.tick();
    env.tick();
    env.tick();

    let dr_status = get_upgrader_disaster_recovery(&env, &upgrader_id, &canister_ids.station);
    assert!(matches!(
        dr_status.recovery_status,
        upgrader_api::RecoveryStatus::Idle
    ));

    assert!(matches!(
        dr_status.last_recovery_result,
        Some(upgrader_api::RecoveryResult::Success)
    ));

    let new_wasm_hash = env
        .canister_status(canister_ids.station, Some(upgrader_id))
        .expect("Failed to get canister status")
        .module_hash
        .expect("No module hash found");

    assert_ne!(old_wasm_hash, new_wasm_hash);

    assert_eq!(new_wasm_hash, sha256_hash(&new_wasm_module));

    let dr_status = get_upgrader_disaster_recovery(&env, &upgrader_id, &canister_ids.station);

    assert!(dr_status.recovery_requests.is_empty());
}

#[test]
fn test_disaster_recovery_in_progress() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let system_info = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.station);
    let upgrader_id = system_info.upgrader_id;

    let new_wasm_module = get_canister_wasm("upgrader");

    // install the upgrader wasm for the station as a test
    let good_request = upgrader_api::RequestDisasterRecoveryInput {
        module: new_wasm_module.clone(),
        arg: Encode!(&upgrader_api::InitArg {
            target_canister: canister_ids.station
        })
        .unwrap(),
        install_mode: upgrader_api::InstallMode::Reinstall,
    };

    let res: (ApiResult<()>,) = update_candid_as(
        &env,
        upgrader_id,
        WALLET_ADMIN_USER,
        "request_disaster_recovery",
        (good_request.clone(),),
    )
    .expect("Failed update call to request disaster recovery");
    res.0.expect("Failed to request disaster recovery");

    let dr_status = get_upgrader_disaster_recovery(&env, &upgrader_id, &canister_ids.station);
    assert!(matches!(
        dr_status.recovery_status,
        upgrader_api::RecoveryStatus::InProgress { .. }
    ));

    let res: (ApiResult<()>,) = update_candid_as(
        &env,
        upgrader_id,
        WALLET_ADMIN_USER,
        "request_disaster_recovery",
        (good_request.clone(),),
    )
    .expect("Failed update call to request disaster recovery");
    res.0.expect("Failed to request disaster recovery");

    let dr_status = get_upgrader_disaster_recovery(&env, &upgrader_id, &canister_ids.station);
    assert!(matches!(
        dr_status.recovery_status,
        upgrader_api::RecoveryStatus::InProgress { .. }
    ));

    let logs_result = get_upgrader_logs(&env, &upgrader_id, &WALLET_ADMIN_USER);

    // assert that the 2nd request is not processed because the first one is still in progress
    assert_eq!(
        logs_result.logs.first().expect("No logs found").entry_type,
        "disaster_recovery_in_progress".to_owned()
    );
}

// Test disaster recovery Install canister by waiting for the station to run out cycles and then doing DR Install
#[test]
fn test_disaster_recovery_install() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env_with_config(crate::setup::SetupConfig {
        start_cycles: Some(1_000_000_000_000),
        ..Default::default()
    });

    let system_info = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.station);
    let upgrader_id = system_info.upgrader_id;

    // top the upgrader up so that it wont run out of cycles
    env.add_cycles(upgrader_id, 100_000_000_000_000);

    // stop the upgrader canister to prevent topping up the station
    env.stop_canister(upgrader_id, Some(NNS_ROOT_CANISTER_ID))
        .expect("Failed to stop canister");

    // burn all cycles in the station
    advance_time_to_burn_cycles(&env, NNS_ROOT_CANISTER_ID, canister_ids.station, 0);

    // start the upgrader canister
    env.start_canister(upgrader_id, Some(NNS_ROOT_CANISTER_ID))
        .expect("Failed to start canister");

    env.tick();

    env.add_cycles(canister_ids.station, 3_000_000_000_000);

    // the station should be wiped by now
    assert!(env
        .canister_status(canister_ids.station, Some(upgrader_id))
        .expect("Failed to get canister status")
        .module_hash
        .is_none());

    // install the upgrader wasm for the station as a test
    let good_request = upgrader_api::RequestDisasterRecoveryInput {
        module: get_canister_wasm("upgrader"),
        arg: Encode!(&upgrader_api::InitArg {
            target_canister: canister_ids.station
        })
        .unwrap(),
        install_mode: upgrader_api::InstallMode::Install,
    };

    let res: (ApiResult<()>,) = update_candid_as(
        &env,
        upgrader_id,
        WALLET_ADMIN_USER,
        "request_disaster_recovery",
        (good_request.clone(),),
    )
    .expect("Failed update call to request disaster recovery");
    res.0.expect("Failed to request disaster recovery");

    env.tick();
    env.tick();
    env.tick();

    let dr_status = get_upgrader_disaster_recovery(&env, &upgrader_id, &canister_ids.station);

    assert!(matches!(
        dr_status.last_recovery_result,
        Some(upgrader_api::RecoveryResult::Success)
    ));
}

#[test]
fn test_disaster_recovery_upgrade() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let system_info = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.station);
    let upgrader_id = system_info.upgrader_id;

    let station_init_arg = SystemInstall::Upgrade(SystemUpgrade { name: None });
    let good_request = upgrader_api::RequestDisasterRecoveryInput {
        module: get_canister_wasm("station"),
        arg: Encode!(&station_init_arg).unwrap(),
        install_mode: upgrader_api::InstallMode::Upgrade,
    };

    let res: (ApiResult<()>,) = update_candid_as(
        &env,
        upgrader_id,
        WALLET_ADMIN_USER,
        "request_disaster_recovery",
        (good_request.clone(),),
    )
    .expect("Failed update call to request disaster recovery");
    res.0.expect("Failed to request disaster recovery");

    env.tick();
    env.tick();
    env.tick();
    env.tick();
    env.tick();
    env.tick();
    env.tick();

    let dr_status = get_upgrader_disaster_recovery(&env, &upgrader_id, &canister_ids.station);

    assert!(matches!(
        dr_status.last_recovery_result,
        Some(upgrader_api::RecoveryResult::Success)
    ));
}

#[test]
fn test_disaster_recovery_failing() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let system_info = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.station);
    let upgrader_id = system_info.upgrader_id;

    // intentionally bad arg to fail Upgrade
    let arg = SystemInstall::Init(SystemInit {
        fallback_controller: None,
        quorum_percentage: None,
        upgrader_wasm_module: vec![],
        name: "Station".to_string(),
        admins: vec![],
    });

    // install with intentionally bad arg to fail
    let good_request = upgrader_api::RequestDisasterRecoveryInput {
        module: get_canister_wasm("station"),
        arg: Encode!(&arg).unwrap(),
        install_mode: upgrader_api::InstallMode::Upgrade,
    };

    let res: (ApiResult<()>,) = update_candid_as(
        &env,
        upgrader_id,
        WALLET_ADMIN_USER,
        "request_disaster_recovery",
        (good_request.clone(),),
    )
    .expect("Failed update call to request disaster recovery");
    res.0.expect("Failed to request disaster recovery");

    env.tick();
    env.tick();
    env.tick();
    env.tick();
    env.tick();
    env.tick();

    let dr_status = get_upgrader_disaster_recovery(&env, &upgrader_id, &canister_ids.station);

    assert!(matches!(
        dr_status.recovery_status,
        upgrader_api::RecoveryStatus::Idle
    ));
    assert!(matches!(
        dr_status.last_recovery_result,
        Some(upgrader_api::RecoveryResult::Failure(_))
    ));
}
