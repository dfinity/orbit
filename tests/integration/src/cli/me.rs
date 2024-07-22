use crate::{
    cli::{dfx_orbit_test, setup_agent, TEST_PRINCIPAL},
    setup::setup_new_env,
    utils::add_user_with_name,
    TestEnv,
};
use candid::Principal;

#[test]
fn me() {
    let TestEnv {
        mut env,
        canister_ids,
        ..
    } = setup_new_env();

    let dfx_user = Principal::from_text(TEST_PRINCIPAL).unwrap();
    let dfx_user_id = add_user_with_name(
        &env,
        String::from("dfx_user"),
        dfx_user,
        vec![],
        canister_ids.station,
    );

    let response = dfx_orbit_test(&mut env, async {
        // Setup the station agent
        let mut station_agent = setup_agent(canister_ids.station).await;

        // Call the counter canister
        let response = station_agent.me().await.unwrap();
        response
    });

    assert_eq!(response.me.id, dfx_user_id.id);
}
