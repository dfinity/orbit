use crate::setup::{setup_new_env, setup_new_env_with_config, SetupConfig, WALLET_ADMIN_USER};
use crate::utils::{
    controller_test_id, upload_canister_modules, user_test_id, NNS_ROOT_CANISTER_ID,
};
use crate::TestEnv;
use candid::Principal;
use control_panel_api::{
    AssociateWithCallerInput, DeployStationAdminUserInput, DeployStationInput,
    DeployStationResponse, ListUserStationsInput, ManageUserStationsInput, RegisterUserInput,
    RegisterUserResponse, UpdateWaitingListInput, UserStationDTO, UserSubscriptionStatusDTO,
};
use control_panel_api::{ListUserStationsResponse, UploadCanisterModulesInput};
use orbit_essentials::api::ApiResult;
use orbit_essentials::cmc::{SubnetFilter, SubnetSelection};
use pocket_ic::{update_candid_as, PocketIc};
use station_api::{HealthStatus, SystemInfoResponse};

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

    let rounds_required_for_station_initialization = 5;
    for _ in 0..rounds_required_for_station_initialization {
        env.tick();
    }

    // the newly created station should be healthy at this point
    let res: (HealthStatus,) = update_candid_as(
        &env,
        newly_created_user_station,
        user_id,
        "health_status",
        (),
    )
    .unwrap();
    let health_status = res.0;
    assert_eq!(health_status, HealthStatus::Healthy);

    check_station_controllers(&env, newly_created_user_station, user_id);
}

fn check_station_controllers(env: &PocketIc, station: Principal, user_id: Principal) {
    // the control panel should set the newly deployed station's controllers
    // to be the upgrader canister and the NNS root canister;
    // first get the upgrader canister ID
    let res: (ApiResult<SystemInfoResponse>,) =
        update_candid_as(env, station, user_id, "system_info", ()).unwrap();
    let upgrader_canister_id = res.0.unwrap().system.upgrader_id;
    // now get the canister status from the management canister on behalf of the upgrader canister
    // (note that only controllers can invoke the canister status management canister method)
    let canister_status = env
        .canister_status(station, Some(upgrader_canister_id))
        .unwrap();
    // assert that the set of controllers is equal to {upgrader_canister_id, NNS_ROOT_CANISTER_ID}
    let station_controllers = canister_status.settings.controllers;
    assert_eq!(station_controllers.len(), 2);
    assert!(station_controllers.contains(&upgrader_canister_id));
    assert!(station_controllers.contains(&NNS_ROOT_CANISTER_ID));
    assert_ne!(upgrader_canister_id, NNS_ROOT_CANISTER_ID);
}

#[test]
fn deploy_too_many_stations() {
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

    // deploy the maximum amount of user stations
    let mut stations = vec![];
    for i in 0..3 {
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
        stations.push(res.0.unwrap().canister_id);
    }

    // check that the user has 3 stations and the first deployed station is the main station
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
    assert_eq!(associated_stations.len(), 3);
    assert_eq!(associated_stations[0].canister_id, stations[0]);

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

    let deploy_station_args = DeployStationInput {
        name: "last_station".to_string(),
        admins: vec![DeployStationAdminUserInput {
            identity: user_id,
            username: "admin".to_string(),
        }],
        associate_with_caller: Some(AssociateWithCallerInput { labels: vec![] }),
        subnet_selection: None,
    };

    // deploying an additional station should fail nonetheless
    let res: (ApiResult<DeployStationResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "deploy_station",
        (deploy_station_args,),
    )
    .unwrap();
    assert_eq!(res.0.unwrap_err().code, "DEPLOY_STATION_QUOTA_EXCEEDED");
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

    // wait until the station is healthy
    let rounds_required_for_station_initialization = 5;
    for _ in 0..rounds_required_for_station_initialization {
        env.tick();
    }

    // the newly created station should be healthy at this point
    let res: (HealthStatus,) = update_candid_as(
        &env,
        newly_created_user_station,
        user_id,
        "health_status",
        (),
    )
    .unwrap();
    let health_status = res.0;
    assert_eq!(health_status, HealthStatus::Healthy);

    check_station_controllers(&env, newly_created_user_station, user_id);
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
                    "Canister {} has insufficient cycles balance to transfer 2500000000000 cycles.",
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
