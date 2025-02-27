use crate::setup::{
    create_canister_with_cycles, get_canister_wasm, setup_new_env, setup_new_env_with_config,
    SetupConfig, WALLET_ADMIN_USER,
};
use crate::utils::{
    await_station_healthy, controller_test_id, get_system_info, set_controllers,
    upload_canister_modules, user_test_id, NNS_ROOT_CANISTER_ID,
};
use crate::TestEnv;
use candid::{Encode, Principal};
use control_panel_api::{
    AssociateWithCallerInput, CanDeployStationResponse, DeployStationAdminUserInput,
    DeployStationInput, DeployStationResponse, ListUserStationsInput, ManageUserStationsInput,
    RegisterUserInput, RegisterUserResponse, UpdateWaitingListInput, UserStationDTO,
    UserSubscriptionStatusDTO,
};
use control_panel_api::{ListUserStationsResponse, UploadCanisterModulesInput};
use orbit_essentials::api::ApiResult;
use orbit_essentials::cmc::{SubnetFilter, SubnetSelection};
use pocket_ic::management_canister::CanisterInstallMode;
use pocket_ic::{update_candid_as, PocketIc};
use sha2::{Digest, Sha256};
use station_api::{
    AdminInitInput, HealthStatus, SystemInfoResponse, SystemInit as SystemInitArg,
    SystemInstall as SystemInstallArg,
};

#[test]
fn register_user_successful() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let user_id = user_test_id(0);

    // register user
    let register_args = RegisterUserInput {
        station: Some(UserStationDTO {
            canister_id: canister_ids.station,
            name: "main".to_string(),
            labels: vec![],
        }),
    };
    let res: (ApiResult<RegisterUserResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "register_user",
        (register_args,),
    )
    .unwrap();
    let user_dto = res.0.unwrap().user;
    assert_eq!(user_dto.identity, user_id);

    // get main station
    let res: (ApiResult<ListUserStationsResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "list_user_stations",
        (ListUserStationsInput {
            filter_by_labels: None,
        },),
    )
    .unwrap();
    let main_station_dto = res.0.unwrap().stations[0].clone();
    assert_eq!(main_station_dto.canister_id, canister_ids.station);
}

#[test]
fn no_station_fallback_controller() {
    let config = SetupConfig {
        fallback_controller: None,
        ..SetupConfig::default()
    };
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env_with_config(config);

    // we deployed the station without a fallback controller,
    // i.e., the upgrader canister is the only controller of the station
    // first get the upgrader canister ID
    let res: (ApiResult<SystemInfoResponse>,) = update_candid_as(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        "system_info",
        (),
    )
    .unwrap();
    let upgrader_canister_id = res.0.unwrap().system.upgrader_id;
    // now get the canister status from the management canister on behalf of the upgrader canister
    // (note that only controllers can invoke the canister status management canister method)
    let canister_status = env
        .canister_status(canister_ids.station, Some(upgrader_canister_id))
        .unwrap();
    // assert that the set of controllers is equal to {upgrader_canister_id}
    let station_controllers = canister_status.settings.controllers;
    assert_eq!(station_controllers, vec![upgrader_canister_id]);
}

#[test]
fn station_fallback_controller() {
    let fallback_controller = WALLET_ADMIN_USER;
    let config = SetupConfig {
        fallback_controller: Some(fallback_controller),
        ..SetupConfig::default()
    };
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env_with_config(config);

    // we deploy the station with the wallet admin user being a fallback controller,
    // i.e., the station's controllers are the upgrader canister and the fallback controller,
    // first get the upgrader canister ID
    let res: (ApiResult<SystemInfoResponse>,) = update_candid_as(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        "system_info",
        (),
    )
    .unwrap();
    let upgrader_canister_id = res.0.unwrap().system.upgrader_id;
    // now get the canister status from the management canister on behalf of the upgrader canister
    // (note that only controllers can invoke the canister status management canister method)
    let canister_status = env
        .canister_status(canister_ids.station, Some(upgrader_canister_id))
        .unwrap();
    // assert that the set of controllers is equal to {upgrader_canister_id, fallback_controller}
    let station_controllers = canister_status.settings.controllers;
    assert_eq!(station_controllers.len(), 2);
    assert!(station_controllers.contains(&upgrader_canister_id));
    assert!(station_controllers.contains(&fallback_controller));
    assert_ne!(upgrader_canister_id, fallback_controller);
}

