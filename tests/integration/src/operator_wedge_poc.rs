use std::time::Duration;

use candid::Principal;
use pocket_ic::PocketIc;
use station_api::{
    AddUserGroupOperationInput, AllowDTO, AuthScopeDTO, CallExternalCanisterOperationInput,
    CanisterMethodDTO, CreateExternalCanisterOperationInput,
    CreateExternalCanisterOperationKindAddExistingDTO, CreateExternalCanisterOperationKindDTO,
    ExternalCanisterCallPermissionDTO, ExternalCanisterCallRequestPolicyRuleInput,
    ExternalCanisterPermissionsCreateInput, ExternalCanisterRequestPoliciesCreateInput, QuorumDTO,
    RequestDTO, RequestOperationInput, RequestPolicyRuleDTO, RequestStatusDTO, UserSpecifierDTO,
    ValidationMethodResourceTargetDTO,
};

use crate::{
    setup::{setup_new_env, WALLET_ADMIN_USER},
    utils::{
        add_user, deploy_test_canister, execute_request, submit_request, try_get_request,
        user_test_id, OPERATOR_GROUP_ID,
    },
    TestEnv,
};

fn tick(env: &PocketIc, n: usize) {
    for _ in 0..n {
        env.advance_time(Duration::from_secs(5));
        env.tick();
    }
}

fn call(
    env: &PocketIc,
    station: Principal,
    who: Principal,
    callee: Principal,
    method: &str,
) -> RequestDTO {
    let op = RequestOperationInput::CallExternalCanister(CallExternalCanisterOperationInput {
        validation_method: None,
        execution_method: CanisterMethodDTO {
            canister_id: callee,
            method_name: method.to_string(),
        },
        arg: None,
        execution_method_cycles: None,
    });
    submit_request(env, who, station, op)
}

fn status(
    env: &PocketIc,
    station: Principal,
    reader: Principal,
    req: &RequestDTO,
) -> RequestStatusDTO {
    try_get_request(env, reader, station, req.clone())
        .unwrap()
        .unwrap()
        .status
}

#[test]
fn operator_wedges_station() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();
    let alice = WALLET_ADMIN_USER;
    let station = canister_ids.station;
    let op_group = OPERATOR_GROUP_ID.hyphenated().to_string();

    let bob = user_test_id(10);
    let carol = user_test_id(11);
    let dave = user_test_id(12);
    for id in [bob, carol, dave] {
        add_user(&env, id, vec![op_group.clone()], station);
    }

    let trap = deploy_test_canister(&env, bob);

    let allow_ops = AllowDTO {
        auth_scope: AuthScopeDTO::Restricted,
        users: vec![],
        user_groups: vec![op_group.clone()],
    };
    let ops_rule = RequestPolicyRuleDTO::Quorum(QuorumDTO {
        approvers: UserSpecifierDTO::Group(vec![op_group.clone()]),
        min_approved: 1,
    });
    let mk_perm = |m: &str| ExternalCanisterCallPermissionDTO {
        allow: allow_ops.clone(),
        validation_method: ValidationMethodResourceTargetDTO::No,
        execution_method: m.to_string(),
    };
    let mk_pol = |m: &str| ExternalCanisterCallRequestPolicyRuleInput {
        policy_id: None,
        rule: ops_rule.clone(),
        validation_method: ValidationMethodResourceTargetDTO::No,
        execution_method: m.to_string(),
    };
    execute_request(
        &env,
        bob,
        station,
        RequestOperationInput::CreateExternalCanister(CreateExternalCanisterOperationInput {
            kind: CreateExternalCanisterOperationKindDTO::AddExisting(
                CreateExternalCanisterOperationKindAddExistingDTO { canister_id: trap },
            ),
            name: "ops-trap".to_string(),
            description: None,
            labels: None,
            metadata: None,
            permissions: ExternalCanisterPermissionsCreateInput {
                read: allow_ops.clone(),
                change: allow_ops.clone(),
                calls: vec![mk_perm("noop"), mk_perm("unstoppable")],
            },
            request_policies: ExternalCanisterRequestPoliciesCreateInput {
                change: vec![],
                calls: vec![mk_pol("noop"), mk_pol("unstoppable")],
            },
        }),
    )
    .unwrap();

    let before = call(&env, station, carol, trap, "noop");
    tick(&env, 15);
    let before_status = status(&env, station, alice, &before);
    println!("before     = {before_status:?}");

    let park = call(&env, station, bob, trap, "unstoppable");
    tick(&env, 15);
    println!("hung_call  = {:?}", status(&env, station, alice, &park));

    let peer = call(&env, station, carol, trap, "noop");
    let admin = submit_request(
        &env,
        alice,
        station,
        RequestOperationInput::AddUserGroup(AddUserGroupOperationInput {
            name: "recovery".to_string(),
        }),
    );
    tick(&env, 80);

    let park_final = status(&env, station, alice, &park);
    let peer_status = status(&env, station, alice, &peer);
    let admin_status = status(&env, station, alice, &admin);

    println!("peer       = {peer_status:?}");
    println!("admin      = {admin_status:?}");
    println!("hung_later = {park_final:?}");

    assert!(
        matches!(before_status, RequestStatusDTO::Completed { .. }),
        "before {before_status:?}"
    );
    assert!(
        matches!(park_final, RequestStatusDTO::Processing { .. }),
        "hung {park_final:?}"
    );
    assert!(
        !matches!(peer_status, RequestStatusDTO::Completed { .. }),
        "peer {peer_status:?}"
    );
    assert!(
        !matches!(admin_status, RequestStatusDTO::Completed { .. }),
        "admin {admin_status:?}"
    );
}
