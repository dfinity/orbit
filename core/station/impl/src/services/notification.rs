use crate::{
    core::{generate_uuid_v4, ic_cdk::next_time, utils::SortDirection, CallContext},
    errors::NotificationError,
    mappers::HelperMapper,
    models::{Notification, NotificationId, NotificationStatus, NotificationType, UserId},
    repositories::{
        NotificationFindByUserWhereClause, NotificationRepository, NotificationSortBy,
        NOTIFICATION_REPOSITORY,
    },
    services::{UserService, USER_SERVICE},
};
use lazy_static::lazy_static;
use orbit_essentials::repository::Repository;
use orbit_essentials::utils::rfc3339_to_timestamp;
use orbit_essentials::{api::ServiceResult, model::ModelValidator};
use station_api::{ListNotificationsInput, MarkNotificationsReadInput};
use std::sync::Arc;
use uuid::Uuid;

lazy_static! {
    pub static ref NOTIFICATION_SERVICE: Arc<NotificationService> =
        Arc::new(NotificationService::new(
            Arc::clone(&USER_SERVICE),
            Arc::clone(&NOTIFICATION_REPOSITORY),
        ));
}

#[derive(Default, Debug)]
pub struct NotificationService {
    user_service: Arc<UserService>,
    notification_repository: Arc<NotificationRepository>,
}

impl NotificationService {
    pub fn new(
        user_service: Arc<UserService>,
        notification_repository: Arc<NotificationRepository>,
    ) -> Self {
        Self {
            user_service,
            notification_repository,
        }
    }

    pub fn get_notification(
        &self,
        id: &NotificationId,
        ctx: &CallContext,
    ) -> ServiceResult<Notification> {
        let notification = self
            .notification_repository
            .get(&Notification::key(*id))
            .ok_or(NotificationError::NotFound {
                id: Uuid::from_bytes(id.to_owned()).hyphenated().to_string(),
            })?;

        self.assert_notification_access(&notification, ctx)?;

        Ok(notification)
    }

    pub fn list_notifications(
        &self,
        input: ListNotificationsInput,
        ctx: &CallContext,
    ) -> ServiceResult<Vec<Notification>> {
        let user = self.user_service.get_user_by_identity(&ctx.caller())?;

        let filter_by_type = input.notification_type.map(|t| t.to_string());

        let notifications = self.notification_repository.find_by_user_where(
            user.id,
            NotificationFindByUserWhereClause {
                created_dt_from: input.from_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
                created_dt_to: input.to_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
                notification_type: filter_by_type,
                status: input.status.map(|status| status.into()),
                sort_by: Some(NotificationSortBy::CreatedDt(SortDirection::Desc)),
            },
        );

        Ok(notifications)
    }

    pub async fn mark_read(
        &self,
        input: MarkNotificationsReadInput,
        ctx: &CallContext,
    ) -> ServiceResult<()> {
        let mut notifications = input
            .notification_ids
            .iter()
            .map(|id| self.get_notification(HelperMapper::to_uuid(id.clone())?.as_bytes(), ctx))
            .collect::<Result<Vec<Notification>, _>>()?;

        for notification in notifications.iter_mut() {
            notification.status = match input.read {
                true => NotificationStatus::Read,
                false => NotificationStatus::Sent,
            };
            notification.last_modification_timestamp = next_time();

            notification.validate()?;
            self.notification_repository
                .insert(notification.to_key(), notification.clone());
        }

        Ok(())
    }

    pub async fn send_notification(
        &self,
        user_id: UserId,
        notification_type: NotificationType,
        title: String,
        message: Option<String>,
    ) {
        let now = next_time();
        let notification_id = generate_uuid_v4().await;
        let notification = Notification {
            id: *notification_id.as_bytes(),
            status: NotificationStatus::Sent,
            target_user_id: user_id,
            title: title
                .chars()
                .take(Notification::MAX_TITLE_LEN as usize)
                .collect(),
            message: message.map(|m| {
                m.chars()
                    .take(Notification::MAX_MESSAGE_LEN as usize)
                    .collect()
            }),
            notification_type,
            created_timestamp: now,
            last_modification_timestamp: now,
        };

        self.notification_repository
            .insert(notification.to_key(), notification);
    }

    fn assert_notification_access(
        &self,
        notification: &Notification,
        ctx: &CallContext,
    ) -> ServiceResult<()> {
        let user = self.user_service.get_user_by_identity(&ctx.caller())?;

        if user.id != notification.target_user_id {
            Err(NotificationError::Forbidden {
                id: Uuid::from_bytes(notification.id.to_owned())
                    .hyphenated()
                    .to_string(),
            })?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::test_utils,
        models::{notification_test_utils::mock_notification, user_test_utils::mock_user, User},
        repositories::UserRepository,
    };
    use candid::Principal;

    struct TestContext {
        repository: NotificationRepository,
        service: NotificationService,
        caller_user: User,
        call_context: CallContext,
    }

    fn setup() -> TestContext {
        test_utils::init_canister_system();

        let call_context = CallContext::new(Principal::from_slice(&[9; 29]));
        let mut user: User = mock_user();
        user.identities = vec![call_context.caller()];

        UserRepository::default().insert(user.to_key(), user.clone());

        TestContext {
            repository: NotificationRepository::default(),
            service: NotificationService::default(),
            call_context,
            caller_user: user,
        }
    }

    #[test]
    fn get_notification() {
        let ctx = setup();
        let mut notification = mock_notification();
        notification.target_user_id = ctx.caller_user.id;

        ctx.repository
            .insert(notification.to_key(), notification.to_owned());

        let result = ctx
            .service
            .get_notification(&notification.id, &ctx.call_context);

        assert_eq!(notification, result.unwrap());
    }

    #[test]
    fn fail_get_notification_not_allowed() {
        let ctx = setup();
        let mut notification = mock_notification();
        notification.target_user_id = [1; 16];

        ctx.repository
            .insert(notification.to_key(), notification.to_owned());

        let result = ctx
            .service
            .get_notification(&notification.id, &ctx.call_context);

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn read_notification_happy_path() {
        let ctx = setup();
        let notification_id = Uuid::new_v4();
        let mut notification = mock_notification();
        notification.id = *notification_id.as_bytes();
        notification.target_user_id = ctx.caller_user.id;
        notification.status = NotificationStatus::Sent;

        ctx.repository
            .insert(notification.to_key(), notification.to_owned());

        let result = ctx
            .service
            .mark_read(
                MarkNotificationsReadInput {
                    notification_ids: vec![notification_id.to_string()],
                    read: true,
                },
                &ctx.call_context,
            )
            .await;

        assert!(result.is_ok());
        assert_eq!(
            ctx.repository.get(&notification.to_key()).unwrap().status,
            NotificationStatus::Read
        );
    }
}
