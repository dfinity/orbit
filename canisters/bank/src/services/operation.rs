use super::{AccountService, WalletService};
use crate::{
    core::{CallContext, WithCallContext},
    errors::OperationError,
    factories::operations::OperationProcessorFactory,
    mappers::{HelperMapper, OperationMapper},
    models::{Account, Operation, OperationFeedback, OperationId, OperationStatus},
    repositories::{OperationRepository, OperationWhereClause},
    transport::{
        EditOperationInput, GetOperationInput, GetWalletInput, ListOperationsInput,
        ListWalletOperationsInput, OperationDTO,
    },
};
use ic_canister_core::api::ServiceResult;
use ic_canister_core::{api::ApiError, repository::Repository};
use ic_canister_core::{cdk::api::time, utils::rfc3339_to_timestamp};
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct OperationService {
    call_context: CallContext,
    account_service: AccountService,
    wallet_service: WalletService,
    operation_repository: OperationRepository,
    operation_mapper: OperationMapper,
    helper_mapper: HelperMapper,
}

impl WithCallContext for OperationService {
    fn with_call_context(&mut self, call_context: CallContext) -> &Self {
        self.call_context = call_context.to_owned();
        self.account_service
            .with_call_context(call_context.to_owned());
        self.wallet_service
            .with_call_context(call_context.to_owned());

        self
    }
}

impl OperationService {
    pub fn create() -> Self {
        Default::default()
    }

    pub async fn get_operation_core(&self, id: OperationId) -> ServiceResult<Operation> {
        let operation = self.operation_repository.get(&Operation::key(id)).ok_or(
            OperationError::OperationNotFound {
                operation_id: Uuid::from_bytes(id.to_owned()).hyphenated().to_string(),
            },
        )?;
        let caller_account = self
            .account_service
            .resolve_account(&self.call_context.caller())?;

        self.check_access_to_operation(&operation, &caller_account)?;

        Ok(operation)
    }

    pub fn check_access_to_operation(
        &self,
        operation: &Operation,
        caller_account: &Account,
    ) -> ServiceResult<()> {
        if operation.account_id != caller_account.id {
            Err(OperationError::Forbidden {
                operation_id: Uuid::from_bytes(operation.id.to_owned())
                    .hyphenated()
                    .to_string(),
            })?
        }

        Ok(())
    }

    pub async fn get_operation(&self, input: GetOperationInput) -> ServiceResult<OperationDTO> {
        let operation_id = self.helper_mapper.uuid_from_str(input.operation_id)?;
        let operation = self
            .get_operation_core(operation_id.as_bytes().to_owned())
            .await?;

        let processor = OperationProcessorFactory::build(&operation.code);
        let context = processor.get_context(&operation)?;
        let operation_dto = self.operation_mapper.to_operation_dto(operation, context);

        Ok(operation_dto)
    }

    pub async fn edit_operation(&self, input: EditOperationInput) -> ServiceResult<OperationDTO> {
        let operation_id = self.helper_mapper.uuid_from_str(input.operation_id)?;
        let mut operation = self
            .get_operation_core(operation_id.as_bytes().to_owned())
            .await?;

        if let (Some(_), Some(_)) = (input.approve.as_ref(), operation.feedback.as_ref()) {
            Err(OperationError::NotAllowedModification {
                operation_id: Uuid::from_bytes(operation.id.to_owned())
                    .hyphenated()
                    .to_string(),
            })?
        }

        if let Some(approve) = input.approve {
            operation.status = match approve {
                true => OperationStatus::Adopted,
                false => OperationStatus::Rejected,
            };
            operation.read = true;
            operation.feedback = Some(OperationFeedback {
                created_at: time(),
                reason: input.reason,
            });
        }

        if let Some(read) = input.read {
            operation.read = read;
        }

        self.operation_repository
            .insert(operation.as_key(), operation.to_owned());

        let processor = OperationProcessorFactory::build(&operation.code);

        processor
            .post_process(&operation)
            .await
            .expect("Operation post processing failed");

        let context = processor.get_context(&operation)?;
        let operation_dto = self.operation_mapper.to_operation_dto(operation, context);

        Ok(operation_dto)
    }

    pub async fn list_operations(
        &self,
        input: ListOperationsInput,
    ) -> ServiceResult<Vec<OperationDTO>> {
        let account = self
            .account_service
            .resolve_account(&self.call_context.caller())?;

        let filter_by_code = match input.code {
            Some(code) => Some(self.operation_mapper.to_code(code)?),
            None => None,
        };
        let dtos = self
            .operation_repository
            .find_by_account_where(
                account.id,
                OperationWhereClause {
                    created_dt_from: input.from_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
                    created_dt_to: input.to_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
                    code: filter_by_code,
                    status: input
                        .status
                        .map(|status| self.operation_mapper.to_status(status)),
                    read: input.read,
                },
            )
            .iter()
            .map(|operation| {
                let processor = OperationProcessorFactory::build(&operation.code);
                let context = processor.get_context(operation)?;

                Ok(self
                    .operation_mapper
                    .to_operation_dto(operation.to_owned(), context))
            })
            .collect::<Result<Vec<OperationDTO>, ApiError>>()?;

        Ok(dtos)
    }

    pub async fn list_wallet_operations(
        &self,
        input: ListWalletOperationsInput,
    ) -> ServiceResult<Vec<OperationDTO>> {
        let account = self
            .account_service
            .resolve_account(&self.call_context.caller())?;
        let wallet = self
            .wallet_service
            .get_wallet_core(GetWalletInput {
                wallet_id: input.wallet_id,
            })
            .await?;

        let filter_by_code = match input.code {
            Some(code) => Some(self.operation_mapper.to_code(code)?),
            None => None,
        };
        let dtos = self
            .operation_repository
            .find_by_wallet_where(
                (account.id, wallet.id),
                OperationWhereClause {
                    created_dt_from: input.from_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
                    created_dt_to: input.to_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
                    code: filter_by_code,
                    status: input
                        .status
                        .map(|status| self.operation_mapper.to_status(status)),
                    read: input.read,
                },
            )
            .iter()
            .map(|operation| {
                let processor = OperationProcessorFactory::build(&operation.code);
                let context = processor.get_context(operation)?;

                Ok(self
                    .operation_mapper
                    .to_operation_dto(operation.to_owned(), context))
            })
            .collect::<Result<Vec<OperationDTO>, ApiError>>()?;

        Ok(dtos)
    }
}
