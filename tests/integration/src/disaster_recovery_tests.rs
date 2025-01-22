use crate::setup::{
    get_canister_wasm, setup_new_env, setup_new_env_with_config, WALLET_ADMIN_USER,
};
use crate::utils::{
    add_user, advance_time_to_burn_cycles, await_station_healthy, execute_request,
    get_account_read_permission, get_account_transfer_permission, get_account_update_permission,
    get_core_canister_health_status, get_disaster_recovery_accounts,
    get_disaster_recovery_accounts_and_assets, get_disaster_recovery_committee,
    get_disaster_recovery_state, get_icp_asset, get_system_info, get_upgrader_disaster_recovery,
    get_upgrader_logs, get_user, request_disaster_recovery, set_disaster_recovery,
    set_disaster_recovery_accounts, set_disaster_recovery_accounts_and_assets,
    set_disaster_recovery_committee, upload_canister_chunks_to_asset_canister, user_test_id,
    NNS_ROOT_CANISTER_ID,
};
use crate::TestEnv;
use candid::{CandidType, Encode, Principal};
use orbit_essentials::api::ApiResult;
use orbit_essentials::utils::sha256_hash;
use pocket_ic::{query_candid_as, update_candid_as, PocketIc};
use serde::Deserialize;
use station_api::{
    AccountDTO, AddAccountOperationInput, AllowDTO, DisasterRecoveryCommitteeDTO, HealthStatus,
    ListAccountsResponse, RequestOperationDTO, RequestOperationInput, RequestPolicyRuleDTO,
    SetDisasterRecoveryOperationInput, SystemInit, SystemInstall, SystemUpgrade,
};
use std::collections::BTreeMap;
use std::str::FromStr;
use upgrader_api::{
    Account, AdminUser, Asset, DisasterRecoveryCommittee, MultiAssetAccount, RecoveryResult,
    RecoveryStatus, StationRecoveryRequest,
};
use uuid::Uuid;

fn await_disaster_recovery_success(env: &PocketIc, station_id: Principal, upgrader_id: Principal) {
    let max_rounds = 100;
    for _ in 0..max_rounds {
        env.tick();

        let dr_status = get_upgrader_disaster_recovery(env, &upgrader_id, &station_id);

        if matches!(
            dr_status.recovery_status,
            upgrader_api::RecoveryStatus::Idle
        ) && matches!(
            dr_status.last_recovery_result,
            Some(upgrader_api::RecoveryResult::Success)
        ) {
            return;
        }
    }
    panic!(
        "Disaster recovery did not succeed within {} rounds.",
        max_rounds
    );
}

