use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{CallExternalCanisterOperation, Request, RequestExecutionPlan, RequestOperation},
    services::ExternalCanisterService,
};
use async_trait::async_trait;
use candid::Decode;
use orbit_essentials::types::UUID;
use sha2::{Digest, Sha256};
use station_api::{CallExternalCanisterOperationInput, CreateRequestInput};
use std::sync::Arc;

pub struct CallExternalCanisterRequestCreate {
    pub external_canister_service: Arc<ExternalCanisterService>,
}

#[async_trait]
impl Create<CallExternalCanisterOperationInput> for CallExternalCanisterRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: CreateRequestInput,
        operation_input: CallExternalCanisterOperationInput,
    ) -> Result<Request, RequestError> {
        let arg_rendering = match operation_input.validation_method {
            Some(ref validation_method) => {
                let rendering_bytes = self
                    .external_canister_service
                    .call_external_canister(
                        validation_method.canister_id,
                        validation_method.method_name.clone(),
                        operation_input.arg.clone(),
                        None,
                    )
                    .await
                    .map_err(|err| RequestError::ValidationError {
                        info: format!(
                            "failed to call validation canister {}: {}",
                            validation_method.canister_id, err
                        ),
                    })?;
                let rendering =
                    Decode!(&rendering_bytes, Result<String, String>).map_err(|err| {
                        RequestError::ValidationError {
                            info: format!(
                                "failed to decode validation canister {} reply: {}",
                                validation_method.canister_id, err
                            ),
                        }
                    })?;
                Some(rendering.map_err(|err| RequestError::ValidationError {
                    info: format!("failed to validate call external canister request: {}", err),
                })?)
            }
            None => None,
        };

        let request = Request::new(
            request_id,
            requested_by_user,
            Request::default_expiration_dt_ns(),
            RequestOperation::CallExternalCanister(CallExternalCanisterOperation {
                arg_checksum: operation_input.arg.as_ref().map(|arg| {
                    let mut hasher = Sha256::new();
                    hasher.update(arg);
                    hasher.finalize().to_vec()
                }),
                arg_rendering,
                execution_method_reply: None,
                input: operation_input.into(),
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(RequestExecutionPlan::Immediate),
            input
                .title
                .unwrap_or_else(|| "CallExternalCanister".to_string()),
            input.summary,
        );

        Ok(request)
    }
}

pub struct CallExternalCanisterRequestExecute<'p, 'o> {
    _request: &'p Request,
    operation: &'o CallExternalCanisterOperation,
    external_canister_service: Arc<ExternalCanisterService>,
}

impl<'p, 'o> CallExternalCanisterRequestExecute<'p, 'o> {
    pub fn new(
        request: &'p Request,
        operation: &'o CallExternalCanisterOperation,
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
impl Execute for CallExternalCanisterRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        let execution_method_reply = self
            .external_canister_service
            .call_external_canister(
                self.operation.input.execution_method.canister_id,
                self.operation.input.execution_method.method_name.clone(),
                self.operation.input.arg.clone(),
                self.operation.input.execution_method_cycles,
            )
            .await
            .map_err(|err| RequestExecuteError::Failed {
                reason: format!(
                    "failed to call external canister {}: {}",
                    self.operation.input.execution_method.canister_id, err
                ),
            })?;
        let mut call_external_canister_operation = self.operation.clone();
        call_external_canister_operation.execution_method_reply = Some(execution_method_reply);

        Ok(RequestExecuteStage::Completed(
            RequestOperation::CallExternalCanister(call_external_canister_operation),
        ))
    }
}
