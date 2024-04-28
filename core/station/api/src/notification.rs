use super::TimestampRfc3339;
use crate::{RequestOperationTypeDTO, UuidDTO};
use candid::{CandidType, Deserialize};
use std::fmt::{Display, Formatter};

pub const SYSTEM_MESSAGE_NOTIFICATION_TYPE: &str = "system-message";
pub const REQUEST_CREATED_NOTIFICATION_TYPE: &str = "request-created";

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum NotificationStatusDTO {
    Sent,
    Read,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum NotificationTypeDTO {
    SystemMessage,
    RequestCreated(RequestCreatedNotificationDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct RequestCreatedNotificationDTO {
    pub request_id: UuidDTO,
    pub operation_type: RequestOperationTypeDTO,
    pub account_id: Option<UuidDTO>,
    pub user_id: Option<UuidDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum NotificationTypeInput {
    SystemMessage,
    RequestCreated,
}

impl Display for NotificationTypeInput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationTypeInput::SystemMessage => {
                write!(f, "{}", SYSTEM_MESSAGE_NOTIFICATION_TYPE)
            }
            NotificationTypeInput::RequestCreated => {
                write!(f, "{}", REQUEST_CREATED_NOTIFICATION_TYPE)
            }
        }
    }
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct NotificationDTO {
    pub id: UuidDTO,
    pub status: NotificationStatusDTO,
    pub notification_type: NotificationTypeDTO,
    pub target_user_id: UuidDTO,
    pub title: String,
    pub message: Option<String>,
    pub created_at: TimestampRfc3339,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListNotificationsInput {
    pub status: Option<NotificationStatusDTO>,
    pub notification_type: Option<NotificationTypeInput>,
    pub from_dt: Option<TimestampRfc3339>,
    pub to_dt: Option<TimestampRfc3339>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ListNotificationsResponse {
    pub notifications: Vec<NotificationDTO>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct MarkNotificationsReadInput {
    pub notification_ids: Vec<UuidDTO>,
    pub read: bool,
}
