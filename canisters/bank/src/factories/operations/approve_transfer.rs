use super::OperationProcessor;
use crate::{
    mappers::{HelperMapper, TransferMapper},
    models::{
        Operation, OperationCode, OperationFeedback, OperationStatus, Transfer, TransferStatus,
        OPERATION_METADATA_KEY_TRANSFER_ID,
    },
    repositories::{OperationRepository, TransferRepository},
    transport::OperationContextDTO,
};
use async_trait::async_trait;
use ic_canister_core::api::ApiError;
use ic_canister_core::cdk::api::time;
use ic_canister_core::repository::Repository;

#[derive(Default, Debug)]
pub struct ApproveTransferOperationProcessor {
    helper_mapper: HelperMapper,
    transfer_repository: TransferRepository,
    operation_repository: OperationRepository,
    transfer_mapper: TransferMapper,
}

impl ApproveTransferOperationProcessor {
    fn validate_type(&self, operation: &Operation) -> Result<(), ApiError> {
        if OperationCode::ApproveTransfer != operation.code {
            return Err(ApiError::new("ERR".to_string(), None, None));
        }

        Ok(())
    }

    fn get_transfer(&self, operation: &Operation) -> Result<Transfer, ApiError> {
        let metadata = operation.metadata_map();
        let unparsed_transfer_id = metadata
            .get(OPERATION_METADATA_KEY_TRANSFER_ID)
            .ok_or(ApiError::new("ERR".to_string(), None, None))?;
        let transfer_id = self
            .helper_mapper
            .uuid_from_str(unparsed_transfer_id.to_owned())?;
        let transfer = self
            .transfer_repository
            .get(&Transfer::key(*transfer_id.as_bytes()))
            .ok_or(ApiError::new("ERR".to_string(), None, None))?;

        Ok(transfer)
    }

    fn reevaluate_transfer(&self, operation: &Operation) -> Result<(), ApiError> {
        let mut transfer = self.get_transfer(operation)?;

        let operations = self.operation_repository.find_by_transfer_id(transfer.id);
        let total_approvals = operations
            .iter()
            .filter(|operations| operations.status == OperationStatus::Adopted)
            .count();
        let missing_feedback = operations
            .iter()
            .filter(|operations| operations.status == OperationStatus::Pending)
            .count();

        let can_still_be_approved =
            total_approvals + missing_feedback >= transfer.policy_snapshot.min_approvals as usize;
        let is_approved = total_approvals >= transfer.policy_snapshot.min_approvals as usize;

        if !can_still_be_approved || is_approved {
            transfer.status =
                match total_approvals >= transfer.policy_snapshot.min_approvals as usize {
                    true => TransferStatus::Approved,
                    _ => TransferStatus::Rejected {
                        reason: "Not enough approvals".to_string(),
                    },
                };

            transfer.last_modification_timestamp = time();
            self.transfer_repository
                .insert(transfer.as_key(), transfer.to_owned());

            operations.iter().for_each(|operation| {
                let mut updated_operation = operation.to_owned();
                if operation.status == OperationStatus::Pending {
                    updated_operation.status = OperationStatus::Abstained;
                    updated_operation.last_modification_timestamp = time();
                    updated_operation.feedback = Some(OperationFeedback {
                        created_at: time(),
                        reason: None,
                    });
                    self.operation_repository
                        .insert(updated_operation.as_key(), updated_operation.to_owned());
                }
            });
        }

        Ok(())
    }
}

#[async_trait]
impl OperationProcessor for ApproveTransferOperationProcessor {
    async fn post_process(&self, operation: &Operation) -> Result<(), ApiError> {
        self.validate_type(operation)?;
        self.reevaluate_transfer(operation)?;

        Ok(())
    }

    fn get_context(&self, operation: &Operation) -> Result<OperationContextDTO, ApiError> {
        let transfer = self.get_transfer(operation)?;

        Ok(OperationContextDTO {
            transfer: Some(self.transfer_mapper.transfer_to_dto(transfer)),
        })
    }
}