fn await_disaster_recovery_failure(env: &PocketIc, station_id: Principal, upgrader_id: Principal) {
    let max_rounds = 100;
    for _ in 0..max_rounds {
        env.tick();

        let dr_status = get_upgrader_disaster_recovery(env, &upgrader_id, &station_id);

        if matches!(
            dr_status.recovery_status,
            upgrader_api::RecoveryStatus::Idle
        ) && matches!(
            dr_status.last_recovery_result,
            Some(upgrader_api::RecoveryResult::Failure(_))
        ) {
            return;
        }
    }
    panic!(
        "Disaster recovery did not fail within {} rounds.",
        max_rounds
    );
}

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

    let quorum = 1;
    let users = vec![
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
    ];
    let committee = DisasterRecoveryCommittee { quorum, users };

    // non-controller can't set disaster recovery committee
    let err = set_disaster_recovery_committee(
        &env,
        upgrader_id,
        Principal::from_slice(&[1]),
        committee.clone(),
    )
    .expect_err("Non-controller should not be able to set disaster recovery committee");
    assert!(err.code == "NOT_CONTROLLER");

    // controller can set disaster recovery committee
    set_disaster_recovery_committee(&env, upgrader_id, canister_ids.station, committee)
        .expect("Failed to set disaster recovery committee");

    let admins = get_disaster_recovery_committee(&env, upgrader_id, canister_ids.station)
        .expect("No committee found");

    assert_eq!(admins.quorum, 1);
    assert_eq!(admins.users.len(), 2);
    assert_eq!(admins.users[0].name, "user_1");
    assert_eq!(admins.users[1].name, "user_2");

    let icp_asset_id = Uuid::from_bytes([0; 16]).hyphenated().to_string();

    let accounts = vec![
        MultiAssetAccount {
            id: Uuid::from_bytes([0; 16]).hyphenated().to_string(),
            name: "Main Account".to_owned(),
            metadata: vec![],
            assets: vec![icp_asset_id.clone()],
            seed: [0; 16],
        },
        MultiAssetAccount {
            id: Uuid::from_bytes([1; 16]).hyphenated().to_string(),
            name: "Another Account".to_owned(),
            metadata: vec![],
            assets: vec![icp_asset_id.clone()],
            seed: [1; 16],
        },
    ];
    let assets = vec![Asset {
        blockchain: "icp".to_owned(),
        id: Uuid::from_bytes([0; 16]).hyphenated().to_string(),
        name: "Internet Computer".to_owned(),
        symbol: "ICP".to_owned(),
        decimals: 8,
        metadata: vec![],
        standards: vec!["icp_native".to_owned()],
    }];

    // non-controller can't set disaster recovery accounts
    let err = set_disaster_recovery_accounts_and_assets(
        &env,
        upgrader_id,
        Principal::from_slice(&[1]),
        accounts.clone(),
        assets.clone(),
    )
    .expect_err("Non-controller should not be able to set disaster recovery accounts");
    assert!(err.code == "NOT_CONTROLLER");

    // controller can set disaster recovery accounts
    set_disaster_recovery_accounts_and_assets(
        &env,
        upgrader_id,
        canister_ids.station,
        accounts,
        assets,
    )
    .expect("Failed to set disaster recovery accounts");

    let (accounts, _) =
        get_disaster_recovery_accounts_and_assets(&env, upgrader_id, canister_ids.station);

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

    assert_eq!(committee.quorum, 1);
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

    let icp_asset = get_icp_asset(&env, canister_ids.station, WALLET_ADMIN_USER);

    // create account for admin user
    let add_account = RequestOperationInput::AddAccount(AddAccountOperationInput {
        name: "admin".to_string(),
        assets: vec![icp_asset.id],
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

    assert_eq!(state.multi_asset_accounts.len(), 1);
    assert_eq!(state.multi_asset_accounts[0].name, "admin");
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
                quorum: 2,
                user_group_id: admin_user.groups[0].id.clone(),
            }),
        },
    );

    let new_wasm_module = get_canister_wasm("upgrader");
    let (base_chunk, module_extra_chunks) =
        upload_canister_chunks_to_asset_canister(&env, new_wasm_module.clone(), 50_000);
    let old_wasm_hash = env
        .canister_status(canister_ids.station, Some(upgrader_id))
        .expect("Failed to get canister status")
        .module_hash
        .expect("No module hash found");

    // install the upgrader wasm for the station as a test
    let good_request = upgrader_api::RequestDisasterRecoveryInput::InstallCode(
        upgrader_api::RequestDisasterRecoveryInstallCodeInput {
            install_mode: upgrader_api::InstallMode::Reinstall,
            module: base_chunk.clone(),
            module_extra_chunks: Some(module_extra_chunks.clone()),
            arg: Encode!(&upgrader_api::InitArg {
                target_canister: canister_ids.station
            })
            .unwrap(),
        },
    );

    let bad_request = upgrader_api::RequestDisasterRecoveryInput::InstallCode(
        upgrader_api::RequestDisasterRecoveryInstallCodeInput {
            module: base_chunk,
            module_extra_chunks: Some(module_extra_chunks),
            arg: vec![1, 2, 3],
            install_mode: upgrader_api::InstallMode::Reinstall,
        },
    );

    request_disaster_recovery(&env, upgrader_id, WALLET_ADMIN_USER, good_request.clone())
        .expect("Failed to request disaster recovery");

    let dr_status = get_upgrader_disaster_recovery(&env, &upgrader_id, &canister_ids.station);
    assert!(matches!(
        dr_status.recovery_status,
        upgrader_api::RecoveryStatus::Idle
    ));

    request_disaster_recovery(&env, upgrader_id, user2_identity, bad_request.clone())
        .expect("Failed to request disaster recovery");

    let dr_status = get_upgrader_disaster_recovery(&env, &upgrader_id, &canister_ids.station);
    assert!(matches!(
        dr_status.recovery_status,
        upgrader_api::RecoveryStatus::Idle
    ));

    assert!(dr_status.last_recovery_result.is_none());

    request_disaster_recovery(&env, upgrader_id, user4_identity, good_request.clone())
        .expect_err("User 4 should not be able to request disaster recovery");

    let dr_status = get_upgrader_disaster_recovery(&env, &upgrader_id, &canister_ids.station);
    assert!(matches!(
        dr_status.recovery_status,
        upgrader_api::RecoveryStatus::Idle
    ));

    assert!(dr_status.last_recovery_result.is_none());

    request_disaster_recovery(&env, upgrader_id, user3_identity, good_request.clone())
        .expect("Failed to request disaster recovery");

    let dr_status = get_upgrader_disaster_recovery(&env, &upgrader_id, &canister_ids.station);
    assert!(matches!(
        dr_status.recovery_status,
        upgrader_api::RecoveryStatus::InProgress { .. }
    ));

    await_disaster_recovery_success(&env, canister_ids.station, upgrader_id);

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

