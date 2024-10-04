use crate::setup::{create_canister, get_canister_wasm, setup_new_env, WALLET_ADMIN_USER};
use crate::utils::{bump_time_to_avoid_ratelimit, execute_request, submit_request_raw};
use crate::{CanisterIds, TestEnv};
use candid::{Encode, Principal};
use orbit_essentials::api::ApiResult;
use pocket_ic::{update_candid_as, PocketIc};
use station_api::{
    CallExternalCanisterOperationInput, CanisterMethodDTO, CreateRequestInput,
    EditUserOperationInput, MeResponse, RequestExecutionScheduleDTO, RequestOperationInput,
};

const REQUEST_RATE_LIMITER_MAX_COUNT: usize = 2000; // 2000 requests per 5mins
const REQUEST_RATE_LIMITER_MAX_SIZE: usize = 10_000_000; // total request size of 10MB per 5mins

#[test]
fn test_request_count_rate_limiter() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    test_rate_limit(&env, canister_ids, REQUEST_RATE_LIMITER_MAX_COUNT, |id| {
        let edit_user_operation_input = EditUserOperationInput {
            id,
            name: None,
            identities: None,
            groups: None,
            status: None,
            cancel_pending_requests: None,
        };
        RequestOperationInput::EditUser(edit_user_operation_input)
    });
}

#[test]
fn test_request_size_rate_limiter() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let request_count = 10;
    let request_size = 1_000_000;
    assert_eq!(REQUEST_RATE_LIMITER_MAX_SIZE, request_count * request_size);
    test_rate_limit(&env, canister_ids, request_count, |_| {
        // the rate limiter uses the binary size of the (candid-encoded) argument
        // and thus we need to subtract the (estimated) overhead here
        let arg_length = request_size - 2000;
        let call_external_canister_operation_input = CallExternalCanisterOperationInput {
            validation_method: None,
            execution_method: CanisterMethodDTO {
                canister_id: Principal::anonymous(),
                method_name: "foo".to_string(),
            },
            arg: Some(vec![42; arg_length]),
            execution_method_cycles: None,
        };
        let operation =
            RequestOperationInput::CallExternalCanister(call_external_canister_operation_input);
        let create_request_input = CreateRequestInput {
            operation: operation.clone(),
            title: None,
            summary: None,
            execution_plan: Some(RequestExecutionScheduleDTO::Immediate),
        };
        let bytes = Encode!(&create_request_input).unwrap();
        assert!(arg_length <= bytes.len() && bytes.len() <= request_size);
        operation
    });
}

fn register_test_canister(env: &PocketIc, canister_ids: &CanisterIds) -> Principal {
    let test_canister = create_canister(env, WALLET_ADMIN_USER);
    let test_canister_wasm = get_canister_wasm("test_canister");
    env.install_canister(
        test_canister,
        test_canister_wasm,
        vec![],
        Some(WALLET_ADMIN_USER),
    );

    // register the test canister as an alternative identity of the admin
    let res: (ApiResult<MeResponse>,) =
        update_candid_as(env, canister_ids.station, WALLET_ADMIN_USER, "me", ()).unwrap();
    let admin_dto = res.0.unwrap().me;
    let edit_user = EditUserOperationInput {
        id: admin_dto.id,
        name: None,
        identities: Some(vec![WALLET_ADMIN_USER, test_canister]),
        groups: None,
        status: None,
        cancel_pending_requests: None,
    };
    execute_request(
        env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        RequestOperationInput::EditUser(edit_user),
    )
    .unwrap();

    test_canister
}

fn test_rate_limit<F>(env: &PocketIc, canister_ids: CanisterIds, rate_limit: usize, f: F)
where
    F: FnOnce(String) -> RequestOperationInput + Copy,
{
    let test_canister = register_test_canister(env, &canister_ids);

    let res: (ApiResult<MeResponse>,) =
        update_candid_as(env, canister_ids.station, WALLET_ADMIN_USER, "me", ()).unwrap();
    let admin_dto = res.0.unwrap().me;

    for _ in 0..2 {
        bump_time_to_avoid_ratelimit(env);
        let create_request_input = CreateRequestInput {
            operation: f(admin_dto.id.clone()),
            title: None,
            summary: None,
            execution_plan: Some(RequestExecutionScheduleDTO::Immediate),
        };
        let create_request_bytes = Encode!(&create_request_input).unwrap();
        update_candid_as::<_, ()>(
            env,
            test_canister,
            WALLET_ADMIN_USER,
            "call",
            (
                canister_ids.station,
                "create_request",
                create_request_bytes,
                rate_limit,
            ),
        )
        .unwrap();
        let res = submit_request_raw(
            env,
            WALLET_ADMIN_USER,
            canister_ids.station,
            f(admin_dto.id.clone()),
        );
        let error_message = res.unwrap().0.unwrap_err().message;
        assert!(error_message
            .unwrap()
            .contains("The request creation has been rate-limited."));
    }
}
