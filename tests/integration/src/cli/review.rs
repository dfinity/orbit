use candid::Principal;

use crate::{
    cli::{
        canister_call::{permit_call_operation, set_four_eyes_on_call},
        setup_counter_canister, TEST_PRINCIPAL,
    },
    setup::setup_new_env,
    utils::{add_user, add_user_with_name, user_test_id},
    TestEnv,
};

#[test]
fn review_canister_upgrate() {
    let TestEnv {
        mut env,
        canister_ids,
        ..
    } = setup_new_env();

    let canister_id = setup_counter_canister(&mut env, &canister_ids);

    // create new user identities and add them to the station
    let dfx_user = Principal::from_text(TEST_PRINCIPAL).unwrap();
    add_user_with_name(
        &env,
        String::from("dfx_user"),
        dfx_user,
        vec![],
        canister_ids.station,
    );
    let other_user = user_test_id(1);
    add_user(&env, other_user, vec![], canister_ids.station);

    permit_call_operation(&env, &canister_ids);
    set_four_eyes_on_call(&env, &canister_ids);

    todo!()
}