// The goal of this test is not to check if the committee is set correctly, but to check that when the station is
// reinstalled, the accounts are also recreated with the same addresses.
#[test]
fn test_disaster_recovery_flow_recreates_same_accounts() {
    // 1. setup the environment with one admin user
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let station_wasm_module = get_canister_wasm("station");
    let system_info = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.station);
    let upgrader_id = system_info.upgrader_id;
    let admin_user = get_user(&env, WALLET_ADMIN_USER, canister_ids.station);

    let icp_asset = get_icp_asset(&env, canister_ids.station, WALLET_ADMIN_USER);

    // 2. create 3 accounts with the same admin user and no approval required
    let mut initial_accounts = BTreeMap::new();
    for account_nr in 0..3 {
        let create_account_args = AddAccountOperationInput {
            name: format!("account-{}", account_nr),
            assets: vec![icp_asset.id.clone()],
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
            transfer_request_policy: Some(RequestPolicyRuleDTO::AutoApproved),
            configs_request_policy: Some(RequestPolicyRuleDTO::AutoApproved),
            metadata: vec![station_api::MetadataDTO {
                key: "key".to_string(),
                value: "value".to_string(),
            }],
        };

        let request = execute_request(
            &env,
            WALLET_ADMIN_USER,
            canister_ids.station,
            RequestOperationInput::AddAccount(create_account_args),
        )
        .expect("Unexpected failed to create account");

        if let RequestOperationDTO::AddAccount(operation) = request.operation {
            let newly_added_account = operation
                .account
                .expect("Unexpected new account not available");

            initial_accounts.insert(newly_added_account.id.clone(), newly_added_account);
        } else {
            panic!("Unexpected request operation found");
        }
    }

    let init_assets_input = station_api::InitAssetInput {
        id: icp_asset.id.clone(),
        name: icp_asset.name.clone(),
        symbol: icp_asset.symbol.clone(),
        decimals: icp_asset.decimals,
        blockchain: icp_asset.blockchain.clone(),
        standards: icp_asset.standards.clone(),
        metadata: vec![],
    };

    // 3. perform a reinstall disaster recovery request
    let init_accounts_input = initial_accounts
        .iter()
        .map(
            |(id, AccountDTO { name, .. })| station_api::InitAccountInput {
                id: Some(id.clone()),
                name: name.to_string(),
                metadata: vec![],
                assets: vec![icp_asset.id.clone()],
                seed: Uuid::from_str(id.as_str())
                    .expect("Failed to parse uuid")
                    .as_bytes()
                    .to_owned(),
            },
        )
        .collect();

    let (base_chunk, module_extra_chunks) =
        upload_canister_chunks_to_asset_canister(&env, station_wasm_module, 500_000);
    request_disaster_recovery(
        &env,
        upgrader_id,
        WALLET_ADMIN_USER,
        upgrader_api::RequestDisasterRecoveryInput::InstallCode(
            upgrader_api::RequestDisasterRecoveryInstallCodeInput {
                module: base_chunk,
                module_extra_chunks: Some(module_extra_chunks),
                arg: Encode!(&station_api::SystemInstall::Init(station_api::SystemInit {
                    name: "Station".to_string(),
                    admins: vec![
                        station_api::AdminInitInput {
                            identity: WALLET_ADMIN_USER,
                            name: "updated-admin-name".to_string(),
                        },
                        station_api::AdminInitInput {
                            identity: Principal::from_slice(&[95; 29]),
                            name: "another-admin".to_string(),
                        },
                        station_api::AdminInitInput {
                            identity: Principal::from_slice(&[97; 29]),
                            name: "yet-another-admin".to_string(),
                        }
                    ],
                    quorum: None,
                    fallback_controller: None,
                    upgrader: station_api::SystemUpgraderInput::Id(upgrader_id),
                    accounts: Some(init_accounts_input),
                    assets: Some(vec![init_assets_input]),
                }))
                .unwrap(),
                install_mode: upgrader_api::InstallMode::Reinstall,
            },
        ),
    )
    .expect("Unexpected failed to request disaster recovery");

    let dr_status = get_upgrader_disaster_recovery(&env, &upgrader_id, &canister_ids.station);
    assert!(matches!(
        dr_status.recovery_status,
        upgrader_api::RecoveryStatus::InProgress { .. }
    ));

    await_disaster_recovery_success(&env, canister_ids.station, upgrader_id);
    await_station_healthy(&env, canister_ids.station, WALLET_ADMIN_USER);

    // 4. assert that the station is reinstalled with the same accounts and the apporoval policies are set correctly
    let res: (ApiResult<ListAccountsResponse>,) = update_candid_as(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        "list_accounts",
        (station_api::ListAccountsInput {
            search_term: None,
            paginate: None,
        },),
    )
    .expect("Unexpected failed update call to list accounts");
    let existing_accounts = res.0.expect("Unexpected failed to list accounts").accounts;

    assert_eq!(existing_accounts.len(), initial_accounts.len());

    fn assert_expected_approval_quorum(policy: &Option<RequestPolicyRuleDTO>, group_id: String) {
        let policy = policy.as_ref().expect("No policy found");

        match policy {
            RequestPolicyRuleDTO::Quorum(quorum) => {
                assert_eq!(quorum.min_approved, 2);
                match &quorum.approvers {
                    station_api::UserSpecifierDTO::Group(groups) => {
                        assert_eq!(groups.len(), 1);
                        assert_eq!(groups[0], group_id);
                    }
                    _ => {
                        panic!("Unexpected approvers found");
                    }
                }
            }
            _ => {
                panic!("Unexpected request policy found");
            }
        }
    }

    fn assert_expected_account_permissions(allow: &AllowDTO, group_id: String) {
        assert_eq!(allow.users.len(), 0);
        assert_eq!(allow.user_groups.len(), 1);
        assert_eq!(allow.user_groups[0], group_id);

        if let station_api::AuthScopeDTO::Restricted = allow.auth_scope {
        } else {
            panic!("Unexpected auth scope found");
        }
    }

    assert_eq!(admin_user.groups.len(), 1);
    let admin_user_group = admin_user.groups.first().expect("No user group found");

    for (id, initial_account) in initial_accounts {
        let account = existing_accounts
            .iter()
            .find(|a| a.id == id)
            .expect("Unexpected account not found");

        assert_eq!(account.name, initial_account.name);

        for account_address in initial_account.addresses.iter() {
            assert!(account.addresses.iter().any(|recovered_account_address| {
                recovered_account_address.address == account_address.address
            }));
        }

        account.metadata.iter().for_each(|m| {
            assert_eq!(m.key, "key");
            assert_eq!(m.value, "value");
        });

        assert_expected_account_permissions(
            &get_account_read_permission(
                &env,
                WALLET_ADMIN_USER,
                canister_ids.station,
                account.id.clone(),
            ),
            admin_user_group.id.clone(),
        );

        assert_expected_account_permissions(
            &get_account_update_permission(
                &env,
                WALLET_ADMIN_USER,
                canister_ids.station,
                account.id.clone(),
            ),
            admin_user_group.id.clone(),
        );

        assert_expected_account_permissions(
            &get_account_transfer_permission(
                &env,
                WALLET_ADMIN_USER,
                canister_ids.station,
                account.id.clone(),
            ),
            admin_user_group.id.clone(),
        );

        assert_expected_approval_quorum(
            &account.transfer_request_policy,
            admin_user_group.id.clone(),
        );
        assert_expected_approval_quorum(
            &account.configs_request_policy,
            admin_user_group.id.clone(),
        );
    }
}

