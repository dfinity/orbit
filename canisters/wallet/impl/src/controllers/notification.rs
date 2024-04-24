use crate::{
    core::middlewares::{call_context, use_status_metric}, mappers::notification::NotificationMapperError,
    services::NotificationService,
};
use ic_canister_core::{api::ApiResult, cdk::api::print};
use ic_canister_macros::with_middleware;
use ic_cdk_macros::{query, update};
use lazy_static::lazy_static;
use uuid::Uuid;
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

    /// No authorization required since the user will be calling this only for their own notifications.
    async fn list_notifications(
        &self,
        input: ListNotificationsInput,
    ) -> ApiResult<ListNotificationsResponse> {
        let notifications = self
            .notification_service
            .list_notifications(input, &call_context())?
            .into_iter()
            .fold(Vec::new(), |mut acc, notification| {
                match NotificationDTO::try_from(notification) {
                    Ok(notification_dto) => acc.push(notification_dto),
                    Err(error) => match error {
                        NotificationMapperError::ProposalNotFound { proposal_id } => {
                            print(format!(
                                "Proposal {} not found when mapping to NotificationDTO",
                                Uuid::from_bytes(proposal_id).hyphenated()
                            ));
                        }
                    },
                }

                acc
            });

        Ok(ListNotificationsResponse { notifications })
    }

    /// No authorization required since the user will be calling this only for their own notifications.
    #[with_middleware(tail = use_status_metric("call_mark_notifications_read", &result))]
    async fn mark_notifications_read(&self, input: MarkNotificationsReadInput) -> ApiResult<()> {
        self.notification_service
            .mark_read(input, &call_context())
            .await?;

        Ok(())
    }
}