#[test]
fn deploy_user_station() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let user_id = user_test_id(0);

    // register user
    let register_args = RegisterUserInput { station: None };
    let res: (ApiResult<RegisterUserResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "register_user",
        (register_args,),
    )
    .unwrap();
    let user_dto = res.0.unwrap().user;
    assert_eq!(user_dto.identity, user_id);

    let deploy_station_args = DeployStationInput {
        name: "station".to_string(),
        admins: vec![DeployStationAdminUserInput {
            identity: user_id,
            username: "admin".to_string(),
        }],
        associate_with_caller: Some(AssociateWithCallerInput { labels: vec![] }),
        subnet_selection: None,
    };

    // user can't deploy station before being approved
    let res: (ApiResult<DeployStationResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "deploy_station",
        (deploy_station_args,),
    )
    .unwrap();
    res.0.unwrap_err();

    // subscribe to waiting list
    let res: (ApiResult<()>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "subscribe_to_waiting_list",
        ("john@example.com".to_string(),),
    )
    .unwrap();
    res.0.unwrap();

    let deploy_station_args = DeployStationInput {
        name: "station".to_string(),
        admins: vec![DeployStationAdminUserInput {
            identity: user_id,
            username: "admin".to_string(),
        }],
        associate_with_caller: Some(AssociateWithCallerInput { labels: vec![] }),
        subnet_selection: None,
    };

    // user can't deploy station before being approved
    let res: (ApiResult<DeployStationResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "deploy_station",
        (deploy_station_args,),
    )
    .unwrap();
    res.0.unwrap_err();

    // only canister controllers can approve users
    let update_waiting_list_args = UpdateWaitingListInput {
        users: vec![user_id],
        new_status: UserSubscriptionStatusDTO::Approved,
    };
    let res: (ApiResult<()>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "update_waiting_list",
        (update_waiting_list_args.clone(),),
    )
    .unwrap();
    res.0.unwrap_err();

    // approve user
    let res: (ApiResult<()>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        controller_test_id(),
        "update_waiting_list",
        (update_waiting_list_args,),
    )
    .unwrap();
    res.0.unwrap();

    let deploy_station_args = DeployStationInput {
        name: "station".to_string(),
        admins: vec![DeployStationAdminUserInput {
            identity: user_id,
            username: "admin".to_string(),
        }],
        associate_with_caller: Some(AssociateWithCallerInput { labels: vec![] }),
        subnet_selection: None,
    };

    // deploy user station
    let res: (ApiResult<DeployStationResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "deploy_station",
        (deploy_station_args,),
    )
    .unwrap();
    let newly_created_user_station = res.0.unwrap().canister_id;

    // get main station
    let res: (ApiResult<ListUserStationsResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "list_user_stations",
        (ListUserStationsInput {
            filter_by_labels: None,
        },),
    )
    .unwrap();
    let main_station_dto = res.0.unwrap().stations[0].clone();
    assert_eq!(main_station_dto.canister_id, newly_created_user_station);

    // the newly created station should be uninitialized at first
    let res: (HealthStatus,) = update_candid_as(
        &env,
        newly_created_user_station,
        user_id,
        "health_status",
        (),
    )
    .unwrap();
    let health_status = res.0;
    assert_eq!(health_status, HealthStatus::Uninitialized);

    await_station_healthy(&env, newly_created_user_station, user_id);

    // the newly created station should be at the same subnet as the control panel
    assert_eq!(
        env.get_subnet(newly_created_user_station).unwrap(),
        env.get_subnet(canister_ids.control_panel).unwrap()
    );
    // which is different from the fiduciary subnet
    assert_ne!(
        env.get_subnet(newly_created_user_station).unwrap(),
        env.topology().get_fiduciary().unwrap()
    );

    check_station_deployment(&env, newly_created_user_station, user_id);
}

