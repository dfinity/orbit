use crate::{
    cli::{dfx_orbit_test, setup_agent, TEST_PRINCIPAL},
    setup::{create_canister, setup_new_env, WALLET_ADMIN_USER},
    utils::{add_user, execute_request, update_raw, user_test_id, COUNTER_WAT},
    TestEnv,
};
use candid::Principal;
use station_api::{
    CallExternalCanisterOperationInput, CanisterMethodDTO, ChangeExternalCanisterResourceTargetDTO,
    CreateRequestInput, EditPermissionOperationInput, RequestOperationInput,
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
    //let user_a = user_test_id(0);
    let dfx_user = Principal::from_text(TEST_PRINCIPAL).unwrap();
    add_user(&env, dfx_user, vec![], canister_ids.station);
    let user_b = user_test_id(1);
    add_user(&env, user_b, vec![], canister_ids.station);

    // allow anyone to create change canister requests
    let add_permission = RequestOperationInput::EditPermission(EditPermissionOperationInput {
        resource: station_api::ResourceDTO::ExternalCanister(
            station_api::ExternalCanisterResourceActionDTO::Change(
                ChangeExternalCanisterResourceTargetDTO::Canister(canister_id),
            ),
        ),
        auth_scope: Some(station_api::AuthScopeDTO::Authenticated),
        user_groups: None,
        users: None,
    });
    execute_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        add_permission,
    )
    .unwrap();

    dfx_orbit_test(&mut env, async {
        // Setup the station agent
        let mut station_agent = setup_agent(canister_ids.station).await;

        // Call the counter canister
        station_agent
            .request(CreateRequestInput {
                operation: RequestOperationInput::CallExternalCanister(
                    CallExternalCanisterOperationInput {
                        validation_method: None,
                        execution_method: CanisterMethodDTO {
                            canister_id,
                            method_name: String::from("inc"),
                        },
                        arg: None,
                        execution_method_cycles: None,
                    },
                ),
                title: None,
                summary: None,
                execution_plan: None,
            })
            .await
            .unwrap();
    });

    todo!()
}
