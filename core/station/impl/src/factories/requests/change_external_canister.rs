use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{ChangeExternalCanisterOperation, Request, RequestOperation},
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
        let request = Request::from_request_creation_input(
            request_id,
            requested_by_user,
            input,
            RequestOperation::ChangeExternalCanister(ChangeExternalCanisterOperation {
                arg_checksum: operation_input.arg.as_ref().map(|arg| {
                    let mut hasher = Sha256::new();
                    hasher.update(arg);
                    hasher.finalize().to_vec()
                }),
                module_checksum: {
                    if let Some(ref module_extra_chunks) = operation_input.module_extra_chunks {
                        module_extra_chunks.wasm_module_hash.clone()
                    } else {
                        let mut hasher = Sha256::new();
                        hasher.update(&operation_input.module);
                        hasher.finalize().to_vec()
                    }
                },
                input: operation_input.into(),
            }),
            "Change canister".to_string(),
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
                &self.operation.input.module_extra_chunks,
                self.operation.input.arg.clone(),
                false,
                None,
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
