use crate::{
    dfx_orbit::{
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
use pocket_ic::PocketIc;
use station_api::{
    AddRequestPolicyOperationInput, AuthScopeDTO, CallExternalCanisterOperationInput,
    CallExternalCanisterResourceTargetDTO, CanisterMethodDTO, CreateRequestInput,
    EditPermissionOperationInput, ExecutionMethodResourceTargetDTO,
    ExternalCanisterResourceActionDTO, QuorumDTO, RequestApprovalStatusDTO, RequestOperationInput,
    RequestPolicyRuleDTO, RequestSpecifierDTO, ResourceDTO, UserSpecifierDTO,
    ValidationMethodResourceTargetDTO,
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

    let request_counter_canister_set = CreateRequestInput {
        operation: RequestOperationInput::CallExternalCanister(
            CallExternalCanisterOperationInput {
                validation_method: None,
                execution_method: CanisterMethodDTO {
                    canister_id,
                    method_name: String::from("set"),
                },
                arg: Some(42_u32.to_le_bytes().to_vec()),
                execution_method_cycles: None,
            },
        ),
        title: None,
        summary: None,
        execution_plan: None,
    };

    let request = dfx_orbit_test(&mut env, DfxOrbitTestConfig::default(), async {
        // Setup the station agent
        let dfx_orbit = setup_dfx_orbit(canister_ids.station).await;

        // Call the counter canister
        let request = dfx_orbit
            .station
            .request(request_counter_canister_set.clone())
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

// TODO: Test with insufficient permissions

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
