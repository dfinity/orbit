use ic_canister_core::utils::timestamp_to_rfc3339;
use std::str::FromStr;
use uuid::Uuid;

use crate::{
    errors::MapperError,
    models::{Operation, OperationCode, OperationStatus},
    transport::{OperationDTO, OperationListItemDTO, OperationStatusDTO},
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
                OperationStatus::Adopted => OperationStatusDTO::Adopted,
                OperationStatus::Rejected => OperationStatusDTO::Rejected,
                OperationStatus::Abstained => OperationStatusDTO::Abstained,
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

    pub fn to_list_item_dto(&self, operation: Operation) -> OperationListItemDTO {
        OperationListItemDTO {
            id: Uuid::from_bytes(operation.id).hyphenated().to_string(),
            account: Uuid::from_bytes(operation.account_id)
                .hyphenated()
                .to_string(),
            status: match operation.status {
                OperationStatus::Pending => OperationStatusDTO::Pending,
                OperationStatus::Adopted => OperationStatusDTO::Adopted,
                OperationStatus::Rejected => OperationStatusDTO::Rejected,
                OperationStatus::Abstained => OperationStatusDTO::Abstained,
            },
            code: operation.code.to_string(),
            created_at: timestamp_to_rfc3339(&operation.created_timestamp),
        }
    }

    pub fn to_status(&self, status: OperationStatusDTO) -> OperationStatus {
        match status {
            OperationStatusDTO::Pending => OperationStatus::Pending,
            OperationStatusDTO::Adopted => OperationStatus::Adopted,
            OperationStatusDTO::Rejected => OperationStatus::Rejected,
            OperationStatusDTO::Abstained => OperationStatus::Abstained,
        }
    }

    pub fn to_code(&self, code: String) -> Result<OperationCode, MapperError> {
        OperationCode::from_str(code.as_str()).map_err(|_| MapperError::UnknownOperationCode {
            code: code.to_owned(),
        })
    }
}
