use super::OperationProcessor;
use crate::core::ic_cdk::api::time;
use crate::{
    errors::AccountError,
    mappers::HelperMapper,
    models::{
        Account, Operation, OperationCode, OperationContext, OperationStatus, Transfer,
        TransferStatus, OPERATION_METADATA_KEY_TRANSFER_ID,
    },
    repositories::{AccountRepository, OperationRepository, TransferRepository},
};
use async_trait::async_trait;
use ic_canister_core::api::ApiError;
use ic_canister_core::repository::Repository;
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct ApproveTransferOperationProcessor {
    transfer_repository: TransferRepository,
    operation_repository: OperationRepository,
    account_repository: AccountRepository,
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
        let transfer_id = HelperMapper::to_uuid(unparsed_transfer_id.to_owned())?;
        let transfer = self
            .transfer_repository
            .get(&Transfer::key(*transfer_id.as_bytes()))
            .ok_or(ApiError::new("ERR".to_string(), None, None))?;

        Ok(transfer)
    }

    fn get_account(&self, operation: &Operation) -> Result<Account, ApiError> {
        let transfer = self.get_transfer(operation)?;
        let account = self
            .account_repository
            .get(&Account::key(transfer.from_account));

        if let Some(account) = account {
            return Ok(account);
        }

        Err(AccountError::AccountNotFound {
            id: Uuid::from_bytes(transfer.from_account)
                .hyphenated()
                .to_string(),
        })?
    }

    fn reevaluate_transfer(&self, operation: &Operation) -> Result<(), ApiError> {
        let mut transfer = self.get_transfer(operation)?;

        let total_approvals = operation
            .decisions
            .iter()
            .filter(|operations| operations.status == OperationStatus::Adopted)
            .count();
        let missing_feedback = operation
            .decisions
            .iter()
            .filter(|operations| operations.status == OperationStatus::Pending)
            .count();

        let is_approved = total_approvals >= transfer.policy_snapshot.min_approvals as usize;
        let can_still_be_approved =
            total_approvals + missing_feedback >= transfer.policy_snapshot.min_approvals as usize;

        if !can_still_be_approved || is_approved {
            transfer.status = match is_approved {
                true => TransferStatus::Approved,
                _ => TransferStatus::Rejected {
                    reason: "Not enough approvals".to_string(),
                },
            };

            transfer.last_modification_timestamp = time();
            self.transfer_repository
                .insert(transfer.to_key(), transfer.to_owned());

            let mut updated_operation = operation.to_owned();
            updated_operation.status = match is_approved {
                true => OperationStatus::Adopted,
                _ => OperationStatus::Rejected,
            };

            updated_operation.decisions.iter_mut().for_each(|decision| {
                if decision.status == OperationStatus::Pending {
                    decision.status = OperationStatus::NotRequired;
                    decision.last_modification_timestamp = time();
                    decision.decided_dt = Some(time());
                }
            });
            self.operation_repository
                .insert(updated_operation.to_key(), updated_operation.to_owned());
        }

        Ok(())
    }
}

#[async_trait]
impl OperationProcessor for ApproveTransferOperationProcessor {
    async fn post_process(&self, operation: &Operation) -> Result<Operation, ApiError> {
        self.validate_type(operation)?;
        self.reevaluate_transfer(operation)?;

        Ok(self
            .operation_repository
            .get(&Operation::key(operation.id))
            .unwrap_or(operation.clone()))
    }

    fn get_context(&self, operation: &Operation) -> Result<OperationContext, ApiError> {
        let transfer = self.get_transfer(operation)?;
        let account = self.get_account(operation)?;

        Ok(OperationContext {
            transfer: Some(transfer),
            account: Some(account),
        })
    }
}
