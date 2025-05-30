use crate::interfaces::{
    default_account, deploy_icrc1_token, get_icp_balance, get_icrc1_balance_of, mint_icp,
    mint_icrc1_tokens, send_icp, send_icp_to_account, ArchiveOptions, Icrc1LedgerInitArgs, ICP,
    ICP_FEE,
};
use crate::setup::{setup_new_env, WALLET_ADMIN_USER};
use crate::station_test_data::asset::add_asset_with_input;
use crate::utils::{
    create_account, create_transfer, fetch_account_balances, get_icp_account_identifier,
    get_icp_asset, user_test_id,
};
use crate::TestEnv;
use candid::Principal;
use ic_ledger_types::AccountIdentifier;
use orbit_essentials::api::ApiResult;
use pocket_ic::{query_candid_as, update_candid_as};
use station_api::{
    AddAccountOperationInput, AddAssetOperationInput, AllowDTO, ApiErrorDTO, CreateRequestInput,
    CreateRequestResponse, GetRequestInput, GetRequestResponse, GetTransfersInput,
    GetTransfersResponse, ListAccountTransfersInput, ListAccountTransfersResponse, MeResponse,
    MetadataDTO, QuorumPercentageDTO, RequestExecutionScheduleDTO, RequestOperationDTO,
    RequestOperationInput, RequestPolicyRuleDTO, RequestStatusDTO, TransferOperationInput,
    UserSpecifierDTO,
};
use std::str::FromStr;
use std::time::Duration;