// The goal of this test is not to check if the committee is set correctly, but to check that when the station is
// reinstalled, the upgrader canister is not recreated.
#[test]
fn test_disaster_recovery_flow_reuses_same_upgrader() {
    // 1. setup the environment with one admin user
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let station_wasm_module = get_canister_wasm("station");
    let system_info = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.station);
    let upgrader_id = system_info.upgrader_id;
    let initial_fallback_controller = Principal::from_slice(&[50; 29]);
    let fallback_controller = Principal::from_slice(&[99; 29]);

    env.set_controllers(
        canister_ids.station,
        Some(upgrader_id),
        vec![initial_fallback_controller, upgrader_id],
    )
    .expect("Unexpected failed to set controllers of the station canister");

    // 2. perform the disaster recovery request with the station wasm and using the same upgrader id
    let (base_chunk, module_extra_chunks) =
        upload_canister_chunks_to_asset_canister(&env, station_wasm_module.clone(), 500_000);
    request_disaster_recovery(
        &env,
        upgrader_id,
        WALLET_ADMIN_USER,
        upgrader_api::RequestDisasterRecoveryInput::InstallCode(
            upgrader_api::RequestDisasterRecoveryInstallCodeInput {
                module: base_chunk,
                module_extra_chunks: Some(module_extra_chunks),
                arg: Encode!(&station_api::SystemInstall::Init(station_api::SystemInit {
                    name: "Station".to_string(),
                    admins: vec![station_api::AdminInitInput {
                        identity: WALLET_ADMIN_USER,
                        name: "updated-admin-name".to_string(),
                    }],
                    quorum: None,
                    fallback_controller: Some(fallback_controller),
                    upgrader: station_api::SystemUpgraderInput::Id(upgrader_id),
                    accounts: None,
                    assets: None,
                }))
                .unwrap(),
                install_mode: upgrader_api::InstallMode::Reinstall,
            },
        ),
    )
    .expect("Unexpected failed to request disaster recovery");

    let dr_status = get_upgrader_disaster_recovery(&env, &upgrader_id, &canister_ids.station);
    assert!(matches!(
        dr_status.recovery_status,
        upgrader_api::RecoveryStatus::InProgress { .. }
    ));

    await_disaster_recovery_success(&env, canister_ids.station, upgrader_id);

    // 3. assert that the upgrader id is the same as the one used in the disaster recovery request
    let system_info = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.station);

    assert_eq!(system_info.upgrader_id, upgrader_id);

    // 4. assert that the station is reinstalled with the same wasm and the user has the new updated name
    let new_wasm_hash = env
        .canister_status(canister_ids.station, Some(upgrader_id))
        .expect("Failed to get canister status")
        .module_hash
        .expect("No module hash found");

    assert_eq!(new_wasm_hash, sha256_hash(&station_wasm_module));

    let admin_user = get_user(&env, WALLET_ADMIN_USER, canister_ids.station);
    assert_eq!(admin_user.name, "updated-admin-name");

    // 5. assert that the fallback controller is updated
    let updated_controllers = env
        .canister_status(canister_ids.station, Some(upgrader_id))
        .expect("Failed to get canister status")
        .settings
        .controllers;

    assert_eq!(updated_controllers.len(), 2);
    assert!(updated_controllers.contains(&fallback_controller));
    assert!(updated_controllers.contains(&upgrader_id));

    // 6. check that the upgrader has the new fallback controller
    let upgrader_controllers = env
        .canister_status(upgrader_id, Some(canister_ids.station))
        .expect("Failed to get canister status")
        .settings
        .controllers;

    assert_eq!(upgrader_controllers.len(), 2);
    assert!(upgrader_controllers.contains(&fallback_controller));
    assert!(upgrader_controllers.contains(&canister_ids.station));
}

