use crate::setup::{setup_new_env, WALLET_ADMIN_USER};
use crate::utils::submit_request_raw;
use crate::TestEnv;
use orbit_essentials::api::ApiResult;
use pocket_ic::{update_candid_as, CallError};
use station_api::{AddAccountOperationInput, AllowDTO, MeResponse, RequestOperationInput};
use std::time::Duration;

#[test]
fn global_rate_limiter() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let res: (ApiResult<MeResponse>,) =
        update_candid_as(&env, canister_ids.station, WALLET_ADMIN_USER, "me", ()).unwrap();
    let user_dto = res.0.unwrap().me;

    for _ in 0..2 {
        for i in 0..101 {
            let add_account_operation_input = AddAccountOperationInput {
                name: format!("test{}", i),
                blockchain: "icp".to_string(),
                standard: "native".to_string(),
                read_permission: AllowDTO {
                    auth_scope: station_api::AuthScopeDTO::Restricted,
                    user_groups: vec![],
                    users: vec![user_dto.id.clone()],
                },
                configs_permission: AllowDTO {
                    auth_scope: station_api::AuthScopeDTO::Restricted,
                    user_groups: vec![],
                    users: vec![user_dto.id.clone()],
                },
                transfer_permission: AllowDTO {
                    auth_scope: station_api::AuthScopeDTO::Restricted,
                    user_groups: vec![],
                    users: vec![user_dto.id.clone()],
                },
                transfer_request_policy: None,
                configs_request_policy: None,
                metadata: vec![],
            };
            let res = submit_request_raw(
                &env,
                WALLET_ADMIN_USER,
                canister_ids.station,
                RequestOperationInput::AddAccount(add_account_operation_input),
            );
            if i >= 100 {
                let call_error = res.unwrap_err();
                match call_error {
                    CallError::UserError(user_error) => {
                        assert!(user_error.description.contains(&format!(
                            "User call rate-limit for user {} exceeded.",
                            user_dto.id
                        )));
                    }
                    CallError::Reject(reject) => panic!("Unexpected reject: {}", reject),
                };
            } else {
                res.unwrap().0.unwrap();
            }
        }
        // the rate limiter aggregation window is 1h and resolution is 1s
        env.advance_time(Duration::from_secs(3601));
    }
}
