use crate::interfaces::{default_account, send_icp_to_account, ICP};
use crate::setup::{setup_new_env, WALLET_ADMIN_USER};
use crate::utils::{execute_request, submit_request_raw, user_test_id};
use crate::CanisterIds;
use crate::TestEnv;
use ic_ledger_types::AccountIdentifier;
use orbit_essentials::api::ApiResult;
use pocket_ic::update_candid_as;
use pocket_ic::PocketIc;
use station_api::{
    AddAccountOperationInput, AllowDTO, MeResponse, RequestOperationDTO, RequestOperationInput,
    TransferOperationInput,
};
use std::time::Duration;

const TRANSFER_RATE_LIMIT: u64 = 1000;
const MISC_RATE_LIMIT: u64 = 10;

#[test]
fn transfer_rate_limiter() {
    let TestEnv {
        env,
        canister_ids,
        controller,
        ..
    } = setup_new_env();

    let res: (ApiResult<MeResponse>,) =
        update_candid_as(&env, canister_ids.station, WALLET_ADMIN_USER, "me", ()).unwrap();
    let user_dto = res.0.unwrap().me;

    let add_account_operation_input = AddAccountOperationInput {
        name: "test".to_string(),
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
    let request = execute_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        RequestOperationInput::AddAccount(add_account_operation_input),
    )
    .unwrap();
    let account_dto = match request.operation {
        RequestOperationDTO::AddAccount(add_account) => add_account.account.unwrap(),
        _ => {
            panic!("Unexpected request operation: {:?}", request.operation);
        }
    };

    // send ICP to orbit station account
    let account_address = AccountIdentifier::from_hex(&account_dto.address).unwrap();
    send_icp_to_account(&env, controller, account_address, 100_000_000, 0, None).unwrap();

    test_rate_limit(&env, canister_ids, TRANSFER_RATE_LIMIT, |_| {
        let beneficiary_id = user_test_id(1);
        let transfer_operation_input = TransferOperationInput {
            from_account_id: account_dto.id.clone(),
            to: default_account(beneficiary_id),
            amount: ICP.into(),
            fee: None,
            metadata: vec![],
            network: None,
        };
        RequestOperationInput::Transfer(transfer_operation_input)
    });
}

#[test]
fn misc_rate_limiter() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let res: (ApiResult<MeResponse>,) =
        update_candid_as(&env, canister_ids.station, WALLET_ADMIN_USER, "me", ()).unwrap();
    let user_dto = res.0.unwrap().me;

    test_rate_limit(&env, canister_ids, MISC_RATE_LIMIT, |i| {
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
        RequestOperationInput::AddAccount(add_account_operation_input)
    });
}

fn test_rate_limit<F>(env: &PocketIc, canister_ids: CanisterIds, rate_limit: u64, f: F)
where
    F: FnOnce(u64) -> RequestOperationInput + Copy,
{
    for _ in 0..2 {
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
        // the rate limiter aggregation window is 1h and resolution is 1s
        env.advance_time(Duration::from_secs(3601));
    }
}