fn check_station_deployment(env: &PocketIc, station_id: Principal, user_id: Principal) {
    // first get the upgrader canister ID
    let system_info = get_system_info(env, user_id, station_id);
    let upgrader_id = system_info.upgrader_id;
    // now get the canister status from the management canister on behalf of the upgrader canister
    // (note that only controllers can invoke the canister status management canister method)
    let canister_status = env.canister_status(station_id, Some(upgrader_id)).unwrap();
    // the control panel should set the newly deployed station's controllers
    // to be the upgrader canister and the NNS root canister;
    // assert that the set of controllers is equal to {upgrader_canister_id, NNS_ROOT_CANISTER_ID}
    let station_controllers = canister_status.settings.controllers;
    assert_eq!(station_controllers.len(), 2);
    assert!(station_controllers.contains(&upgrader_id));
    assert!(station_controllers.contains(&NNS_ROOT_CANISTER_ID));
    assert_ne!(upgrader_id, NNS_ROOT_CANISTER_ID);

    // stop the station and upgrader to get their cycles balance including reservations
    env.stop_canister(upgrader_id, Some(station_id)).unwrap();
    env.stop_canister(station_id, Some(upgrader_id)).unwrap();
    // check the cycles balance of station and upgrader
    let upgrader_cycles = env.cycle_balance(upgrader_id);
    assert!((900_000_000_000..1_100_000_000_000).contains(&upgrader_cycles));
    let station_cycles = env.cycle_balance(station_id);
    assert!((900_000_000_000..1_100_000_000_000).contains(&station_cycles));
}

#[test]
fn deploy_too_many_stations() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    // top up the control panel to deploy all the many stations
    env.add_cycles(canister_ids.control_panel, 10_000_000_000_000_000);

    let bootstrap_user = |user_id: Principal| {
        // register user
        let register_args = RegisterUserInput { station: None };
        let res: (ApiResult<RegisterUserResponse>,) = update_candid_as(
            &env,
            canister_ids.control_panel,
            user_id,
            "register_user",
            (register_args,),
        )
        .unwrap();
        let user_dto = res.0.unwrap().user;
        assert_eq!(user_dto.identity, user_id);

        // approve user
        let update_waiting_list_args = UpdateWaitingListInput {
            users: vec![user_id],
            new_status: UserSubscriptionStatusDTO::Approved,
        };
        let res: (ApiResult<()>,) = update_candid_as(
            &env,
            canister_ids.control_panel,
            controller_test_id(),
            "update_waiting_list",
            (update_waiting_list_args,),
        )
        .unwrap();
        res.0.unwrap();
    };

    let can_deploy = |user_id: Principal| -> ApiResult<CanDeployStationResponse> {
        update_candid_as::<_, (ApiResult<CanDeployStationResponse>,)>(
            &env,
            canister_ids.control_panel,
            user_id,
            "can_deploy_station",
            ((),),
        )
        .unwrap()
        .0
    };

    let deploy = |user_id: Principal, day: usize, i: usize| -> ApiResult<DeployStationResponse> {
        let deploy_station_args = DeployStationInput {
            name: format!("station_{}_{}_{}", user_id, day, i),
            admins: vec![DeployStationAdminUserInput {
                identity: user_id,
                username: "admin".to_string(),
            }],
            associate_with_caller: Some(AssociateWithCallerInput { labels: vec![] }),
            subnet_selection: None,
        };
        update_candid_as::<_, (ApiResult<DeployStationResponse>,)>(
            &env,
            canister_ids.control_panel,
            user_id,
            "deploy_station",
            (deploy_station_args,),
        )
        .unwrap()
        .0
    };

    let max_stations_per_user = 2;
    let max_stations_per_day = 100;

    for i in 0..(max_stations_per_day + 1) {
        bootstrap_user(user_test_id(i));
        // to prevent rate-limiting
        env.advance_time(std::time::Duration::from_secs(60));
    }

    for day in 0..2 {
        // deploy the maximum amount of stations per user
        let user_id = user_test_id(0);
        let mut stations = vec![];
        for i in 0..max_stations_per_user {
            assert!(matches!(
                can_deploy(user_id).unwrap(),
                CanDeployStationResponse::Allowed(remaining) if max_stations_per_user == remaining + i
            ));
            let station_id = deploy(user_id, day, i).unwrap().canister_id;
            stations.push(station_id);
        }

        // check that the stations have been deployed by listing the user's stations
        let res: (ApiResult<ListUserStationsResponse>,) = update_candid_as(
            &env,
            canister_ids.control_panel,
            user_id,
            "list_user_stations",
            (ListUserStationsInput {
                filter_by_labels: None,
            },),
        )
        .unwrap();
        let associated_stations = res.0.unwrap().stations;
        assert_eq!(associated_stations.len(), day + max_stations_per_user);

        // reset all but one deployed station
        let manage_user_stations_args = ManageUserStationsInput::Remove(stations[1..].to_vec());
        let res: (ApiResult<()>,) = update_candid_as(
            &env,
            canister_ids.control_panel,
            user_id,
            "manage_user_stations",
            (manage_user_stations_args,),
        )
        .unwrap();
        assert!(res.0.is_ok());

        // deploying a new station should fail nonetheless
        assert!(matches!(
            can_deploy(user_id).unwrap(),
            CanDeployStationResponse::QuotaExceeded
        ));
        assert_eq!(
            deploy(user_id, day, max_stations_per_user)
                .unwrap_err()
                .code,
            "DEPLOY_STATION_QUOTA_EXCEEDED"
        );

        // so far `max_stations_per_user` have been deployed today;
        // now deploy up to `max_stations_per_day` on behalf of pairwise distinct users
        for i in 1..(max_stations_per_day + 1 - max_stations_per_user as u64) {
            assert!(matches!(
                can_deploy(user_test_id(i)).unwrap(),
                CanDeployStationResponse::Allowed(remaining) if remaining == std::cmp::min(max_stations_per_user, max_stations_per_day as usize - max_stations_per_user - (i as usize - 1))
            ));
            deploy(user_test_id(i), day, 0).unwrap();
        }

        // deploying one more station on behalf of yet another use should fail due to global rate limit
        assert!(matches!(
            can_deploy(user_test_id(max_stations_per_day)).unwrap(),
            CanDeployStationResponse::QuotaExceeded
        ));
        assert_eq!(
            deploy(user_test_id(max_stations_per_day), day, 0)
                .unwrap_err()
                .code,
            "DEPLOY_STATION_QUOTA_EXCEEDED"
        );

        // tomorrow the user should again be able to deploy stations
        env.advance_time(std::time::Duration::from_secs(86400));
    }
}

