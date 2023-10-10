use crate::{
    errors::MapperError,
    models::{Operation, OperationCode},
    transport::{OperationContextDTO, OperationDTO},
};
use ic_canister_core::utils::timestamp_to_rfc3339;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Default, Clone, Debug)]
pub struct OperationMapper {}

impl OperationMapper {
    pub fn to_operation_dto(
        &self,
        operation: Operation,
        context: OperationContextDTO,
    ) -> OperationDTO {
        OperationDTO {
            id: Uuid::from_bytes(operation.id).hyphenated().to_string(),
            originator_account_id: operation
                .originator_account_id
                .map(|id| Uuid::from_bytes(id).hyphenated().to_string()),
            status: operation.status.into(),
            metadata: operation.metadata,
            code: operation.code.to_string(),
            created_at: timestamp_to_rfc3339(&operation.created_timestamp),
            decisions: operation
                .decisions
                .iter()
                .map(|decision| decision.to_owned().into())
                .collect(),
            context,
        }
    }

    pub fn to_code(&self, code: String) -> Result<OperationCode, MapperError> {
        OperationCode::from_str(code.as_str()).map_err(|_| MapperError::UnknownOperationCode {
            code: code.to_owned(),
        })
    }
}
