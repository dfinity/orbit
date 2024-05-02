use crate::setup::{get_canister_wasm, setup_new_env, setup_new_env_with_config, SetupConfig};
use crate::utils::{controller_test_id, user_test_id};
use crate::TestEnv;
use candid::Principal;
use control_panel_api::UploadCanisterModulesInput;
use control_panel_api::{
    DeployStationInput, DeployStationResponse, GetMainStationResponse, ManageUserInput,
    ManageUserResponse, RegisterUserInput, RegisterUserResponse, UpdateWaitingListInput,
    UserStationDTO, UserSubscriptionStatusDTO,
};
use orbit_essentials::api::ApiResult;
use pocket_ic::update_candid_as;
use station_api::HealthStatus;

#[test]
fn register_user_successful() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let user_id = user_test_id(0);

    // user has no station so far
    let res: (ApiResult<GetMainStationResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "get_main_station",
        (),
    )
    .unwrap();
    let err = res.0.unwrap_err();
    assert_eq!(err.code, "NOT_FOUND");

    // register user
    let register_args = RegisterUserInput {
        station: Some(UserStationDTO {
            canister_id: canister_ids.station,
            name: "main".to_string(),
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
    let res: (ApiResult<GetMainStationResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "get_main_station",
        (),
    )
    .unwrap();
    let main_station_dto = res.0.unwrap().station.unwrap();
    assert_eq!(main_station_dto.canister_id, canister_ids.station);
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
        station_name: "station".to_string(),
        admin_name: "admin".to_string(),
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
        station_name: "station".to_string(),
        admin_name: "admin".to_string(),
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
        station_name: "station".to_string(),
        admin_name: "admin".to_string(),
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
    let res: (ApiResult<GetMainStationResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "get_main_station",
        (),
    )
    .unwrap();
    let main_station_dto = res.0.unwrap().station.unwrap();
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
            station_name: format!("station_{}", i),
            admin_name: "admin".to_string(),
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

    // check that the user has 10 stations and the first deployed station is the main station
    let res: (ApiResult<ManageUserResponse>,) =
        update_candid_as(&env, canister_ids.control_panel, user_id, "get_user", ()).unwrap();
    let user_dto = res.0.unwrap().user;
    assert_eq!(user_dto.stations.len(), 3);
    assert_eq!(user_dto.main_station, Some(stations[0]));

    // reset all but one deployed station
    let manage_user_args = ManageUserInput {
        main_station: Some(stations[0]),
        stations: Some(vec![UserStationDTO {
            canister_id: stations[0],
            name: "main".to_string(),
        }]),
    };
    let res: (ApiResult<ManageUserResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "manage_user",
        (manage_user_args,),
    )
    .unwrap();
    let user_dto = res.0.unwrap().user;
    assert_eq!(user_dto.stations.len(), 1);

    let deploy_station_args = DeployStationInput {
        station_name: "last_station".to_string(),
        admin_name: "admin".to_string(),
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
        station_name: "station".to_string(),
        admin_name: "admin".to_string(),
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

    // upload canister modules
    let upgrader_wasm = get_canister_wasm("upgrader").to_vec();
    let station_wasm = get_canister_wasm("station").to_vec();
    let upload_canister_modules_args = UploadCanisterModulesInput {
        station_wasm_module: station_wasm.to_owned(),
        upgrader_wasm_module: upgrader_wasm.to_owned(),
    };
    let res: (ApiResult<()>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        controller,
        "upload_canister_modules",
        (upload_canister_modules_args.clone(),),
    )
    .unwrap();
    res.0.unwrap();

    // deploying user station succeeds after uploading canister modules
    let deploy_station_args = DeployStationInput {
        station_name: "station".to_string(),
        admin_name: "admin".to_string(),
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

    let upgrader_wasm = get_canister_wasm("upgrader").to_vec();
    let station_wasm = get_canister_wasm("station").to_vec();
    let upload_canister_modules_args = UploadCanisterModulesInput {
        station_wasm_module: station_wasm.to_owned(),
        upgrader_wasm_module: upgrader_wasm.to_owned(),
    };
    let res: (ApiResult<()>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        controller,
        "upload_canister_modules",
        (upload_canister_modules_args.clone(),),
    )
    .unwrap();
    res.0.unwrap();

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
