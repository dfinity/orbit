use crate::{
    dfx_orbit::{
        setup::{
            dfx_orbit_test, setup_counter_canister, setup_dfx_orbit, setup_dfx_user,
            DfxOrbitTestConfig,
        },
        util::permit_change_operation,
    },
    setup::setup_new_env,
    utils::user_test_id,
    TestEnv,
};
use dfx_orbit::{
    args::{RequestArgs, RequestArgsActions},
    canister::{RequestCanisterActionArgs, RequestCanisterArgs, RequestCanisterUpdateSettingsArgs},
};

/// Test that adding and removing controllers works
#[test]
fn change_controllers() {
    let TestEnv {
        mut env,
        canister_ids,
        ..
    } = setup_new_env();

    let canister_id = setup_counter_canister(&mut env, &canister_ids);

    let (dfx_user, _) = setup_dfx_user(&env, &canister_ids);
    permit_change_operation(&env, &canister_ids);

    let config = DfxOrbitTestConfig {
        canister_ids: vec![(String::from("counter"), canister_id)],
        ..Default::default()
    };

    let add_controller_args = RequestCanisterUpdateSettingsArgs {
        canister: String::from("counter"),
        add_controller: vec![user_test_id(1)],
        remove_controller: vec![],
    };

    let remove_controller_args = RequestCanisterUpdateSettingsArgs {
        canister: String::from("counter"),
        add_controller: vec![],
        remove_controller: vec![user_test_id(1)],
    };

    let request = dfx_orbit_test(&mut env, config, async {
        // Setup the station agent
        let dfx_orbit = setup_dfx_orbit(canister_ids.station).await;

        // Call the test canister
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

        request.request
    });
    todo!()
}