#[test]
fn test_disaster_recovery_in_progress() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let system_info = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.station);
    let upgrader_id = system_info.upgrader_id;

    let new_wasm_module = get_canister_wasm("upgrader");
    let (base_chunk, module_extra_chunks) =
        upload_canister_chunks_to_asset_canister(&env, new_wasm_module, 50_000);

    // install the upgrader wasm for the station as a test
    let good_request = upgrader_api::RequestDisasterRecoveryInput::InstallCode(
        upgrader_api::RequestDisasterRecoveryInstallCodeInput {
            module: base_chunk,
            module_extra_chunks: Some(module_extra_chunks),
            arg: Encode!(&upgrader_api::InitArg {
                target_canister: canister_ids.station
            })
            .unwrap(),
            install_mode: upgrader_api::InstallMode::Reinstall,
        },
    );

    request_disaster_recovery(&env, upgrader_id, WALLET_ADMIN_USER, good_request.clone())
        .expect("Failed to request disaster recovery");

    let dr_status = get_upgrader_disaster_recovery(&env, &upgrader_id, &canister_ids.station);
    assert!(matches!(
        dr_status.recovery_status,
        upgrader_api::RecoveryStatus::InProgress { .. }
    ));

    request_disaster_recovery(&env, upgrader_id, WALLET_ADMIN_USER, good_request.clone())
        .expect("Failed to request disaster recovery");

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
        start_cycles: Some(10_000_000_000_000),
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
    let new_wasm_module = get_canister_wasm("upgrader");
    let (base_chunk, module_extra_chunks) =
        upload_canister_chunks_to_asset_canister(&env, new_wasm_module, 50_000);
    let good_request = upgrader_api::RequestDisasterRecoveryInput::InstallCode(
        upgrader_api::RequestDisasterRecoveryInstallCodeInput {
            module: base_chunk,
            module_extra_chunks: Some(module_extra_chunks),
            arg: Encode!(&upgrader_api::InitArg {
                target_canister: canister_ids.station
            })
            .unwrap(),
            install_mode: upgrader_api::InstallMode::Install,
        },
    );

    request_disaster_recovery(&env, upgrader_id, WALLET_ADMIN_USER, good_request.clone())
        .expect("Failed to request disaster recovery");

    await_disaster_recovery_success(&env, canister_ids.station, upgrader_id);
}

