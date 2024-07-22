use crate::{
    cli::{dfx_orbit_test, setup_agent, setup_dfx_user},
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

    let response = dfx_orbit_test(&mut env, async {
        // Setup the station agent
        let mut station_agent = setup_agent(canister_ids.station).await;

        // Call the counter canister
        let response = station_agent.me().await.unwrap();
        response
    });

    assert_eq!(response.me.id, dfx_user.id);
}
