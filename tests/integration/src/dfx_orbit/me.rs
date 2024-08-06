use crate::{
    dfx_orbit::{dfx_orbit_test, setup_dfx_orbit, setup_dfx_user, DfxOrbitTestConfig},
    setup::setup_new_env,
    TestEnv,
};

/// Test that the [`StationAgent::me`] method works and returns the correct data.
#[test]
fn me() {
    let TestEnv {
        mut env,
        canister_ids,
        ..
    } = setup_new_env();

    let (_, dfx_user) = setup_dfx_user(&env, &canister_ids);

    let response = dfx_orbit_test(&mut env, DfxOrbitTestConfig::default(), async {
        // Setup the station agent
        let dfx_orbit = setup_dfx_orbit(canister_ids.station).await;

        // Call the counter canister
        dfx_orbit.station.me().await.unwrap()
    });

    assert_eq!(response.me.id, dfx_user.id);
}
