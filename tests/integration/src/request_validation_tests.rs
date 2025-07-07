use crate::setup::{setup_new_env, WALLET_ADMIN_USER};
use crate::utils::user_test_id;
use crate::TestEnv;
use orbit_essentials::utils::timestamp_to_rfc3339;
use pocket_ic::update_candid_as;
use station_api::{
    AddUserOperationInput, ApiErrorDTO, CreateRequestInput, CreateRequestResponse, GetRequestInput,
    GetRequestResponse, RequestExecutionScheduleDTO, RequestOperationDTO, RequestOperationInput,
    RequestStatusDTO, TimestampRfc3339,
};
use std::time::{Duration, Instant};

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
        execution_plan: Some(RequestExecutionScheduleDTO::Scheduled { execution_time: execution_time_nanos }),
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
    println!("res: {:?}", res.0);

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

    println!("res: {:?}", res.0);
    assert!(res.0.is_err());
    
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

    println!("res: {:?}", res.0);
    assert!(res.0.is_err());

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

}
