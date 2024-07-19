use crate::{
    cli::{dfx_orbit_test, setup_agent, TEST_PRINCIPAL},
    setup::{create_canister, setup_new_env, WALLET_ADMIN_USER},
    utils::{
        add_user, add_user_with_name, execute_request, submit_request, submit_request_approval,
        update_raw, user_test_id, wait_for_request, COUNTER_WAT,
    },
    TestEnv,
};
use candid::Principal;
use station_api::{
    AddRequestPolicyOperationInput, AuthScopeDTO, CallExternalCanisterOperationInput,
    CallExternalCanisterResourceTargetDTO, CanisterMethodDTO,
    ChangeExternalCanisterResourceTargetDTO, CreateRequestInput, EditPermissionOperationInput,
    ExecutionMethodResourceTargetDTO, ExternalCanisterResourceActionDTO, QuorumDTO,
    RequestApprovalStatusDTO, RequestOperationInput, RequestPolicyRuleDTO, RequestSpecifierDTO,
    ResourceDTO, UserSpecifierDTO, ValidationMethodResourceTargetDTO,
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

    // allow anyone to create change canister requests
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
    execute_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        add_permission,
    )
    .unwrap();

    // set four eyes principle for canister changes
    let add_request_policy =
        RequestOperationInput::AddRequestPolicy(AddRequestPolicyOperationInput {
            specifier: RequestSpecifierDTO::ChangeExternalCanister(
                ChangeExternalCanisterResourceTargetDTO::Canister(canister_id),
            ),
            rule: RequestPolicyRuleDTO::Quorum(QuorumDTO {
                approvers: UserSpecifierDTO::Any,
                min_approved: 2,
            }),
        });
    execute_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        add_request_policy,
    )
    .unwrap();

    let request_counter_canister_inc = CreateRequestInput {
        operation: RequestOperationInput::CallExternalCanister(
            CallExternalCanisterOperationInput {
                validation_method: None,
                execution_method: CanisterMethodDTO {
                    canister_id,
                    method_name: String::from("zet"),
                },
                arg: Some(42_u32.to_le_bytes().to_vec()),
                execution_method_cycles: None,
            },
        ),
        title: None,
        summary: None,
        execution_plan: None,
    };

    let request = dfx_orbit_test(&mut env, async {
        // Setup the station agent
        let mut station_agent = setup_agent(canister_ids.station).await;

        // Call the counter canister
        let request = station_agent
            .request(request_counter_canister_inc.clone())
            .await
            .unwrap();

        dbg!(&request);
        request.request
    });

    // Check that the counter has not updated yet
    let ctr = update_raw(&env, canister_id, Principal::anonymous(), "read", vec![]).unwrap();
    assert_eq!(ctr, 0_u32.to_le_bytes());

    // The other user approves the request
    // submit_request_approval(
    //     &env,
    //     other_user,
    //     canister_ids.station,
    //     request.clone(),
    //     RequestApprovalStatusDTO::Approved,
    // );
    wait_for_request(&env, other_user, canister_ids.station, request).unwrap();

    let ctr = update_raw(&env, canister_id, Principal::anonymous(), "read", vec![]).unwrap();
    assert_eq!(ctr, 1_u32.to_le_bytes());
    todo!()
}

// TODO: Test with insufficient permissions