#[test]
fn test_disaster_recovery_upgrade() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let system_info = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.station);
    let upgrader_id = system_info.upgrader_id;

    let station_init_arg = SystemInstall::Upgrade(SystemUpgrade { name: None });
    let new_wasm_module = get_canister_wasm("station");
    let (base_chunk, module_extra_chunks) =
        upload_canister_chunks_to_asset_canister(&env, new_wasm_module, 500_000);
    let good_request = upgrader_api::RequestDisasterRecoveryInput::InstallCode(
        upgrader_api::RequestDisasterRecoveryInstallCodeInput {
            module: base_chunk,
            module_extra_chunks: Some(module_extra_chunks),
            arg: Encode!(&station_init_arg).unwrap(),
            install_mode: upgrader_api::InstallMode::Upgrade,
        },
    );

    request_disaster_recovery(&env, upgrader_id, WALLET_ADMIN_USER, good_request.clone())
        .expect("Failed to request disaster recovery");

    await_disaster_recovery_success(&env, canister_ids.station, upgrader_id);
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
        quorum: None,
        upgrader: station_api::SystemUpgraderInput::Deploy(
            station_api::DeploySystemUpgraderInput {
                wasm_module: vec![],
                initial_cycles: 0,
            },
        ),
        name: "Station".to_string(),
        admins: vec![],
        accounts: None,
        assets: None,
    });

    // install with intentionally bad arg to fail
    let new_wasm_module = get_canister_wasm("station");
    let (base_chunk, module_extra_chunks) =
        upload_canister_chunks_to_asset_canister(&env, new_wasm_module, 500_000);
    let good_request = upgrader_api::RequestDisasterRecoveryInput::InstallCode(
        upgrader_api::RequestDisasterRecoveryInstallCodeInput {
            module: base_chunk,
            module_extra_chunks: Some(module_extra_chunks),
            arg: Encode!(&arg).unwrap(),
            install_mode: upgrader_api::InstallMode::Upgrade,
        },
    );

    request_disaster_recovery(&env, upgrader_id, WALLET_ADMIN_USER, good_request.clone())
        .expect("Failed to request disaster recovery");

    await_disaster_recovery_failure(&env, canister_ids.station, upgrader_id);
}

