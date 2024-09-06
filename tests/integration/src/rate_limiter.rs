use crate::setup::{setup_new_env, WALLET_ADMIN_USER};
use crate::utils::{bump_time_to_avoid_ratelimit, submit_request_raw};
use crate::{CanisterIds, TestEnv};
use candid::Principal;
use orbit_essentials::api::ApiResult;
use pocket_ic::{update_candid_as, PocketIc};
use station_api::{
    AddAccountOperationInput, AllowDTO, CallExternalCanisterOperationInput, CanisterMethodDTO,
    MeResponse, RequestOperationInput,
};

const REQUEST_RATE_LIMITER_MAX_COUNT: u64 = 2000; // 2000 requests per 5mins
const REQUEST_RATE_LIMITER_MAX_SIZE: u64 = 10_000_000; // total request size of 10MB per 5mins

#[test]
fn test_request_count_rate_limiter() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let res: (ApiResult<MeResponse>,) =
        update_candid_as(&env, canister_ids.station, WALLET_ADMIN_USER, "me", ()).unwrap();
    let user_dto = res.0.unwrap().me;

    test_rate_limit(&env, canister_ids, REQUEST_RATE_LIMITER_MAX_COUNT, |i| {
        let default_permission = AllowDTO {
            auth_scope: station_api::AuthScopeDTO::Restricted,
            user_groups: vec![],
            users: vec![user_dto.id.clone()],
        };
        let add_account_operation_input = AddAccountOperationInput {
            name: format!("test{}", i),
            blockchain: "icp".to_string(),
            standard: "native".to_string(),
            read_permission: default_permission.clone(),
            configs_permission: default_permission.clone(),
            transfer_permission: default_permission,
            transfer_request_policy: None,
            configs_request_policy: None,
            metadata: vec![],
        };
        RequestOperationInput::AddAccount(add_account_operation_input)
    });
}

#[test]
fn test_request_size_rate_limiter() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let request_count = 10;
    let reqest_size = 1_000_000;
    assert_eq!(REQUEST_RATE_LIMITER_MAX_SIZE, request_count * reqest_size);
    test_rate_limit(&env, canister_ids, request_count, |_| {
        let arg_length = (reqest_size - 100) as usize; // all requests have a default size of 100
        let call_external_canister_operation_input = CallExternalCanisterOperationInput {
            validation_method: None,
            execution_method: CanisterMethodDTO {
                canister_id: Principal::anonymous(),
                method_name: "foo".to_string(),
            },
            arg: Some(vec![42; arg_length]),
            execution_method_cycles: None,
        };
        RequestOperationInput::CallExternalCanister(call_external_canister_operation_input)
    });
}

fn test_rate_limit<F>(env: &PocketIc, canister_ids: CanisterIds, rate_limit: u64, f: F)
where
    F: FnOnce(u64) -> RequestOperationInput + Copy,
{
    for _ in 0..2 {
        bump_time_to_avoid_ratelimit(env);
        for i in 0..(rate_limit + 1) {
            let res = submit_request_raw(env, WALLET_ADMIN_USER, canister_ids.station, f(i));
            if i >= rate_limit {
                let error_message = res.unwrap().0.unwrap_err().message;
                assert!(error_message
                    .unwrap()
                    .contains("The request creation has been rate-limited."));
            } else {
                res.unwrap().0.unwrap();
            }
        }
    }
}
