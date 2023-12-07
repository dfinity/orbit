use crate::core::middlewares::ResourceAccess;
use crate::models::access_control::{AccessModifier, Resource};
use crate::{
    core::middlewares::{authorize, call_context},
    services::NotificationService,
};
use ic_canister_core::api::{ApiError, ApiResult};
use ic_canister_macros::with_middleware;
use ic_cdk_macros::{query, update};
use lazy_static::lazy_static;
use wallet_api::{
    ListNotificationsInput, ListNotificationsResponse, MarkNotificationsReadInput, NotificationDTO,
};

// Canister entrypoints for the controller.
#[query(name = "list_notifications")]
async fn list_notifications(input: ListNotificationsInput) -> ApiResult<ListNotificationsResponse> {
    CONTROLLER.list_notifications(input).await
}

#[update(name = "mark_notifications_read")]
async fn mark_notifications_read(input: MarkNotificationsReadInput) -> ApiResult<()> {
    CONTROLLER.mark_notifications_read(input).await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: NotificationController =
        NotificationController::new(NotificationService::default());
}

#[derive(Debug)]
pub struct NotificationController {
    notification_service: NotificationService,
}

impl NotificationController {
    fn new(notification_service: NotificationService) -> Self {
        Self {
            notification_service,
        }
    }

    #[with_middleware(
        guard = "authorize",
        context = "call_context",
        args = [
            ResourceAccess(Resource::User, AccessModifier::Default)
        ],
        is_async = true
    )]
    async fn list_notifications(
        &self,
        input: ListNotificationsInput,
    ) -> ApiResult<ListNotificationsResponse> {
        let notifications = self
            .notification_service
            .list_notifications(input, &call_context())?
            .into_iter()
            .try_fold(Vec::new(), |mut acc, notification| {
                acc.push(NotificationDTO::from(notification));
                Ok::<Vec<_>, ApiError>(acc)
            })?;

        Ok(ListNotificationsResponse { notifications })
    }

    #[with_middleware(
        guard = "authorize",
        context = "call_context",
        args = [
            ResourceAccess(Resource::User, AccessModifier::Default)
        ],
        is_async = true
    )]
    async fn mark_notifications_read(&self, input: MarkNotificationsReadInput) -> ApiResult<()> {
        self.notification_service
            .mark_read(input, &call_context())
            .await?;

        Ok(())
    }
}
