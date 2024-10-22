use crate::{
    dfx_orbit::{
        setup::{
            dfx_orbit_test, setup_counter_canister, setup_dfx_orbit, setup_dfx_user,
            DfxOrbitTestConfig,
        },
        util::{permit_change_operation, poll_request_completion, set_auto_approve_on_change},
    },
    setup::setup_new_env,
    utils::user_test_id,
    TestEnv,
};
use candid::Principal;
use dfx_orbit::{
    args::{RequestArgs, RequestArgsActions},
    canister::{RequestCanisterActionArgs, RequestCanisterArgs, RequestCanisterUpdateSettingsArgs},
};
use std::time::Duration;

/// Test that adding and removing controllers works
#[test]
fn change_controllers() {
    let TestEnv {
        mut env,
        canister_ids,
        ..
    } = setup_new_env();

    let canister_id = setup_counter_canister(&mut env, &canister_ids);

    let (_, _) = setup_dfx_user(&env, &canister_ids);

    permit_change_operation(&env, &canister_ids);
    set_auto_approve_on_change(&env, &canister_ids);

    let config = DfxOrbitTestConfig {
        canister_ids: vec![(String::from("counter"), canister_id)],
        ..Default::default()
    };

    let add_controller_args = RequestCanisterUpdateSettingsArgs {
        canister: String::from("counter"),
        add_controller: vec![user_test_id(1)],
        remove_controller: vec![],
    };

    let add_remove_controller_args = RequestCanisterUpdateSettingsArgs {
        canister: String::from("counter"),
        add_controller: vec![user_test_id(2)],
        remove_controller: vec![user_test_id(1)],
    };

    let remove_controller_args = RequestCanisterUpdateSettingsArgs {
        canister: String::from("counter"),
        add_controller: vec![],
        remove_controller: vec![canister_ids.station],
    };

    dfx_orbit_test(&mut env, config, async {
        // Setup the station agent
        let dfx_orbit = setup_dfx_orbit(canister_ids.station).await;

        let controllers = dfx_orbit.get_controllers(canister_id).await.unwrap();
        assert_eq!(controllers, vec![canister_ids.station]);

        // Add another controller to the test canister
        let request = RequestArgs {
            title: None,
            summary: None,
            action: RequestArgsActions::Canister(RequestCanisterArgs {
                action: RequestCanisterActionArgs::UpdateSettings(add_controller_args.clone()),
            }),
        }
        .into_request(&dfx_orbit)
        .await
        .unwrap();
        let request = dfx_orbit.station.request(request.clone()).await.unwrap();
        poll_request_completion(&dfx_orbit, request.request.id, Duration::from_secs(30)).await;

        let controllers = dfx_orbit.get_controllers(canister_id).await.unwrap();
        assert_controllers_eq(controllers, vec![canister_ids.station, user_test_id(1)]);

        // Add and remove a controller to the test canister
        let request = RequestArgs {
            title: None,
            summary: None,
            action: RequestArgsActions::Canister(RequestCanisterArgs {
                action: RequestCanisterActionArgs::UpdateSettings(
                    add_remove_controller_args.clone(),
                ),
            }),
        }
        .into_request(&dfx_orbit)
        .await
        .unwrap();
        let request = dfx_orbit.station.request(request.clone()).await.unwrap();
        poll_request_completion(&dfx_orbit, request.request.id, Duration::from_secs(30)).await;

        let controllers = dfx_orbit.get_controllers(canister_id).await.unwrap();
        assert_controllers_eq(controllers, vec![canister_ids.station, user_test_id(2)]);

        // Add and remove a controller to the test canister
        let request = RequestArgs {
            title: None,
            summary: None,
            action: RequestArgsActions::Canister(RequestCanisterArgs {
                action: RequestCanisterActionArgs::UpdateSettings(remove_controller_args.clone()),
            }),
        }
        .into_request(&dfx_orbit)
        .await
        .unwrap();
        let request = dfx_orbit.station.request(request.clone()).await.unwrap();
        poll_request_completion(&dfx_orbit, request.request.id, Duration::from_secs(30)).await;

        let controllers = dfx_orbit.get_controllers(canister_id).await.unwrap();
        assert_controllers_eq(controllers, vec![user_test_id(2)]);
    });
}

/// Checks that the controllers are equal, by sorthing them before comparison
fn assert_controllers_eq(mut left: Vec<Principal>, mut right: Vec<Principal>) {
    left.sort();
    right.sort();
    assert_eq!(left, right);
}