#[test]
fn no_upload_canister_modules() {
    let TestEnv {
        env,
        canister_ids,
        controller,
        ..
    } = setup_new_env_with_config(SetupConfig {
        upload_canister_modules: false,
        fallback_controller: None,
        start_cycles: None,
        ..Default::default()
    });

    let user_id = user_test_id(0);

    // register user
    let register_args = RegisterUserInput { station: None };
    let res: (ApiResult<RegisterUserResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "register_user",
        (register_args,),
    )
    .unwrap();
    let user_dto = res.0.unwrap().user;
    assert_eq!(user_dto.identity, user_id);

    // approve user
    let update_waiting_list_args = UpdateWaitingListInput {
        users: vec![user_id],
        new_status: UserSubscriptionStatusDTO::Approved,
    };
    let res: (ApiResult<()>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        controller_test_id(),
        "update_waiting_list",
        (update_waiting_list_args,),
    )
    .unwrap();
    res.0.unwrap();

    // deploying user station fails before uploading canister modules
    let deploy_station_args = DeployStationInput {
        name: "station".to_string(),
        admins: vec![DeployStationAdminUserInput {
            identity: user_id,
            username: "admin".to_string(),
        }],
        associate_with_caller: Some(AssociateWithCallerInput { labels: vec![] }),
        subnet_selection: None,
    };
    let res: (ApiResult<DeployStationResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "deploy_station",
        (deploy_station_args,),
    )
    .unwrap();
    assert!(res
        .0
        .unwrap_err()
        .message
        .unwrap()
        .contains("Canister config not initialized"));

    upload_canister_modules(&env, canister_ids.control_panel, controller);

    // deploying user station succeeds after uploading canister modules
    let deploy_station_args = DeployStationInput {
        name: "station".to_string(),
        admins: vec![DeployStationAdminUserInput {
            identity: user_id,
            username: "admin".to_string(),
        }],
        associate_with_caller: Some(AssociateWithCallerInput { labels: vec![] }),
        subnet_selection: None,
    };
    let res: (ApiResult<DeployStationResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "deploy_station",
        (deploy_station_args,),
    )
    .unwrap();
    res.0.unwrap();
}