#[test]
fn test_disaster_recovery_supports_legacy_format() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let system_info = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.station);
    let upgrader_id = system_info.upgrader_id;

    let accounts = vec![
        Account {
            id: Uuid::from_bytes([0; 16]).hyphenated().to_string(),
            name: "Main Account".to_owned(),
            metadata: vec![],
            blockchain: "icp".to_owned(),
            address: "1".to_owned(),
            standard: "icp_native".to_owned(),
            symbol: "ICP1".to_owned(),
            decimals: 8,
        },
        Account {
            id: Uuid::from_bytes([1; 16]).hyphenated().to_string(),
            name: "Another Account".to_owned(),
            metadata: vec![],
            blockchain: "icp".to_owned(),
            address: "2".to_owned(),
            standard: "icp_native".to_owned(),
            symbol: "ICP2".to_owned(),
            decimals: 8,
        },
    ];

    set_disaster_recovery_accounts(&env, upgrader_id, canister_ids.station, accounts)
        .expect("Failed to set disaster recovery accounts");

    let actual_accounts = get_disaster_recovery_accounts(&env, upgrader_id, canister_ids.station);

    assert!(actual_accounts.len() == 2);
    assert_eq!(actual_accounts[0].name, "Main Account");
    assert_eq!(actual_accounts[0].address, "1");

    assert_eq!(actual_accounts[1].name, "Another Account");
    assert_eq!(actual_accounts[1].address, "2");

    // old response format should deserialize correctly
    #[derive(Clone, Debug, CandidType, Deserialize)]
    pub struct GetDisasterRecoveryStateResponse {
        pub committee: Option<DisasterRecoveryCommittee>,
        pub accounts: Vec<Account>,

        pub recovery_requests: Vec<StationRecoveryRequest>,
        pub recovery_status: RecoveryStatus,
        pub last_recovery_result: Option<RecoveryResult>,
    }

    let res: (ApiResult<GetDisasterRecoveryStateResponse>,) = query_candid_as(
        &env,
        upgrader_id,
        canister_ids.station,
        "get_disaster_recovery_state",
        ((),),
    )
    .expect("Failed query call to get disaster recovery accounts");

    let res = res.0.expect("Failed to get disaster recovery accounts");

    assert!(res.accounts.len() == 2);
}

#[test]
fn test_disaster_recovery_committee_change_with_open_requests() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let upgrader_id = get_system_info(&env, WALLET_ADMIN_USER, canister_ids.station).upgrader_id;

    let users: Vec<_> = (0..5)
        .map(|i: u64| AdminUser {
            id: Uuid::from_u128(i.into()).hyphenated().to_string(),
            name: format!("user_{}", i),
            identities: vec![Principal::from_slice(&i.to_le_bytes())],
        })
        .collect();
    let request = upgrader_api::RequestDisasterRecoveryInput::InstallCode(
        upgrader_api::RequestDisasterRecoveryInstallCodeInput {
            module: vec![],
            module_extra_chunks: None,
            arg: vec![],
            install_mode: upgrader_api::InstallMode::Reinstall,
        },
    );
    let disaster_recovery = |i: usize| {
        request_disaster_recovery(
            &env,
            upgrader_id,
            *users[i].identities.first().unwrap(),
            request.clone(),
        )
        .unwrap();
    };

    // set committee to be {u0, u1, u2, u3}, quorum=3
    let quorum = 3;
    let committee = DisasterRecoveryCommittee {
        quorum,
        users: users[..=3].to_vec(),
    };
    set_disaster_recovery_committee(&env, upgrader_id, canister_ids.station, committee.clone())
        .unwrap();

    // users u0, u1 make disaster recovery request
    disaster_recovery(0);
    disaster_recovery(1);

    // user u0 leaves the committee: set committee to be {u1, u2, u3}, quorum=3
    let quorum = 3;
    let committee = DisasterRecoveryCommittee {
        quorum,
        users: users[1..=3].to_vec(),
    };
    set_disaster_recovery_committee(&env, upgrader_id, canister_ids.station, committee.clone())
        .unwrap();

    // user u2 makes disaster recovery request
    disaster_recovery(2);

    // disaster recovery should not be triggered as only users u1, u2 (less than quorum=3)
    // from the current committee submitted disaster recovery requests,
    // but the requests from the current committee should be retained
    let some_committee_member = *users[3].identities.first().unwrap();
    let state = get_disaster_recovery_state(&env, upgrader_id, some_committee_member);
    assert_eq!(state.recovery_requests.len(), 2);
    state
        .recovery_requests
        .iter()
        .all(|request| request.user_id == users[1].id || request.user_id == users[2].id);
    assert_eq!(state.recovery_status, upgrader_api::RecoveryStatus::Idle);
    assert!(state.last_recovery_result.is_none());
}