#[test]
fn make_transfer_successful() {
    let TestEnv {
        env,
        canister_ids,
        controller,
        ..
    } = setup_new_env();

    let beneficiary_id = user_test_id(1);

    // register user
    let res: (ApiResult<MeResponse>,) =
        update_candid_as(&env, canister_ids.station, WALLET_ADMIN_USER, "me", ()).unwrap();
    let user_dto = res.0.unwrap().me;

    let icp_asset = get_icp_asset(&env, canister_ids.station, WALLET_ADMIN_USER);

    // create account
    let create_account_args = AddAccountOperationInput {
        name: "test".to_string(),
        assets: vec![icp_asset.id.clone()],
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
        transfer_request_policy: Some(RequestPolicyRuleDTO::QuorumPercentage(
            QuorumPercentageDTO {
                approvers: UserSpecifierDTO::Id(vec![user_dto.id.clone()]),
                min_approved: 100,
            },
        )),
        configs_request_policy: Some(RequestPolicyRuleDTO::QuorumPercentage(
            QuorumPercentageDTO {
                approvers: UserSpecifierDTO::Id(vec![user_dto.id.clone()]),
                min_approved: 100,
            },
        )),
        metadata: vec![],
    };
    let add_account_request = CreateRequestInput {
        operation: RequestOperationInput::AddAccount(create_account_args),
        title: None,
        summary: None,
        execution_plan: Some(RequestExecutionScheduleDTO::Immediate),
        expiration_dt: None,
    };
    let res: (ApiResult<CreateRequestResponse>,) = update_candid_as(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        "create_request",
        (add_account_request,),
    )
    .unwrap();

    // wait for the request to be approved (timer's period is 5 seconds)
    env.advance_time(Duration::from_secs(5));
    env.tick();

    let account_creation_request_dto = res.0.unwrap().request;
    match account_creation_request_dto.status {
        RequestStatusDTO::Approved => {}
        _ => {
            panic!("request must be approved by now");
        }
    };

    // wait for the request to be executed (timer's period is 5 seconds)
    env.advance_time(Duration::from_secs(5));
    env.tick();

    // fetch the created account id from the request
    let get_request_args = GetRequestInput {
        request_id: account_creation_request_dto.id,
        with_full_info: Some(false),
    };
    let res: (ApiResult<CreateRequestResponse>,) = update_candid_as(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        "get_request",
        (get_request_args,),
    )
    .unwrap();
    let finalized_request = res.0.unwrap().request;
    match finalized_request.status {
        RequestStatusDTO::Completed { .. } => {}
        _ => {
            panic!(
                "request must be completed by now but instead is {:?}",
                finalized_request.status
            );
        }
    };

    let account_dto = match finalized_request.operation {
        RequestOperationDTO::AddAccount(add_account) => add_account.account.unwrap(),
        _ => {
            panic!("request must be AddAccount");
        }
    };

    // send ICP to user
    send_icp(&env, controller, WALLET_ADMIN_USER, ICP + 2 * ICP_FEE, 0).unwrap();
    let user_balance = get_icp_balance(&env, WALLET_ADMIN_USER);
    assert_eq!(user_balance, ICP + 2 * ICP_FEE);

    // send ICP to orbit station account
    let account_address = AccountIdentifier::from_hex(
        &get_icp_account_identifier(&account_dto.addresses).expect("no icp address found"),
    )
    .unwrap();
    send_icp_to_account(
        &env,
        WALLET_ADMIN_USER,
        account_address,
        ICP + ICP_FEE,
        0,
        None,
        None,
    )
    .unwrap();

    // check user balance after transfer to orbit station account
    let new_user_balance = get_icp_balance(&env, WALLET_ADMIN_USER);
    assert_eq!(new_user_balance, 0);

    // check beneficiary balance
    let old_beneficiary_balance = get_icp_balance(&env, beneficiary_id);
    assert_eq!(old_beneficiary_balance, 0);

    // make transfer request to beneficiary
    let transfer = TransferOperationInput {
        from_account_id: account_dto.id.clone(),
        from_asset_id: icp_asset.id.clone(),
        with_standard: "icp_native".to_string(),
        to: default_account(beneficiary_id),
        amount: ICP.into(),
        fee: None,
        metadata: vec![],
        network: None,
    };
    let transfer_request = CreateRequestInput {
        operation: RequestOperationInput::Transfer(transfer),
        title: None,
        summary: None,
        execution_plan: Some(RequestExecutionScheduleDTO::Immediate),
        expiration_dt: None,
    };
    let res: (Result<CreateRequestResponse, ApiErrorDTO>,) = update_candid_as(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        "create_request",
        (transfer_request,),
    )
    .unwrap();
    let request_dto = res.0.unwrap().request;

    // wait for the request to be approved (timer's period is 5 seconds)
    env.advance_time(Duration::from_secs(5));
    env.tick();
    // wait for the request to be processing (timer's period is 5 seconds) and first is set to processing
    env.advance_time(Duration::from_secs(5));
    env.tick();
    env.tick();
    env.tick();
    env.advance_time(Duration::from_secs(5));
    env.tick();
    env.tick();
    env.tick();

    // check transfer request status
    let get_request_args = GetRequestInput {
        request_id: request_dto.id.clone(),
        with_full_info: Some(false),
    };
    let res: (Result<GetRequestResponse, ApiErrorDTO>,) = update_candid_as(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        "get_request",
        (get_request_args,),
    )
    .unwrap();
    let new_request_dto = res.0.unwrap().request;
    match new_request_dto.status {
        RequestStatusDTO::Completed { .. } => {}
        _ => {
            panic!(
                "request must be completed by now but instead is {:?}",
                new_request_dto.status
            );
        }
    };

    // request has the transfer id filled out
    let transfer_id = match new_request_dto.operation {
        RequestOperationDTO::Transfer(transfer) => transfer
            .transfer_id
            .expect("transfer id must be set for completed transfer"),
        _ => {
            panic!("request must be Transfer");
        }
    };

    // fetch the transfer and check if its request id matches the request id that created it
    let res: (Result<GetTransfersResponse, ApiErrorDTO>,) = query_candid_as(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        "get_transfers",
        (GetTransfersInput {
            transfer_ids: vec![transfer_id],
        },),
    )
    .unwrap();

    let request_id_in_transfer_dto = res
        .0
        .unwrap()
        .transfers
        .first()
        .expect("One transaction must be returned")
        .request_id
        .clone();

    assert_eq!(request_id_in_transfer_dto, request_dto.id);

    // check beneficiary balance after completed transfer
    let new_beneficiary_balance = get_icp_balance(&env, beneficiary_id);
    assert_eq!(new_beneficiary_balance, ICP);

    // load account transfers
    let res: (Result<ListAccountTransfersResponse, ApiErrorDTO>,) = query_candid_as(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        "list_account_transfers",
        (ListAccountTransfersInput {
            account_id: account_dto.id,
            from_dt: None,
            to_dt: None,
            status: None,
        },),
    )
    .unwrap();

    // transactions should be completed and have transaction hash
    let all_have_transaction_hash = res.0.unwrap().transfers.iter().all(|t| match &t.status {
        station_api::TransferStatusDTO::Completed { hash, .. } => hash.is_some(),
        _ => {
            panic!("transfer should be completed");
        }
    });

    assert!(all_have_transaction_hash);
}