#[test]
fn upload_canister_modules_authorization() {
    let TestEnv {
        env,
        canister_ids,
        controller,
        ..
    } = setup_new_env();

    upload_canister_modules(&env, canister_ids.control_panel, controller);

    let upload_canister_modules_args = UploadCanisterModulesInput {
        upgrader_wasm_module: None,
        station_wasm_module: None,
        station_wasm_module_extra_chunks: None,
    };
    let res: (ApiResult<()>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        Principal::anonymous(),
        "upload_canister_modules",
        (upload_canister_modules_args,),
    )
    .unwrap();
    let error = res.0.unwrap_err();
    error
        .message
        .unwrap()
        .contains("You don't have permission to make the call.");
    assert_eq!(
        error.details.unwrap().get("method").unwrap(),
        "upload_canister_modules"
    );
}

#[test]
fn deploy_user_station_to_different_subnet() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let user_id = user_test_id(0);

    // register user
    let register_args = RegisterUserInput { station: None };
    let res: (ApiResult<RegisterUserResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "register_user",
        (register_args,),
    )
    .unwrap();
    let user_dto = res.0.unwrap().user;
    assert_eq!(user_dto.identity, user_id);

    // subscribe to waiting list
    let res: (ApiResult<()>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "subscribe_to_waiting_list",
        ("john@example.com".to_string(),),
    )
    .unwrap();
    res.0.unwrap();

    // approve user
    let update_waiting_list_args = UpdateWaitingListInput {
        users: vec![user_id],
        new_status: UserSubscriptionStatusDTO::Approved,
    };
    let res: (ApiResult<()>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        controller_test_id(),
        "update_waiting_list",
        (update_waiting_list_args,),
    )
    .unwrap();
    res.0.unwrap();

    // deploy user station
    let deploy_station_args = DeployStationInput {
        name: "station".to_string(),
        admins: vec![DeployStationAdminUserInput {
            identity: user_id,
            username: "admin".to_string(),
        }],
        associate_with_caller: Some(AssociateWithCallerInput { labels: vec![] }),
        subnet_selection: Some(SubnetSelection::Filter(SubnetFilter {
            subnet_type: Some("fiduciary".to_string()),
        })),
    };
    let res: (ApiResult<DeployStationResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "deploy_station",
        (deploy_station_args,),
    )
    .unwrap();
    let newly_created_user_station = res.0.unwrap().canister_id;

    // check that the station has been deployed to the fiduciary subnet
    assert_eq!(
        env.get_subnet(newly_created_user_station).unwrap(),
        env.topology().get_fiduciary().unwrap()
    );
    // which is different from the subnet to which the control panel is deployed
    assert_ne!(
        env.get_subnet(newly_created_user_station).unwrap(),
        env.get_subnet(canister_ids.control_panel).unwrap()
    );

    await_station_healthy(&env, newly_created_user_station, user_id);
    check_station_deployment(&env, newly_created_user_station, user_id);
}

#[test]
fn insufficient_control_panel_cycles() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env_with_config(SetupConfig {
        start_cycles: Some(10_000_000_000_000),
        ..Default::default()
    });

    let mut i = 0;
    loop {
        let user_id = user_test_id(i);
        i += 1;

        // register user
        let register_args = RegisterUserInput { station: None };
        let res: (ApiResult<RegisterUserResponse>,) = update_candid_as(
            &env,
            canister_ids.control_panel,
            user_id,
            "register_user",
            (register_args,),
        )
        .unwrap();
        let user_dto = res.0.unwrap().user;
        assert_eq!(user_dto.identity, user_id);

        // approve user
        let update_waiting_list_args = UpdateWaitingListInput {
            users: vec![user_id],
            new_status: UserSubscriptionStatusDTO::Approved,
        };
        let res: (ApiResult<()>,) = update_candid_as(
            &env,
            canister_ids.control_panel,
            controller_test_id(),
            "update_waiting_list",
            (update_waiting_list_args,),
        )
        .unwrap();
        res.0.unwrap();

        // deploy station
        let deploy_station_args = DeployStationInput {
            name: format!("station_{}", i),
            admins: vec![DeployStationAdminUserInput {
                identity: user_id,
                username: "admin".to_string(),
            }],
            associate_with_caller: Some(AssociateWithCallerInput { labels: vec![] }),
            subnet_selection: None,
        };

        let res: (ApiResult<DeployStationResponse>,) = update_candid_as(
            &env,
            canister_ids.control_panel,
            user_id,
            "deploy_station",
            (deploy_station_args,),
        )
        .unwrap();
        if let Err(e) = res.0 {
            assert_eq!(
                *e.details.unwrap().get("reason").unwrap(),
                format!(
                    "Canister {} has insufficient cycles balance to transfer 1500000000000 cycles.",
                    canister_ids.control_panel
                )
            );
            let station_status = env
                .canister_status(canister_ids.control_panel, Some(controller_test_id()))
                .unwrap();
            let min_balance_for_deploy_station = station_status.idle_cycles_burned_per_day
                * station_status.settings.freezing_threshold
                * 2_u64
                / 86_400_u64;
            // control panel does not deploy station if balance < `min_balance_for_deploy_station` + `INITIAL_STATION_CYCLES` and station deployment takes `INITIAL_STATION_CYCLES`
            assert!(
                env.cycle_balance(canister_ids.control_panel) >= min_balance_for_deploy_station
            );
            break;
        }
    }
}

