use crate::interfaces::{
    default_account, get_icp_balance, send_icp, send_icp_to_account, ICP, ICP_FEE,
};
use crate::setup::{setup_new_env, WALLET_ADMIN_USER};
use crate::utils::user_test_id;
use crate::TestEnv;
use ic_ledger_types::AccountIdentifier;
use orbit_essentials::api::ApiResult;
use pocket_ic::{query_candid_as, update_candid_as};
use station_api::{
    AddAccountOperationInput, AllowDTO, ApiErrorDTO, CreateRequestInput, CreateRequestResponse,
    GetRequestInput, GetRequestResponse, GetTransfersInput, GetTransfersResponse,
    ListAccountTransfersInput, ListAccountTransfersResponse, MeResponse, QuorumPercentageDTO,
    RequestExecutionScheduleDTO, RequestOperationDTO, RequestOperationInput, RequestPolicyRuleDTO,
    RequestStatusDTO, TransferOperationInput, UserSpecifierDTO,
};
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

    // create account
    let create_account_args = AddAccountOperationInput {
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
        RequestStatusDTO::Approved { .. } => {}
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
    let account_address = AccountIdentifier::from_hex(&account_dto.address).unwrap();
    send_icp_to_account(
        &env,
        WALLET_ADMIN_USER,
        account_address,
        ICP + ICP_FEE,
        0,
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
