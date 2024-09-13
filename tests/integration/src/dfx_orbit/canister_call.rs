use crate::{
    dfx_orbit::setup::{
        dfx_orbit_test, setup_counter_canister, setup_dfx_orbit, setup_dfx_user, DfxOrbitTestConfig,
    },
    setup::{setup_new_env, WALLET_ADMIN_USER},
    utils::{
        add_user, execute_request, submit_request_approval, update_raw, user_test_id,
        wait_for_request,
    },
    CanisterIds, TestEnv,
};
use candid::Principal;
use dfx_orbit::args::{
    request::{
        canister::{RequestCanisterActionArgs, RequestCanisterArgs, RequestCanisterCallArgs},
        RequestArgs, RequestArgsActions,
    },
    verify::{VerifyArgs, VerifyArgsAction, VerifyCanisterActionArgs, VerifyCanisterArgs},
};
use pocket_ic::PocketIc;
use station_api::{
    AddRequestPolicyOperationInput, AuthScopeDTO, CallExternalCanisterResourceTargetDTO,
    EditPermissionOperationInput, ExecutionMethodResourceTargetDTO,
    ExternalCanisterResourceActionDTO, GetRequestInput, QuorumDTO, RequestApprovalStatusDTO,
    RequestOperationInput, RequestPolicyRuleDTO, RequestSpecifierDTO, ResourceDTO,
    UserSpecifierDTO, ValidationMethodResourceTargetDTO,
};

/// Test a canister call through orbit using the station agent
#[test]
fn canister_call() {
    let TestEnv {
        mut env,
        canister_ids,
        ..
    } = setup_new_env();

    let canister_id = setup_counter_canister(&mut env, &canister_ids);

    setup_dfx_user(&env, &canister_ids);
    let other_user = user_test_id(1);
    add_user(&env, other_user, vec![], canister_ids.station);

    permit_call_operation(&env, &canister_ids);
    set_four_eyes_on_call(&env, &canister_ids);

    let inner_args = RequestCanisterCallArgs {
        canister: String::from("counter"),
        method_name: String::from("set"),
        argument: None,
        arg_file: None,
        raw_arg: Some(String::from("2a000000")),
        with_cycles: None,
    };

    let config = DfxOrbitTestConfig {
        canister_ids: vec![(String::from("counter"), canister_id.clone())],
        ..Default::default()
    };

    let request = dfx_orbit_test(&mut env, config, async {
        // Setup the station agent
        let dfx_orbit = setup_dfx_orbit(canister_ids.station).await;

        // Call the counter canister
        let request = RequestArgs {
            title: None,
            summary: None,
            action: RequestArgsActions::Canister(RequestCanisterArgs {
                action: RequestCanisterActionArgs::Call(inner_args.clone()),
            }),
        }
        .into_request(&dfx_orbit)
        .await
        .unwrap();

        let request = dfx_orbit.station.request(request.clone()).await.unwrap();

        // Check that the request verifies
        let req_response = dfx_orbit
            .station
            .review_id(GetRequestInput {
                request_id: request.request.id.clone(),
            })
            .await
            .unwrap();

        VerifyArgs {
            request_id: request.request.id.clone(),
            and_approve: false,
            or_reject: false,
            action: VerifyArgsAction::Canister(VerifyCanisterArgs {
                action: VerifyCanisterActionArgs::Call(inner_args),
            }),
        }
        .verify(&dfx_orbit, &req_response)
        .await
        .unwrap();

        request.request
    });

    // Check that the counter has not updated yet
    let ctr = update_raw(&env, canister_id, Principal::anonymous(), "read", vec![]).unwrap();
    assert_eq!(ctr, 0_u32.to_le_bytes());

    // The other user approves the request
    submit_request_approval(
        &env,
        other_user,
        canister_ids.station,
        request.clone(),
        RequestApprovalStatusDTO::Approved,
    );
    wait_for_request(&env, other_user, canister_ids.station, request).unwrap();

    let ctr = update_raw(&env, canister_id, Principal::anonymous(), "read", vec![]).unwrap();
    assert_eq!(ctr, 42_u32.to_le_bytes());
}

/// Allow anyone to create change canister requests
pub(crate) fn permit_call_operation(env: &PocketIc, canister_ids: &CanisterIds) {
    let add_permission = RequestOperationInput::EditPermission(EditPermissionOperationInput {
        resource: ResourceDTO::ExternalCanister(ExternalCanisterResourceActionDTO::Call(
            CallExternalCanisterResourceTargetDTO {
                validation_method: ValidationMethodResourceTargetDTO::No,
                execution_method: ExecutionMethodResourceTargetDTO::Any,
            },
        )),
        auth_scope: Some(AuthScopeDTO::Authenticated),
        user_groups: None,
        users: None,
    });
    execute_request(env, WALLET_ADMIN_USER, canister_ids.station, add_permission).unwrap();
}

/// Set four eyes principle for canister calls
pub(crate) fn set_four_eyes_on_call(env: &PocketIc, canister_ids: &CanisterIds) {
    let add_request_policy =
        RequestOperationInput::AddRequestPolicy(AddRequestPolicyOperationInput {
            specifier: RequestSpecifierDTO::CallExternalCanister(
                CallExternalCanisterResourceTargetDTO {
                    validation_method: ValidationMethodResourceTargetDTO::No,
                    execution_method: ExecutionMethodResourceTargetDTO::Any,
                },
            ),
            rule: RequestPolicyRuleDTO::Quorum(QuorumDTO {
                approvers: UserSpecifierDTO::Any,
                min_approved: 2,
            }),
        });
    execute_request(
        env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        add_request_policy,
    )
    .unwrap();
}
