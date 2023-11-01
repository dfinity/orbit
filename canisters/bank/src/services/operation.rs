use super::{AccountService, WalletService};
use crate::{
    core::{ic_cdk::api::time, CallContext, WithCallContext},
    errors::OperationError,
    factories::operations::OperationProcessorFactory,
    mappers::{HelperMapper, OperationMapper},
    models::{Operation, OperationContext, OperationId, OperationStatus},
    repositories::{OperationFindByAccountWhereClause, OperationRepository, OperationWhereClause},
    transport::{EditOperationInput, ListOperationsInput, ListWalletOperationsInput},
};
use ic_canister_core::repository::Repository;
use ic_canister_core::utils::rfc3339_to_timestamp;
use ic_canister_core::{api::ServiceResult, model::ModelValidator};
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct OperationService {
    call_context: CallContext,
    account_service: AccountService,
    wallet_service: WalletService,
    operation_repository: OperationRepository,
}

impl WithCallContext for OperationService {
    fn with_call_context(call_context: CallContext) -> Self {
        Self {
            call_context: call_context.clone(),
            account_service: AccountService::with_call_context(call_context.clone()),
            wallet_service: WalletService::with_call_context(call_context.clone()),
            ..Default::default()
        }
    }
}

impl OperationService {
    pub fn get_operation(&self, id: &OperationId) -> ServiceResult<Operation> {
        let operation = self.operation_repository.get(&Operation::key(*id)).ok_or(
            OperationError::OperationNotFound {
                operation_id: Uuid::from_bytes(id.to_owned()).hyphenated().to_string(),
            },
        )?;

        self.assert_operation_access(&operation)?;

        Ok(operation)
    }

    pub fn get_operation_context(&self, id: &OperationId) -> ServiceResult<OperationContext> {
        let operation = self.get_operation(id)?;
        let processor = OperationProcessorFactory::build(&operation.code);
        let context = processor.get_context(&operation)?;

        Ok(context)
    }

    pub fn list_operations(&self, input: ListOperationsInput) -> ServiceResult<Vec<Operation>> {
        let account = self
            .account_service
            .get_account_by_identity(&self.call_context.caller())?;

        let filter_by_code = match input.code {
            Some(code) => Some(OperationMapper::to_code(code)?),
            None => None,
        };
        let operations = self.operation_repository.find_by_account_where(
            account.id,
            OperationFindByAccountWhereClause {
                created_dt_from: input.from_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
                created_dt_to: input.to_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
                code: filter_by_code,
                status: input.status.map(|status| status.into()),
                read: input.read,
            },
        );

        Ok(operations)
    }

    pub fn list_wallet_operations(
        &self,
        input: ListWalletOperationsInput,
    ) -> ServiceResult<Vec<Operation>> {
        let account = self
            .account_service
            .get_account_by_identity(&self.call_context.caller())?;
        let wallet = self
            .wallet_service
            .get_wallet(HelperMapper::to_uuid(input.wallet_id)?.as_bytes())?;

        let filter_by_code = match input.code {
            Some(code) => Some(OperationMapper::to_code(code)?),
            None => None,
        };
        let operations = self.operation_repository.find_by_wallet_where(
            (account.id, wallet.id),
            OperationWhereClause {
                created_dt_from: input.from_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
                created_dt_to: input.to_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
                code: filter_by_code,
                status: input.status.map(|status| status.into()),
            },
        );

        Ok(operations)
    }

