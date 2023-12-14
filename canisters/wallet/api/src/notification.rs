use super::TimestampRfc3339;
use crate::UuidDTO;
use candid::{CandidType, Deserialize};
use std::fmt::{Display, Formatter};

pub const SYSTEM_MESSAGE_NOTIFICATION_TYPE: &str = "system-message";
pub const PROPOSAL_CREATED_NOTIFICATION_TYPE: &str = "proposal-created";
pub const TRANSFER_PROPOSAL_CREATED_NOTIFICATION_TYPE: &str = "transfer-proposal-created";
pub const ACCOUNT_PROPOSAL_CREATED_NOTIFICATION_TYPE: &str = "account-proposal-created";

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
    AccountProposalCreated(AccountProposalCreatedNotificationDTO),
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct ProposalCreatedNotificationDTO {
    pub proposal_id: UuidDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AccountProposalCreatedNotificationDTO {
    pub proposal_id: UuidDTO,
    pub account_id: UuidDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct TransferProposalCreatedNotificationDTO {
    pub proposal_id: UuidDTO,
    pub account_id: UuidDTO,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum NotificationTypeInput {
    SystemMessage,
    ProposalCreated,
    TransferProposalCreated,
    AccountProposalCreated,
}

impl Display for NotificationTypeInput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationTypeInput::SystemMessage => {
                write!(f, "{}", SYSTEM_MESSAGE_NOTIFICATION_TYPE)
            }
            NotificationTypeInput::TransferProposalCreated => {
                write!(f, "{}", TRANSFER_PROPOSAL_CREATED_NOTIFICATION_TYPE)
            }
            NotificationTypeInput::ProposalCreated => {
                write!(f, "{}", PROPOSAL_CREATED_NOTIFICATION_TYPE)
            }
            NotificationTypeInput::AccountProposalCreated => {
                write!(f, "{}", ACCOUNT_PROPOSAL_CREATED_NOTIFICATION_TYPE)
            }
        }
    }
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct NotificationLocalizedTextDTO {
    pub locale_key: String,
    pub body: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct NotificationDTO {
    pub id: UuidDTO,
    pub status: NotificationStatusDTO,
    pub notification_type: NotificationTypeDTO,
    pub target_user_id: UuidDTO,
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
    pub notification_ids: Vec<UuidDTO>,
    pub read: bool,
}
