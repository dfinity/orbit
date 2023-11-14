use crate::{
    core::{
        CallContext, WithCallContext, PERMISSION_READ_NOTIFICATION, PERMISSION_WRITE_NOTIFICATION,
    },
    services::NotificationService,
    transport::{
        ListNotificationsInput, ListNotificationsResponse, MarkNotificationsReadInput,
        NotificationDTO,
    },
};
use ic_canister_core::api::{ApiError, ApiResult};
use ic_cdk_macros::{query, update};

#[query(name = "list_notifications")]
async fn list_notifications(input: ListNotificationsInput) -> ApiResult<ListNotificationsResponse> {
    CallContext::get().check_access(PERMISSION_READ_NOTIFICATION);
    let service = NotificationService::with_call_context(CallContext::get());

    let notifications = service.list_notifications(input)?.into_iter().try_fold(
        Vec::new(),
        |mut acc, notification| {
            acc.push(NotificationDTO::from(notification));
            Ok::<Vec<_>, ApiError>(acc)
        },
    )?;

    Ok(ListNotificationsResponse { notifications })
}

#[update(name = "mark_notifications_read")]
async fn mark_notifications_read(input: MarkNotificationsReadInput) -> ApiResult<()> {
    CallContext::get().check_access(PERMISSION_WRITE_NOTIFICATION);
    let service = NotificationService::with_call_context(CallContext::get());

    service.mark_read(input).await?;

    Ok(())
}
