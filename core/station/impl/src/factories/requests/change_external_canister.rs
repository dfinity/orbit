use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{ChangeExternalCanisterOperation, Request, RequestExecutionPlan, RequestOperation},
    services::ChangeCanisterService,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;
use sha2::{Digest, Sha256};
use station_api::{ChangeExternalCanisterOperationInput, CreateRequestInput};
use std::sync::Arc;

pub struct ChangeExternalCanisterRequestCreate;

#[async_trait]
impl Create<ChangeExternalCanisterOperationInput> for ChangeExternalCanisterRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: CreateRequestInput,
        operation_input: ChangeExternalCanisterOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::new(
            request_id,
            requested_by_user,
            Request::default_expiration_dt_ns(),
            RequestOperation::ChangeExternalCanister(ChangeExternalCanisterOperation {
                arg_checksum: operation_input.arg.as_ref().map(|arg| {
                    let mut hasher = Sha256::new();
                    hasher.update(arg);
                    hasher.finalize().to_vec()
                }),
                module_checksum: {
                    let mut hasher = Sha256::new();
                    hasher.update(&operation_input.module);
                    hasher.finalize().to_vec()
                },
                input: operation_input.into(),
            }),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(RequestExecutionPlan::Immediate),
            input
                .title
                .unwrap_or_else(|| "ChangeExternalCanister".to_string()),
            input.summary,
        );

        Ok(request)
    }
}

pub struct ChangeExternalCanisterRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o ChangeExternalCanisterOperation,
    change_canister_service: Arc<ChangeCanisterService>,
}

impl<'p, 'o> ChangeExternalCanisterRequestExecute<'p, 'o> {
    pub fn new(
        request: &'p Request,
        operation: &'o ChangeExternalCanisterOperation,
        change_canister_service: Arc<ChangeCanisterService>,
    ) -> Self {
        Self {
            request,
            operation,
            change_canister_service,
        }
    }
}

#[async_trait]
impl Execute for ChangeExternalCanisterRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        self.change_canister_service
            .install_canister(
                self.operation.input.canister_id,
                self.operation.input.mode.clone(),
                &self.operation.input.module,
                &None,
                self.operation.input.arg.clone(),
            )
            .await
            .map_err(|err| RequestExecuteError::Failed {
                reason: format!(
                    "failed to install external canister {}: {}",
                    self.operation.input.canister_id, err
                ),
            })?;

        Ok(RequestExecuteStage::Completed(
            self.request.operation.clone(),
        ))
    }
}