#[test]
fn deploy_station_with_insufficient_cycles() {
    let TestEnv { env, .. } = setup_new_env();

    let upgrader_initial_cycles = 10_000_000_000_000; // 10T

    // deploy the station with 400B cycles and add the station as its own controller
    let station_initial_cycles = 400_000_000_000; // 400B
    let station = create_canister_with_cycles(&env, WALLET_ADMIN_USER, station_initial_cycles);
    assert_eq!(env.cycle_balance(station), station_initial_cycles);
    set_controllers(
        &env,
        Some(WALLET_ADMIN_USER),
        station,
        vec![WALLET_ADMIN_USER, station],
    );

    // upload the station WASM to the station's ICP chunk store
    let station_wasm = get_canister_wasm("station").to_vec();
    env.clear_chunk_store(station, Some(WALLET_ADMIN_USER))
        .unwrap();
    let chunks: Vec<_> = station_wasm.chunks(1_000_000).collect();
    let mut hashes = vec![];
    for chunk in chunks {
        let hash = env
            .upload_chunk(station, Some(WALLET_ADMIN_USER), chunk.to_vec())
            .unwrap();
        hashes.push(hash);
    }
    let mut hasher = Sha256::new();
    hasher.update(station_wasm);
    let wasm_module_hash = hasher.finalize().to_vec();

    // station init args
    let upgrader_wasm = get_canister_wasm("upgrader").to_vec();
    let station_init_args = Encode!(&SystemInstallArg::Init(SystemInitArg {
        name: "Station".to_string(),
        admins: vec![AdminInitInput {
            identity: WALLET_ADMIN_USER,
            name: "station-admin".to_string(),
        }],
        assets: None,
        quorum: Some(1),
        upgrader: station_api::SystemUpgraderInput::Deploy(
            station_api::DeploySystemUpgraderInput {
                wasm_module: upgrader_wasm,
                initial_cycles: Some(upgrader_initial_cycles),
            },
        ),
        fallback_controller: None,
        accounts: None,
    }))
    .unwrap();

    // installing the station should fail due to insufficient balance for deploying the upgrader
    // and consume no more than 50B cycles
    let cycles_before_install = env.cycle_balance(station);
    let err = env
        .install_chunked_canister(
            station,
            Some(WALLET_ADMIN_USER),
            CanisterInstallMode::Install,
            station,
            hashes.clone(),
            wasm_module_hash.clone(),
            station_init_args.clone(),
        )
        .unwrap_err();
    assert!(err.reject_message.contains(&format!(
        "insufficient for transferring {} cycles when deploying the upgrader",
        upgrader_initial_cycles
    )));
    let cycles_after_failed_install = env.cycle_balance(station);
    assert!(cycles_before_install <= cycles_after_failed_install + 50_000_000_000);

    // top up the station to have enough cycles to transfer when deploying the upgrader
    env.add_cycles(station, upgrader_initial_cycles);

    // skip a few rounds to prevent instruction rate-limiting for canister installation
    for _ in 0..100 {
        env.tick();
    }

    // now installing the station should succeed
    env.install_chunked_canister(
        station,
        Some(WALLET_ADMIN_USER),
        CanisterInstallMode::Install,
        station,
        hashes,
        wasm_module_hash,
        station_init_args,
    )
    .unwrap();

    // and the station should eventually become healthy
    await_station_healthy(&env, station, WALLET_ADMIN_USER);
}
