use super::{AccountIdDTO, ProposalIdDTO, TimestampRfc3339, UserIdDTO};
use candid::{CandidType, Deserialize};

pub type NotificationIdDTO = String;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum NotificationStatusDTO {
    Sent,
    Read,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum NotificationTypeDTO {
    SystemMessage,
    ProposalCreated(ProposalCreatedNotificationDTO),
    TransferProposalCreated(TransferProposalCreatedNotificationDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ProposalCreatedNotificationDTO {
    pub proposal_id: ProposalIdDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct TransferProposalCreatedNotificationDTO {
    pub proposal_id: ProposalIdDTO,
    pub account_id: AccountIdDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum NotificationTypeInput {
    SystemMessage,
    ProposalCreated,
    TransferProposalCreated,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct NotificationLocalizedTextDTO {
    pub locale_key: String,
    pub body: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct NotificationDTO {
    pub id: NotificationIdDTO,
    pub status: NotificationStatusDTO,
    pub notification_type: NotificationTypeDTO,
    pub target_user_id: UserIdDTO,
    pub title: NotificationLocalizedTextDTO,
    pub message: NotificationLocalizedTextDTO,
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
    pub notification_ids: Vec<NotificationIdDTO>,
    pub read: bool,
}