#[test]
fn make_icrc1_transfer() {
    let TestEnv {
        mut env,
        canister_ids,
        // controller,
        ..
    } = setup_new_env();

    let beneficiary_id = user_test_id(1);

    // register user
    let res: (ApiResult<MeResponse>,) =
        update_candid_as(&env, canister_ids.station, WALLET_ADMIN_USER, "me", ()).unwrap();
    let user_dto = res.0.unwrap().me;

    let ledger_controller = Principal::from_slice(&[99; 29]);

    let token_ledger_canister_id = deploy_icrc1_token(
        &mut env,
        ledger_controller,
        Icrc1LedgerInitArgs {
            minting_account: icrc_ledger_types::icrc1::account::Account {
                owner: ledger_controller,
                subaccount: None,
            },
            fee_collector_account: None,
            initial_balances: vec![],
            transfer_fee: 50u64.into(),
            decimals: Some(12),
            token_name: "TEST_ICRC1".to_owned(),
            token_symbol: "TST".to_owned(),
            metadata: vec![],
            archive_options: ArchiveOptions {
                trigger_threshold: 1000,
                num_blocks_to_archive: 1000,
                node_max_memory_size_bytes: None,
                max_message_size_bytes: None,
                controller_id: ledger_controller,
                more_controller_ids: None,
                cycles_for_archive_creation: None,
                max_transactions_per_response: None,
            },
            max_memo_length: None,
            feature_flags: None,
            maximum_number_of_accounts: None,
            accounts_overflow_trim_quantity: None,
        },
    );

    let asset = add_asset_with_input(
        &env,
        canister_ids.station,
        user_dto.identities[0],
        AddAssetOperationInput {
            name: "Test ICRC1 Token".to_owned(),
            blockchain: "icp".to_owned(),
            standards: vec!["icrc1".to_owned()],
            symbol: "TEST".to_owned(),
            decimals: 4,
            metadata: vec![
                MetadataDTO {
                    key: "ledger_canister_id".to_owned(),
                    value: Principal::to_text(&token_ledger_canister_id),
                },
                MetadataDTO {
                    key: "index_canister_id".to_owned(),
                    value: Principal::to_text(&token_ledger_canister_id),
                },
            ],
        },
    );

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
            assets: vec![asset.id.clone()],
            metadata: vec![],
            read_permission: permission.clone(),
            configs_permission: permission.clone(),
            transfer_permission: permission.clone(),
            configs_request_policy: Some(RequestPolicyRuleDTO::AutoApproved),
            transfer_request_policy: Some(RequestPolicyRuleDTO::AutoApproved),
        },
    );

    let station_account_icrc1_account =
        icrc_ledger_types::icrc1::account::Account::from_str(&account.addresses[0].address)
            .expect("invalid account address");

    mint_icrc1_tokens(
        &env,
        token_ledger_canister_id,
        ledger_controller,
        station_account_icrc1_account,
        1_000_000,
    )
    .expect("failed to mint icrc1 tokens");

    let to_address = icrc_ledger_types::icrc1::account::Account {
        owner: beneficiary_id,
        subaccount: None,
    }
    .to_string();

    create_transfer(
        &env,
        canister_ids.station,
        user_dto.identities[0],
        station_api::TransferOperationInput {
            from_account_id: account.id.clone(),
            from_asset_id: asset.id.clone(),
            with_standard: "icrc1".to_owned(),
            to: to_address.clone(),
            amount: candid::Nat::from(100u128),
            fee: Some(50u64.into()),
            metadata: vec![],
            network: None,
        },
    );

    let balance = get_icrc1_balance_of(
        &env,
        token_ledger_canister_id,
        icrc_ledger_types::icrc1::account::Account {
            owner: beneficiary_id,
            subaccount: None,
        },
    );

    assert_eq!(balance, candid::Nat::from(100u128));

    let account_balances = fetch_account_balances(
        &env,
        canister_ids.station,
        user_dto.identities[0],
        station_api::FetchAccountBalancesInput {
            account_ids: vec![account.id.clone()],
        },
    );

    assert_eq!(
        account_balances.balances[0]
            .as_ref()
            .expect("should have balance")
            .balance,
        candid::Nat::from(999_850u128)
    );

    // test transfering without specifying fee
    let transfer_without_fee = create_transfer(
        &env,
        canister_ids.station,
        user_dto.identities[0],
        station_api::TransferOperationInput {
            from_account_id: account.id.clone(),
            from_asset_id: asset.id.clone(),
            with_standard: "icrc1".to_owned(),
            to: to_address,
            amount: candid::Nat::from(500u128),
            fee: None,
            metadata: vec![],
            network: None,
        },
    );

    // the station queries the ledger canister to get the fee
    assert_eq!(transfer_without_fee.fee, candid::Nat::from(50u64));
}

