use ic_canister_core::utils::timestamp_to_rfc3339;
use uuid::Uuid;

use crate::{
    models::{Operation, OperationStatus},
    transport::{OperationDTO, OperationStatusDTO},
};

#[derive(Default, Clone, Debug)]
pub struct OperationMapper {}

impl OperationMapper {
    pub fn to_operation_dto(&self, operation: Operation) -> OperationDTO {
        OperationDTO {
            id: Uuid::from_bytes(operation.id).hyphenated().to_string(),
            account: Uuid::from_bytes(operation.account_id)
                .hyphenated()
                .to_string(),
            read: operation.read,
            status: match operation.status {
                OperationStatus::Pending => OperationStatusDTO::Pending,
                OperationStatus::Completed => OperationStatusDTO::Completed,
                OperationStatus::Rejected => OperationStatusDTO::Rejected,
            },
            code: operation.code.to_string(),
            created_at: timestamp_to_rfc3339(&operation.created_timestamp),
            feedback_time_at: operation
                .feedback
                .to_owned()
                .map(|feedback| timestamp_to_rfc3339(&feedback.created_at)),
            feedback_reason: match operation.feedback {
                Some(feedback) => feedback.reason,
                None => None,
            },
        }
    }
}
