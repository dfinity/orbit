use crate::setup::{setup_new_env, WALLET_ADMIN_USER};
use crate::utils::{
    execute_request, get_core_canister_health_status, get_system_info,
    get_upgrader_disaster_recovery, get_user,
};
use crate::TestEnv;
use candid::Principal;
use orbit_essentials::api::ApiResult;
use pocket_ic::{query_candid_as, update_candid_as};
use station_api::{AddAccountOperationInput, AllowDTO, HealthStatus, RequestOperationInput};
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
            quorum: 1,
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

    assert_eq!(admins.quorum, 1);
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