#[test]
fn make_icrc1_icp_transfer() {
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

    assert_eq!(account.addresses.len(), 2);

    let icp_account_identifier = AccountIdentifier::from_hex(
        &account
            .addresses
            .iter()
            .find(|a| a.format == "icp_account_identifier")
            .expect("cannot get ICP account identifier")
            .address,
    )
    .expect("cannot parse ICP account identifier");

    let icp_icrc1_account = icrc_ledger_types::icrc1::account::Account::from_str(
        &account
            .addresses
            .iter()
            .find(|a| a.format == "icrc1_account")
            .expect("cannot get ICRC1 account")
            .address,
    )
    .expect("invalid account address");

    mint_icp(&env, minter, &icp_account_identifier, 10 * 100_000_000)
        .expect("failed to mint ICP to account");

    mint_icrc1_tokens(
        &env,
        Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(),
        minter,
        icp_icrc1_account,
        20 * 100_000_000,
    )
    .expect("failed to mint ICP to ICRC1 account");

    let account_balances = fetch_account_balances(
        &env,
        canister_ids.station,
        user_dto.identities[0],
        station_api::FetchAccountBalancesInput {
            account_ids: vec![account.id.clone()],
        },
    );
    assert_eq!(account_balances.balances.len(), 1);
    assert_eq!(
        account_balances.balances[0]
            .as_ref()
            .expect("should have balance")
            .balance,
        candid::Nat::from(30 * 100_000_000u64)
    );

    create_transfer(
        &env,
        canister_ids.station,
        user_dto.identities[0],
        station_api::TransferOperationInput {
            from_account_id: account.id.clone(),
            from_asset_id: icp_asset.id.clone(),
            with_standard: "icrc1".to_owned(),
            to: icrc_ledger_types::icrc1::account::Account {
                owner: user_dto.identities[0],
                subaccount: None,
            }
            .to_string(),
            amount: candid::Nat::from(25 * 100_000_000u64),
            fee: None,
            metadata: vec![],
            network: None,
        },
    );

    let account_balances = fetch_account_balances(
        &env,
        canister_ids.station,
        user_dto.identities[0],
        station_api::FetchAccountBalancesInput {
            account_ids: vec![account.id.clone()],
        },
    );
    assert_eq!(account_balances.balances.len(), 1);
    assert_eq!(
        account_balances.balances[0]
            .as_ref()
            .expect("should have balance")
            .balance,
        candid::Nat::from(5 * 100_000_000u64 - 10_000)
    );
}
