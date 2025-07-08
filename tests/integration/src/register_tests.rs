use crate::setup::{setup_new_env, WALLET_ADMIN_USER};
use crate::utils::user_test_id;
use crate::TestEnv;
use pocket_ic::update_candid_as;
use station_api::{
    AddUserOperationInput, ApiErrorDTO, CreateRequestInput, CreateRequestResponse, GetRequestInput,
    GetRequestResponse, RequestExecutionScheduleDTO, RequestOperationDTO, RequestOperationInput,
    RequestStatusDTO,
};
use std::time::Duration;

#[test]
fn register_user_successful() {
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
    let add_user_request = CreateRequestInput {
        operation: RequestOperationInput::AddUser(add_user),
        title: None,
        summary: None,
        execution_plan: Some(RequestExecutionScheduleDTO::Immediate),
        expiration_dt: None,
        tags: None,
    };

    let res: (Result<CreateRequestResponse, ApiErrorDTO>,) = update_candid_as(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        "create_request",
        (add_user_request,),
    )
    .unwrap();
    let request_dto = res.0.unwrap().request;

    // wait for the request to be approved and scheduled (timer's period is 5 seconds)
    env.advance_time(Duration::from_secs(5));
    env.tick();
    // wait for the request to be executed (timer's period is 5 seconds)
    env.advance_time(Duration::from_secs(5));
    env.tick();

    // check transfer request status
    let get_request_args = GetRequestInput {
        request_id: request_dto.id,
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

    if let RequestOperationDTO::AddUser(add_user) = new_request_dto.operation {
        assert_eq!(add_user.user.unwrap().name, "test".to_string());
    } else {
        panic!("request operation must be AddUser");
    }
}
