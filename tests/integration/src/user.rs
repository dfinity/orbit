use crate::setup::{setup_new_env, WALLET_ADMIN_USER};
use crate::utils::{
    bump_time_to_avoid_ratelimit, execute_request, get_request, submit_request, user_test_id,
};
use crate::TestEnv;
use station_api::{
    AddUserOperationInput, EditPermissionOperationInput, EditUserOperationInput,
    RequestOperationDTO, RequestOperationInput, RequestStatusDTO, SystemResourceActionDTO,
    SystemUpgradeOperationInput, SystemUpgradeTargetDTO,
};

#[test]
fn cancel_pending_requests() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    // allow anyone to create system upgrade requests
    let add_permission = RequestOperationInput::EditPermission(EditPermissionOperationInput {
        resource: station_api::ResourceDTO::System(SystemResourceActionDTO::Upgrade),
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

    // register a new user (Alice)
    let alice_user_id = user_test_id(0);
    let add_user = RequestOperationInput::AddUser(AddUserOperationInput {
        name: "Alice".to_string(),
        identities: vec![alice_user_id],
        groups: vec![],
        status: station_api::UserStatusDTO::Active,
    });
    let request_dto =
        execute_request(&env, WALLET_ADMIN_USER, canister_ids.station, add_user).unwrap();
    let alice_user_dto = match request_dto.operation {
        RequestOperationDTO::AddUser(operation) => operation.user.unwrap(),
        _ => panic!("Unexpected request operation: {:?}", request_dto.operation),
    };

    // register a new user (Bob)
    let bob_user_id = user_test_id(1);
    let add_user = RequestOperationInput::AddUser(AddUserOperationInput {
        name: "Bob".to_string(),
        identities: vec![bob_user_id],
        groups: vec![],
        status: station_api::UserStatusDTO::Active,
    });
    execute_request(&env, WALLET_ADMIN_USER, canister_ids.station, add_user).unwrap();

    // Alice makes a bunch of system upgrade requests
    let station_upgrade = RequestOperationInput::SystemUpgrade(SystemUpgradeOperationInput {
        target: SystemUpgradeTargetDTO::UpgradeStation,
        module: vec![],
        module_extra_chunks: None,
        arg: None,
        take_backup_snapshot: None,
    });
    let mut alice_request_dtos = vec![];
    for _ in 0..10 {
        let request_dto = submit_request(
            &env,
            alice_user_id,
            canister_ids.station,
            station_upgrade.clone(),
        );
        bump_time_to_avoid_ratelimit(&env);
        alice_request_dtos.push(request_dto);
    }
    for request_dto in alice_request_dtos.clone() {
        let new_request_dto = get_request(&env, alice_user_id, canister_ids.station, request_dto);
        match new_request_dto.status {
            RequestStatusDTO::Created => (),
            _ => panic!("Unexpected request status: {:?}", new_request_dto.status),
        };
    }

    // Bob makes a bunch of system upgrade requests
    let mut bob_request_dtos = vec![];
    for _ in 0..10 {
        let request_dto = submit_request(
            &env,
            bob_user_id,
            canister_ids.station,
            station_upgrade.clone(),
        );
        bump_time_to_avoid_ratelimit(&env);
        bob_request_dtos.push(request_dto);
    }
    for request_dto in bob_request_dtos.clone() {
        let new_request_dto = get_request(&env, bob_user_id, canister_ids.station, request_dto);
        match new_request_dto.status {
            RequestStatusDTO::Created => (),
            _ => panic!("Unexpected request status: {:?}", new_request_dto.status),
        };
    }

    // the admin makes a no-op `EditUserOperation`
    let edit_user = RequestOperationInput::EditUser(EditUserOperationInput {
        id: alice_user_dto.id.clone(),
        name: None,
        identities: None,
        groups: None,
        status: None,
        cancel_pending_requests: None,
    });
    execute_request(&env, WALLET_ADMIN_USER, canister_ids.station, edit_user).unwrap();
    for request_dto in alice_request_dtos.clone() {
        let new_request_dto = get_request(&env, alice_user_id, canister_ids.station, request_dto);
        match new_request_dto.status {
            RequestStatusDTO::Created => (),
            _ => panic!("Unexpected request status: {:?}", new_request_dto.status),
        };
    }

    // now the admin cancels all Alice's pending requests
    let edit_user = RequestOperationInput::EditUser(EditUserOperationInput {
        id: alice_user_dto.id,
        name: None,
        identities: None,
        groups: None,
        status: None,
        cancel_pending_requests: Some(true),
    });
    execute_request(&env, WALLET_ADMIN_USER, canister_ids.station, edit_user).unwrap();
    for request_dto in alice_request_dtos {
        let new_request_dto = get_request(&env, alice_user_id, canister_ids.station, request_dto);
        match new_request_dto.status {
            RequestStatusDTO::Cancelled { reason } => assert!(reason
                .unwrap()
                .contains("The request has been cancelled by an `EditUserOperation`.")),
            _ => panic!("Unexpected request status: {:?}", new_request_dto.status),
        };
    }
    // leaving Bob's pending requests unchanged
    for request_dto in bob_request_dtos {
        let new_request_dto = get_request(&env, bob_user_id, canister_ids.station, request_dto);
        match new_request_dto.status {
            RequestStatusDTO::Created => (),
            _ => panic!("Unexpected request status: {:?}", new_request_dto.status),
        };
    }
}