    pub async fn edit_operation(&self, input: EditOperationInput) -> ServiceResult<Operation> {
        let caller_account = self
            .account_service
            .get_account_by_identity(&self.call_context.caller())?;
        let operation_id = HelperMapper::to_uuid(input.operation_id)?;
        let mut operation = self.get_operation(operation_id.as_bytes())?;
        let decision = operation
            .decisions
            .iter_mut()
            .find(|decision| decision.account_id == caller_account.id);

        if decision.is_none() {
            Err(OperationError::Forbidden {
                operation_id: Uuid::from_bytes(operation.id.to_owned())
                    .hyphenated()
                    .to_string(),
            })?
        }

        let decision = decision.unwrap();

        if let (Some(_), Some(_)) = (input.approve.as_ref(), decision.decided_dt.as_ref()) {
            Err(OperationError::NotAllowedModification {
                operation_id: Uuid::from_bytes(operation.id.to_owned())
                    .hyphenated()
                    .to_string(),
            })?
        }

        if let Some(approve) = input.approve {
            decision.status = match approve {
                true => OperationStatus::Adopted,
                false => OperationStatus::Rejected,
            };
            decision.read = true;
            decision.decided_dt = Some(time());
            decision.status_reason = input.reason;
        } else if let Some(read) = input.read {
            decision.read = read;
        }

        operation.validate()?;

        self.operation_repository
            .insert(operation.to_key(), operation.to_owned());

        let processor = OperationProcessorFactory::build(&operation.code);

        let operation = processor
            .post_process(&operation)
            .await
            .expect("Operation post processing failed");

        Ok(operation)
    }

    fn assert_operation_access(&self, operation: &Operation) -> ServiceResult<()> {
        let account = self
            .account_service
            .get_account_by_identity(&self.call_context.caller())?;

        if !operation.accounts().contains(&account.id) {
            Err(OperationError::Forbidden {
                operation_id: Uuid::from_bytes(operation.id.to_owned())
                    .hyphenated()
                    .to_string(),
            })?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::test_utils,
        models::{
            account_test_utils::mock_account, operation_test_utils::mock_operation,
            transfer_test_utils::mock_transfer, Account, OperationDecision,
            OPERATION_METADATA_KEY_TRANSFER_ID,
        },
        repositories::{AccountRepository, TransferRepository},
    };
    use candid::Principal;

    struct TestContext {
        repository: OperationRepository,
        transfer_repository: TransferRepository,
        service: OperationService,
        caller_account: Account,
    }

    fn setup() -> TestContext {
        test_utils::init_canister_config();

        let call_context = CallContext::new(Principal::from_slice(&[9; 29]));
        let mut account = mock_account();
        account.identities = vec![call_context.caller()];

        AccountRepository::default().insert(account.to_key(), account.clone());

        TestContext {
            repository: OperationRepository::default(),
            transfer_repository: TransferRepository::default(),
            service: OperationService::with_call_context(call_context),
            caller_account: account,
        }
    }

    #[test]
    fn get_operation() {
        let ctx = setup();
        let mut operation = mock_operation();
        operation.originator_account_id = Some(ctx.caller_account.id);

        ctx.repository
            .insert(operation.to_key(), operation.to_owned());

        let result = ctx.service.get_operation(&operation.id);

        assert_eq!(operation, result.unwrap());
    }

    #[test]
    fn fail_get_operation_not_allowed() {
        let ctx = setup();
        let mut operation = mock_operation();
        operation.originator_account_id = None;

        ctx.repository
            .insert(operation.to_key(), operation.to_owned());

        let result = ctx.service.get_operation(&operation.id);

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn reject_operation_happy_path() {
        let ctx = setup();
        let transfer_id = Uuid::new_v4();
        let mut transfer = mock_transfer();
        transfer.id = *transfer_id.as_bytes();
        let mut operation = mock_operation();
        operation.originator_account_id = None;
        operation.decisions = vec![OperationDecision {
            account_id: ctx.caller_account.id,
            decided_dt: None,
            last_modification_timestamp: 0,
            read: false,
            status: OperationStatus::Pending,
            status_reason: None,
        }];
        operation.metadata = vec![(
            OPERATION_METADATA_KEY_TRANSFER_ID.to_string(),
            transfer_id.to_string(),
        )];

        ctx.transfer_repository
            .insert(transfer.to_key(), transfer.clone());
        ctx.repository
            .insert(operation.to_key(), operation.to_owned());

        let result = ctx
            .service
            .edit_operation(EditOperationInput {
                operation_id: Uuid::from_bytes(operation.id.to_owned())
                    .hyphenated()
                    .to_string(),
                approve: Some(false),
                reason: None,
                read: Some(true),
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().decisions[0].status,
            OperationStatus::Rejected
        );
    }
}
