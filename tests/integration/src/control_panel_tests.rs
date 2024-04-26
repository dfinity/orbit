use crate::setup::setup_new_env;
use crate::utils::{controller_test_id, user_test_id};
use crate::TestEnv;
use control_panel_api::{
    DeployStationResponse, GetMainStationResponse, ManageUserInput, ManageUserResponse,
    RegisterUserInput, RegisterUserResponse, UpdateWaitingListInput, UserStationDTO,
    UserSubscriptionStatusDTO,
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
        station_id: Some(canister_ids.station),
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
    let register_args = RegisterUserInput { station_id: None };
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

    // user can't deploy station before being approved
    let res: (ApiResult<DeployStationResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "deploy_station",
        (),
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

    // user can't deploy station before being approved
    let res: (ApiResult<DeployStationResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "deploy_station",
        (),
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

    // deploy user station
    let res: (ApiResult<DeployStationResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "deploy_station",
        (),
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
    let register_args = RegisterUserInput { station_id: None };
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
    for _ in 0..10 {
        let res: (ApiResult<DeployStationResponse>,) = update_candid_as(
            &env,
            canister_ids.control_panel,
            user_id,
            "deploy_station",
            (),
        )
        .unwrap();
        stations.push(res.0.unwrap().canister_id);
    }

    // check that the user has 10 stations and the first deployed station is the main station
    let res: (ApiResult<ManageUserResponse>,) =
        update_candid_as(&env, canister_ids.control_panel, user_id, "get_user", ()).unwrap();
    let user_dto = res.0.unwrap().user;
    assert_eq!(user_dto.stations.len(), 10);
    assert_eq!(user_dto.main_station, Some(stations[0]));

    // reset all but one deployed station
    let manage_user_args = ManageUserInput {
        main_station: Some(stations[0]),
        stations: Some(vec![UserStationDTO {
            canister_id: stations[0],
            name: None,
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

    // deploying an additional station should fail nonetheless
    let res: (ApiResult<DeployStationResponse>,) = update_candid_as(
        &env,
        canister_ids.control_panel,
        user_id,
        "deploy_station",
        (),
    )
    .unwrap();
    assert_eq!(res.0.unwrap_err().code, "DEPLOY_WALLET_QUOTA_EXCEEDED");
}
