use std::time::Duration;

use crate::interfaces::mint_icp;
use crate::setup::{setup_new_env, WALLET_ADMIN_USER};
use crate::utils::{create_account, expect_await_call_result, get_icp_asset};
use crate::TestEnv;
use candid::Encode;
use ic_ledger_types::AccountIdentifier;
use orbit_essentials::api::ApiResult;
use pocket_ic::update_candid_as;
use station_api::{
    AddAccountOperationInput, AllowDTO, FetchAccountBalancesInput, FetchAccountBalancesResponse,
    MeResponse, RequestPolicyRuleDTO,
};

#[test]
fn test_fetch_balances() {
    let TestEnv {
        env,
        canister_ids,
        // controller,
        minter,
        ..
    } = setup_new_env();

    // register user
    let res: (ApiResult<MeResponse>,) =
        update_candid_as(&env, canister_ids.station, WALLET_ADMIN_USER, "me", ()).unwrap();
    let user_dto = res.0.unwrap().me;

    let icp_asset = get_icp_asset(&env, canister_ids.station, WALLET_ADMIN_USER);

    let permission = AllowDTO {
        auth_scope: station_api::AuthScopeDTO::Restricted,
        user_groups: vec![],
        users: vec![user_dto.id.clone()],
    };

    let account = create_account(
        &env,
        canister_ids.station,
        user_dto.identities[0],
        AddAccountOperationInput {
            name: "test account".to_owned(),
            assets: vec![icp_asset.id.clone()],
            metadata: vec![],
            read_permission: permission.clone(),
            configs_permission: permission.clone(),
            transfer_permission: permission.clone(),
            configs_request_policy: Some(RequestPolicyRuleDTO::AutoApproved),
            transfer_request_policy: Some(RequestPolicyRuleDTO::AutoApproved),
        },
    );

    let icp_account_identifier = AccountIdentifier::from_hex(
        &account
            .addresses
            .iter()
            .find(|a| a.format == "icp_account_identifier")
            .expect("cannot get ICP account identifier")
            .address,
    )
    .expect("cannot parse ICP account identifier");

    mint_icp(&env, minter, &icp_account_identifier, 10 * 100_000_000)
        .expect("failed to mint ICP to account");

    let messages_ids = [
        env.submit_call(
            canister_ids.station,
            user_dto.identities[0],
            "fetch_account_balances",
            Encode!(&FetchAccountBalancesInput {
                account_ids: vec![account.id.clone()],
            })
            .unwrap(),
        )
        .expect("failed to submit call"),
        env.submit_call(
            canister_ids.station,
            user_dto.identities[0],
            "fetch_account_balances",
            Encode!(&FetchAccountBalancesInput {
                account_ids: vec![account.id.clone()],
            })
            .unwrap(),
        )
        .expect("failed to submit call"),
    ];

    let results = messages_ids
        .into_iter()
        .map(|message_id| {
            expect_await_call_result::<(ApiResult<FetchAccountBalancesResponse>,)>(
                env.await_call(message_id).expect("failed to await call"),
            )
            .0
            .expect("failed to get result")
        })
        .collect::<Vec<_>>();

    results.iter().any(|result| {
        result.balances[0]
            .as_ref()
            .is_some_and(|account_balance| account_balance.query_state == "fresh")
    });
    results.iter().any(|result| result.balances[0].is_none());

    let messages_ids = [
        env.submit_call(
            canister_ids.station,
            user_dto.identities[0],
            "fetch_account_balances",
            Encode!(&FetchAccountBalancesInput {
                account_ids: vec![account.id.clone()],
            })
            .unwrap(),
        )
        .expect("failed to submit call"),
        env.submit_call(
            canister_ids.station,
            user_dto.identities[0],
            "fetch_account_balances",
            Encode!(&FetchAccountBalancesInput {
                account_ids: vec![account.id.clone()],
            })
            .unwrap(),
        )
        .expect("failed to submit call"),
    ];

    let results = messages_ids
        .into_iter()
        .map(|message_id| {
            expect_await_call_result::<(ApiResult<FetchAccountBalancesResponse>,)>(
                env.await_call(message_id).expect("failed to await call"),
            )
            .0
            .expect("failed to get result")
        })
        .collect::<Vec<_>>();

    results.iter().all(|result| {
        result.balances[0]
            .as_ref()
            .is_some_and(|account_balance| account_balance.query_state == "fresh")
    });

    env.advance_time(Duration::from_secs(10));

    let messages_ids = [
        env.submit_call(
            canister_ids.station,
            user_dto.identities[0],
            "fetch_account_balances",
            Encode!(&FetchAccountBalancesInput {
                account_ids: vec![account.id.clone()],
            })
            .unwrap(),
        )
        .expect("failed to submit call"),
        env.submit_call(
            canister_ids.station,
            user_dto.identities[0],
            "fetch_account_balances",
            Encode!(&FetchAccountBalancesInput {
                account_ids: vec![account.id.clone()],
            })
            .unwrap(),
        )
        .expect("failed to submit call"),
    ];

    let results = messages_ids
        .into_iter()
        .map(|message_id| {
            expect_await_call_result::<(ApiResult<FetchAccountBalancesResponse>,)>(
                env.await_call(message_id).expect("failed to await call"),
            )
            .0
            .expect("failed to get result")
        })
        .collect::<Vec<_>>();

    results.iter().any(|result| {
        result.balances[0]
            .as_ref()
            .is_some_and(|account_balance| account_balance.query_state == "fresh")
    });
    results.iter().any(|result| {
        result.balances[0]
            .as_ref()
            .is_some_and(|account_balance| account_balance.query_state == "stale_refreshing")
    });
}
