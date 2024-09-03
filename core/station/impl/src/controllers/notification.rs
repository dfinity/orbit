use crate::{
    core::middlewares::{authorize, call_context, use_canister_call_metric},
    mappers::authorization::MarkNotificationsReadInputRef,
    mappers::notification::NotificationMapperError,
    models::resource::Resource,
    services::NotificationService,
};
use ic_cdk_macros::{query, update};
use lazy_static::lazy_static;
use orbit_essentials::with_middleware;
use orbit_essentials::{api::ApiResult, cdk::api::print};
use station_api::{
    ListNotificationsInput, ListNotificationsResponse, MarkNotificationsReadInput, NotificationDTO,
};
use uuid::Uuid;

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

    #[with_middleware(guard = authorize(&call_context(), &[Resource::from(&input)]))]
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
                        NotificationMapperError::RequestNotFound { request_id } => {
                            print(format!(
                                "Request {} not found when mapping to NotificationDTO",
                                Uuid::from_bytes(request_id).hyphenated()
                            ));
                        }
                        NotificationMapperError::InvalidRequestStatus {
                            expected, found
                        } => {
                            print(format!("Invalid request status when mapping to NotificationDTO: expected \"{}\", found \"{}\"", expected, found));
                        }
                    },
                }

                acc
            });

        Ok(ListNotificationsResponse { notifications })
    }

    #[with_middleware(guard = authorize(&call_context(), &MarkNotificationsReadInputRef(&input).to_resources()))]
    #[with_middleware(tail = use_canister_call_metric("mark_notifications_read", &result))]
    async fn mark_notifications_read(&self, input: MarkNotificationsReadInput) -> ApiResult<()> {
        self.notification_service.mark_read(input).await?;

        Ok(())
    }
}
