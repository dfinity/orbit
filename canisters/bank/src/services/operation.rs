use super::{AccountService, UserService};
use crate::{
    core::{ic_cdk::api::time, CallContext, WithCallContext},
    errors::OperationError,
    factories::operations::OperationProcessorFactory,
    mappers::{HelperMapper, OperationMapper},
    models::{Operation, OperationContext, OperationId, OperationStatus},
    repositories::{OperationFindByUserWhereClause, OperationRepository, OperationWhereClause},
    transport::{EditOperationInput, ListAccountOperationsInput, ListOperationsInput},
};
use ic_canister_core::repository::Repository;
use ic_canister_core::utils::rfc3339_to_timestamp;
use ic_canister_core::{api::ServiceResult, model::ModelValidator};
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct OperationService {
    call_context: CallContext,
    user_service: UserService,
    account_service: AccountService,
    operation_repository: OperationRepository,
}

impl WithCallContext for OperationService {
    fn with_call_context(call_context: CallContext) -> Self {
        Self {
            call_context: call_context.clone(),
            user_service: UserService::with_call_context(call_context.clone()),
            account_service: AccountService::with_call_context(call_context.clone()),
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
        let user = self
            .user_service
            .get_user_by_identity(&self.call_context.caller())?;

        let filter_by_code = match input.code {
            Some(code) => Some(OperationMapper::to_code(code)?),
            None => None,
        };
        let operations = self.operation_repository.find_by_user_where(
            user.id,
            OperationFindByUserWhereClause {
                created_dt_from: input.from_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
                created_dt_to: input.to_dt.map(|dt| rfc3339_to_timestamp(dt.as_str())),
                code: filter_by_code,
                status: input.status.map(|status| status.into()),
                read: input.read,
            },
        );

        Ok(operations)
    }

    pub fn list_account_operations(
        &self,
        input: ListAccountOperationsInput,
    ) -> ServiceResult<Vec<Operation>> {
        let user = self
            .user_service
            .get_user_by_identity(&self.call_context.caller())?;
        let account = self
            .account_service
            .get_account(HelperMapper::to_uuid(input.account_id)?.as_bytes())?;

        let filter_by_code = match input.code {
            Some(code) => Some(OperationMapper::to_code(code)?),
            None => None,
        };
        let operations = self.operation_repository.find_by_account_where(
            (user.id, account.id),
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
        let caller_user = self
            .user_service
            .get_user_by_identity(&self.call_context.caller())?;
        let operation_id = HelperMapper::to_uuid(input.operation_id)?;
        let mut operation = self.get_operation(operation_id.as_bytes())?;
        let decision = operation
            .decisions
            .iter_mut()
            .find(|decision| decision.user_id == caller_user.id);

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
        let user = self
            .user_service
            .get_user_by_identity(&self.call_context.caller())?;

        if !operation.users().contains(&user.id) {
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
            operation_test_utils::mock_operation, transfer_test_utils::mock_transfer,
            user_test_utils::mock_user, OperationDecision, User,
            OPERATION_METADATA_KEY_TRANSFER_ID,
        },
        repositories::{TransferRepository, UserRepository},
    };
    use candid::Principal;

    struct TestContext {
        repository: OperationRepository,
        transfer_repository: TransferRepository,
        service: OperationService,
        caller_user: User,
    }

    fn setup() -> TestContext {
        test_utils::init_canister_config();

        let call_context = CallContext::new(Principal::from_slice(&[9; 29]));
        let mut user = mock_user();
        user.identities = vec![call_context.caller()];

        UserRepository::default().insert(user.to_key(), user.clone());

        TestContext {
            repository: OperationRepository::default(),
            transfer_repository: TransferRepository::default(),
            service: OperationService::with_call_context(call_context),
            caller_user: user,
        }
    }

    #[test]
    fn get_operation() {
        let ctx = setup();
        let mut operation = mock_operation();
        operation.proposed_by = Some(ctx.caller_user.id);

        ctx.repository
            .insert(operation.to_key(), operation.to_owned());

        let result = ctx.service.get_operation(&operation.id);

        assert_eq!(operation, result.unwrap());
    }

    #[test]
    fn fail_get_operation_not_allowed() {
        let ctx = setup();
        let mut operation = mock_operation();
        operation.proposed_by = None;

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
        operation.proposed_by = None;
        operation.decisions = vec![OperationDecision {
            user_id: ctx.caller_user.id,
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
