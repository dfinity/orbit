use crate::setup::{setup_new_env, WALLET_ADMIN_USER};
use crate::utils::user_test_id;
use crate::TestEnv;
use candid::Principal;
use orbit_essentials::utils::timestamp_to_rfc3339;
use pocket_ic::{update_candid_as, PocketIc};
use station_api::{
    AddUserOperationInput, ApiErrorDTO, CreateRequestInput, CreateRequestResponse,
    ListRequestsInput, ListRequestsResponse, RequestExecutionScheduleDTO, RequestOperationInput,
    RequestStatusDTO,
};
use std::time::Duration;

fn assert_list_requests_only_one_active_requests_with_dedup_key(
    env: &PocketIc,
    canister_id: Principal,
    deduplication_key: &str,
) {
    // list the requests, see that there is a request with the same deduplication key
    let res: (Result<ListRequestsResponse, ApiErrorDTO>,) = update_candid_as(
        env,
        canister_id,
        WALLET_ADMIN_USER,
        "list_requests",
        (ListRequestsInput {
            deduplication_keys: Some(vec![deduplication_key.to_string()]),
            requester_ids: None,
            approver_ids: None,
            statuses: None,
            operation_types: None,
            expiration_from_dt: None,
            expiration_to_dt: None,
            created_from_dt: None,
            created_to_dt: None,
            paginate: None,
            sort_by: None,
            only_approvable: false,
            with_evaluation_results: false,
        },),
    )
    .unwrap();
    assert!(res.0.is_ok());
    let requests = res.0.unwrap().requests;
    let active_requests = requests
        .into_iter()
        .filter(|r| {
            matches!(
                r.status,
                RequestStatusDTO::Created
                    | RequestStatusDTO::Approved
                    | RequestStatusDTO::Scheduled { .. }
                    | RequestStatusDTO::Processing { .. }
            )
        })
        .collect::<Vec<_>>();
    assert!(active_requests.len() == 1);
    assert!(active_requests[0].deduplication_key == Some(deduplication_key.to_string()));
}

#[test]
fn test_request_deduplication() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    let user_id = user_test_id(0);

    // add a user through a request
    let add_user = AddUserOperationInput {
        name: "test".to_string(),
        identities: vec![user_id],
        groups: vec![],
        status: station_api::UserStatusDTO::Active,
    };
    let execution_time = env.get_time() + Duration::from_secs(5);
    let execution_time_nanos = timestamp_to_rfc3339(&execution_time.as_nanos_since_unix_epoch());
    let mut add_user_request = CreateRequestInput {
        operation: RequestOperationInput::AddUser(add_user),
        title: None,
        summary: None,
        execution_plan: Some(RequestExecutionScheduleDTO::Scheduled {
            execution_time: execution_time_nanos,
        }),
        expiration_dt: None,
        deduplication_key: Some("test".to_string()),
    };

    let res: (Result<CreateRequestResponse, ApiErrorDTO>,) = update_candid_as(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        "create_request",
        (add_user_request.clone(),),
    )
    .unwrap();
    assert!(res.0.is_ok());
    assert_list_requests_only_one_active_requests_with_dedup_key(
        &env,
        canister_ids.station,
        "test",
    );

    add_user_request.execution_plan = Some(RequestExecutionScheduleDTO::Immediate);

    // submit the same request again
    let res: (Result<CreateRequestResponse, ApiErrorDTO>,) = update_candid_as(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        "create_request",
        (add_user_request.clone(),),
    )
    .unwrap();

    assert!(res.0.is_err());
    assert_list_requests_only_one_active_requests_with_dedup_key(
        &env,
        canister_ids.station,
        "test",
    );

    // wait for the request to be approved and scheduled (timer's period is 5 seconds)
    env.advance_time(Duration::from_secs(5));
    env.tick();

    // submit the same request again
    let res: (Result<CreateRequestResponse, ApiErrorDTO>,) = update_candid_as(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        "create_request",
        (add_user_request.clone(),),
    )
    .unwrap();

    assert!(res.0.is_err());
    assert_list_requests_only_one_active_requests_with_dedup_key(
        &env,
        canister_ids.station,
        "test",
    );

    // wait for the request to be executed (timer's period is 5 seconds)
    env.advance_time(Duration::from_secs(5));
    env.tick();

    // submit the same request again
    let res: (Result<CreateRequestResponse, ApiErrorDTO>,) = update_candid_as(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        "create_request",
        (add_user_request,),
    )
    .unwrap();

    assert!(res.0.is_ok());
    assert_list_requests_only_one_active_requests_with_dedup_key(
        &env,
        canister_ids.station,
        "test",
    );
}
