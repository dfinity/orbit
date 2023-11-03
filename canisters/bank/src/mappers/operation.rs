use crate::{
    errors::MapperError,
    models::{Operation, OperationCode, OperationContext},
    transport::{OperationContextDTO, OperationDTO},
};
use ic_canister_core::utils::timestamp_to_rfc3339;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Default, Clone, Debug)]
pub struct OperationMapper {}

impl OperationMapper {
    pub fn to_dto(operation: Operation, context: OperationContext) -> OperationDTO {
        OperationDTO {
            id: Uuid::from_bytes(operation.id).hyphenated().to_string(),
            proposed_by: operation
                .proposed_by
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
            context: OperationContextDTO {
                transfer: context.transfer.map(|transfer| transfer.to_dto()),
                wallet: context.wallet.map(|wallet| wallet.to_dto()),
            },
        }
    }

    pub fn to_code(code: String) -> Result<OperationCode, MapperError> {
        OperationCode::from_str(code.as_str()).map_err(|_| MapperError::UnknownOperationCode {
            code: code.to_owned(),
        })
    }
}

impl Operation {
    pub fn to_dto(&self, context: OperationContext) -> OperationDTO {
        OperationMapper::to_dto(self.clone(), context)
    }
}
