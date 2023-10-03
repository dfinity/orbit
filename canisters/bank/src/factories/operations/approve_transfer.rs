use super::OperationProcessor;
use crate::{
    mappers::HelperMapper,
    models::{
        Operation, OperationCode, OperationFeedback, OperationStatus,
        OperationTransferIndexCriteria, Transfer, TransferStatus,
        OPERATION_METADATA_KEY_TRANSFER_ID,
    },
    repositories::{OperationRepository, OperationTransferIndexRepository, TransferRepository},
};
use async_trait::async_trait;
use ic_canister_core::cdk::api::time;
use ic_canister_core::repository::Repository;
use ic_canister_core::{api::ApiError, repository::IndexRepository};

#[derive(Default, Debug)]
pub struct ApproveTransferOperationProcessor {
    helper_mapper: HelperMapper,
    transfer_repository: TransferRepository,
    operation_repository: OperationRepository,
    operation_transfer_index: OperationTransferIndexRepository,
}

impl ApproveTransferOperationProcessor {
    fn validate_type(&self, operation: &Operation) -> Result<(), ApiError> {
        if OperationCode::ApproveTransfer != operation.code {
            return Err(ApiError::new("ERR".to_string(), None, None));
        }

        Ok(())
    }

    fn reevaluate_transfer(&self, operation: &Operation) -> Result<(), ApiError> {
        let metadata = operation.metadata_map();
        let unparsed_transfer_id = metadata
            .get(OPERATION_METADATA_KEY_TRANSFER_ID)
            .ok_or(ApiError::new("ERR".to_string(), None, None))?;
        let transfer_id = self
            .helper_mapper
            .uuid_from_str(unparsed_transfer_id.to_owned())?;
        let mut transfer = self
            .transfer_repository
            .get(&Transfer::key(*transfer_id.as_bytes()))
            .ok_or(ApiError::new("ERR".to_string(), None, None))?;

        let operations =
            self.operation_transfer_index
                .find_by_criteria(OperationTransferIndexCriteria {
                    transfer_id: transfer.id,
                    code: None,
                    read: None,
                    status: None,
                });

        let approvals = operations
            .iter()
            .filter(|operations| operations.status == OperationStatus::Adopted)
            .count();

        if approvals >= transfer.policy_snapshot.min_approvals as usize {
            transfer.status = TransferStatus::Approved;
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
        let processor = ApproveTransferOperationProcessor::default();
        processor.validate_type(operation)?;
        processor.reevaluate_transfer(operation)?;

        Ok(())
    }
}
