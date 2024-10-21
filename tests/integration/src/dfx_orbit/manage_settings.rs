use crate::{
    dfx_orbit::{
        setup::{
            dfx_orbit_test, setup_counter_canister, setup_dfx_orbit, setup_dfx_user,
            DfxOrbitTestConfig,
        },
        util::{permit_change_operation, set_auto_approve_on_change},
    },
    setup::{setup_new_env, WALLET_ADMIN_USER},
    utils::{submit_request, user_test_id},
    TestEnv,
};
use dfx_orbit::{
    args::{RequestArgs, RequestArgsActions},
    canister::{RequestCanisterActionArgs, RequestCanisterArgs, RequestCanisterUpdateSettingsArgs},
};
use station_api::{
    AllowDTO, CreateExternalCanisterOperationInput,
    CreateExternalCanisterOperationKindAddExistingDTO, CreateExternalCanisterOperationKindDTO,
    ExternalCanisterPermissionsCreateInput, ExternalCanisterRequestPoliciesCreateInput,
    RequestOperationInput,
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

    // Register the canister with the station
    submit_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        RequestOperationInput::CreateExternalCanister(CreateExternalCanisterOperationInput {
            kind: CreateExternalCanisterOperationKindDTO::AddExisting(
                CreateExternalCanisterOperationKindAddExistingDTO { canister_id },
            ),
            name: String::from("counter"),
            description: None,
            labels: None,
            permissions: ExternalCanisterPermissionsCreateInput {
                calls: vec![],
                read: AllowDTO {
                    auth_scope: station_api::AuthScopeDTO::Restricted,
                    user_groups: vec![],
                    users: vec![],
                },
                change: AllowDTO {
                    auth_scope: station_api::AuthScopeDTO::Restricted,
                    user_groups: vec![],
                    users: vec![],
                },
            },
            request_policies: ExternalCanisterRequestPoliciesCreateInput {
                change: vec![],
                calls: vec![],
            },
        }),
    );

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
        dfx_orbit.station.request(request.clone()).await.unwrap();

        tokio::time::sleep(Duration::from_secs(1)).await;

        let controllers = dfx_orbit.get_controllers(canister_id).await.unwrap();
        assert_eq!(controllers, vec![user_test_id(1), canister_ids.station]);

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
        dfx_orbit.station.request(request.clone()).await.unwrap();

        tokio::time::sleep(Duration::from_secs(1)).await;

        let controllers = dfx_orbit.get_controllers(canister_id).await.unwrap();
        assert_eq!(controllers, vec![user_test_id(2), canister_ids.station]);

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
        dfx_orbit.station.request(request.clone()).await.unwrap();

        tokio::time::sleep(Duration::from_secs(1)).await;

        let controllers = dfx_orbit.get_controllers(canister_id).await.unwrap();
        assert_eq!(controllers, vec![user_test_id(2)]);
    });
}
