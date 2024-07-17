use candid::Principal;
use dfx_orbit::StationAgent;

use crate::{
    setup::{create_canister, setup_new_env},
    utils::{add_user, update_raw, user_test_id, COUNTER_WAT},
    TestEnv,
};

/// Test a canister call through orbit using the station agent
#[test]
fn canister_call() {
    let TestEnv {
        mut env,
        canister_ids,
        ..
    } = setup_new_env();

    // create and install the counter canister
    let canister_id = create_canister(&mut env, canister_ids.station);
    let module_bytes = wat::parse_str(COUNTER_WAT).unwrap();
    env.install_canister(
        canister_id,
        module_bytes.clone(),
        vec![],
        Some(canister_ids.station),
    );

    // the counter should initially be set at 0
    let ctr = update_raw(&env, canister_id, Principal::anonymous(), "read", vec![]).unwrap();
    assert_eq!(ctr, 0_u32.to_le_bytes());

    // create new user identities and add them to the station
    let user_a = user_test_id(0);
    add_user(&env, user_a, vec![], canister_ids.station);
    let user_b = user_test_id(1);
    add_user(&env, user_b, vec![], canister_ids.station);

    // TODO: set up the station agent
    StationAgent::new().unwrap();

    todo!()
}
