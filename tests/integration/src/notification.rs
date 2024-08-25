use crate::setup::{setup_new_env, WALLET_ADMIN_USER};
use crate::utils::{
    execute_request, execute_request_with_extra_ticks, get_request, submit_request,
    submit_request_approval, user_test_id,
};
use crate::TestEnv;
use orbit_essentials::api::ApiResult;
use pocket_ic::{update_candid_as, CallError};
use station_api::{
    AddUserOperationInput, ChangeCanisterOperationInput, ChangeCanisterResourceActionDTO,
    ChangeCanisterTargetDTO, EditPermissionOperationInput, ListNotificationsInput,
    ListNotificationsResponse, MarkNotificationsReadInput, MeResponse, RequestApprovalStatusDTO,
    RequestOperationInput, RequestStatusDTO, ResourceDTO,
};

#[test]
fn notification_authorization() {
    let TestEnv {
        env, canister_ids, ..
    } = setup_new_env();

    // admin makes a failed request which triggers a notification
    let change_canister_operation_input = ChangeCanisterOperationInput {
        target: ChangeCanisterTargetDTO::UpgradeUpgrader,
        module: vec![],
        arg: None,
    };
    let request_status = execute_request_with_extra_ticks(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        RequestOperationInput::ChangeCanister(change_canister_operation_input.clone()),
        10,
    )
    .unwrap_err()
    .unwrap();
    match request_status {
        RequestStatusDTO::Failed { .. } => (),
        _ => panic!("Unexpected request status: {:?}", request_status),
    };

    // admin user can list and update notifications
    let list_notifications_input = ListNotificationsInput {
        status: None,
        notification_type: None,
        from_dt: None,
        to_dt: None,
    };
    let res: (ApiResult<ListNotificationsResponse>,) = update_candid_as(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        "list_notifications",
        (list_notifications_input.clone(),),
    )
    .unwrap();
    let notifications = res.0.unwrap().notifications;
    assert_eq!(notifications.len(), 1);
    let admin_notification = &notifications[0];
    let mark_notifications_read_input = MarkNotificationsReadInput {
        notification_ids: vec![admin_notification.id.clone()],
        read: true,
    };
    let res: (ApiResult<()>,) = update_candid_as(
        &env,
        canister_ids.station,
        WALLET_ADMIN_USER,
        "mark_notifications_read",
        (mark_notifications_read_input.clone(),),
    )
    .unwrap();
    res.0.unwrap();

    // principal without a user cannot list notifications
    let user_id = user_test_id(0);
    let err = update_candid_as::<_, (ApiResult<MeResponse>,)>(
        &env,
        canister_ids.station,
        user_id,
        "list_notifications",
        (list_notifications_input.clone(),),
    )
    .unwrap_err();
    match err {
        CallError::UserError(err) => assert!(err
            .description
            .contains("Unauthorized access to resources: Notification(List)")),
        CallError::Reject(msg) => panic!("Unexpected reject: {}", msg),
    };
    // and cannot mark foreign notifications read
    let err = update_candid_as::<_, (ApiResult<()>,)>(
        &env,
        canister_ids.station,
        user_id,
        "mark_notifications_read",
        (mark_notifications_read_input.clone(),),
    )
    .unwrap_err();
    match err {
        CallError::UserError(err) => assert!(err.description.contains(&format!(
            "Unauthorized access to resources: Notification(Update(Id({})))",
            admin_notification.id
        ))),
        CallError::Reject(msg) => panic!("Unexpected reject: {}", msg),
    };

    // register new non-admin user
    let add_user = AddUserOperationInput {
        name: "test".to_string(),
        identities: vec![user_id],
        groups: vec![],
        status: station_api::UserStatusDTO::Active,
    };
    execute_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        RequestOperationInput::AddUser(add_user),
    )
    .unwrap();

    // allow anyone to create change canister requests
    let edit_permission_operation_input = EditPermissionOperationInput {
        resource: ResourceDTO::ChangeCanister(ChangeCanisterResourceActionDTO::Create),
        auth_scope: Some(station_api::AuthScopeDTO::Authenticated),
        user_groups: None,
        users: None,
    };
    execute_request(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        RequestOperationInput::EditPermission(edit_permission_operation_input),
    )
    .unwrap();

    // now the request to upgrade the upgrader canister can be successfully submitted
    let change_canister_operation_request = submit_request(
        &env,
        user_id,
        canister_ids.station,
        RequestOperationInput::ChangeCanister(change_canister_operation_input),
    );

    // let the admin user reject the request => the request becomes rejected which triggers a notification
    submit_request_approval(
        &env,
        WALLET_ADMIN_USER,
        canister_ids.station,
        change_canister_operation_request.clone(),
        RequestApprovalStatusDTO::Rejected,
    );
    let rejected_request = get_request(
        &env,
        user_id,
        canister_ids.station,
        change_canister_operation_request,
    );
    match rejected_request.status {
        RequestStatusDTO::Rejected { .. } => (),
        _ => panic!("Request should have been rejected."),
    };

    // list notifications as non-admin user (every authenticated user can list notifications)
    let res: (ApiResult<ListNotificationsResponse>,) = update_candid_as(
        &env,
        canister_ids.station,
        user_id,
        "list_notifications",
        (list_notifications_input.clone(),),
    )
    .unwrap();
    let notifications = res.0.unwrap().notifications;
    assert_eq!(notifications.len(), 1);
    let notification = &notifications[0];

    // mark notification read as non-admin user
    let mark_notifications_read_input = MarkNotificationsReadInput {
        notification_ids: vec![notification.id.clone()],
        read: true,
    };
    let res: (ApiResult<()>,) = update_candid_as(
        &env,
        canister_ids.station,
        user_id,
        "mark_notifications_read",
        (mark_notifications_read_input.clone(),),
    )
    .unwrap();
    res.0.unwrap();

    // mark foreign notification read as non-admin user
    let mark_notifications_read_input = MarkNotificationsReadInput {
        notification_ids: vec![admin_notification.id.clone()],
        read: true,
    };
    let err = update_candid_as::<_, (ApiResult<()>,)>(
        &env,
        canister_ids.station,
        user_id,
        "mark_notifications_read",
        (mark_notifications_read_input.clone(),),
    )
    .unwrap_err();
    match err {
        CallError::UserError(err) => assert!(err.description.contains(&format!(
            "Unauthorized access to resources: Notification(Update(Id({})))",
            admin_notification.id
        ))),
        CallError::Reject(msg) => panic!("Unexpected reject: {}", msg),
    };
}
