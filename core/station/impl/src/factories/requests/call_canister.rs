use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{CallCanisterOperation, Request, RequestExecutionPlan, RequestOperation},
    services::ExternalCanisterService,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;
use sha2::{Digest, Sha256};
use station_api::{CallCanisterOperationInput, CreateRequestInput};
use std::sync::Arc;

pub struct CallCanisterRequestCreate;

impl Create<CallCanisterOperationInput> for CallCanisterRequestCreate {
    fn create(
        request_id: UUID,
        requested_by_user: UUID,
        input: CreateRequestInput,
        operation_input: CallCanisterOperationInput,
    ) -> Result<Request, RequestError> {
        let mut hasher = Sha256::new();
        hasher.update(operation_input.arg.clone());
        let arg_checksum = hasher.finalize().to_vec();
        let request = Request::new(
            request_id,
            requested_by_user,
            Request::default_expiration_dt_ns(),
            RequestOperation::CallCanister(CallCanisterOperation {
                arg_checksum,
                arg_rendering: None,
                execution_method_reply: None,
                input: operation_input.into(),
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(RequestExecutionPlan::Immediate),
            input.title.unwrap_or_else(|| "CallCanister".to_string()),
            input.summary,
        );

        Ok(request)
    }
}

pub struct CallCanisterRequestExecute<'p, 'o> {
    _request: &'p Request,
    operation: &'o CallCanisterOperation,
    external_canister_service: Arc<ExternalCanisterService>,
}

impl<'p, 'o> CallCanisterRequestExecute<'p, 'o> {
    pub fn new(
        request: &'p Request,
        operation: &'o CallCanisterOperation,
        external_canister_service: Arc<ExternalCanisterService>,
    ) -> Self {
        Self {
            _request: request,
            operation,
            external_canister_service,
        }
    }
}

#[async_trait]
impl Execute for CallCanisterRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        let execution_method_reply = self
            .external_canister_service
            .call_canister(
                self.operation.input.execution_method.canister_id,
                self.operation.input.execution_method.method_name.clone(),
                self.operation.input.arg.clone(),
            )
            .await
            .map_err(|err| RequestExecuteError::Failed {
                reason: format!(
                    "failed to call canister {}: {}",
                    self.operation.input.execution_method.canister_id, err
                ),
            })?;
        let mut call_canister_operation = self.operation.clone();
        call_canister_operation.execution_method_reply = Some(execution_method_reply);

        Ok(RequestExecuteStage::Completed(
            RequestOperation::CallCanister(call_canister_operation),
        ))
    }
}
