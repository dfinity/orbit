use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{Request, RequestOperation, SystemUpgradeOperation, SystemUpgradeTarget},
    services::SystemService,
};
use async_trait::async_trait;
use candid::Encode;
use orbit_essentials::types::UUID;
use sha2::{Digest, Sha256};
use station_api::{CreateRequestInput, SystemUpgradeOperationInput};
use std::sync::Arc;

pub struct SystemUpgradeRequestCreate;

#[async_trait]
impl Create<SystemUpgradeOperationInput> for SystemUpgradeRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: CreateRequestInput,
        operation_input: SystemUpgradeOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::from_request_creation_input(
            request_id,
            requested_by_user,
            input,
            RequestOperation::SystemUpgrade(SystemUpgradeOperation {
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
                take_backup_snapshot: operation_input.take_backup_snapshot,
                input: operation_input.into(),
            }),
            "Upgrade System".to_string(),
        );

        Ok(request)
    }
}

pub struct SystemUpgradeRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o SystemUpgradeOperation,
    system_service: Arc<SystemService>,
}

impl<'p, 'o> SystemUpgradeRequestExecute<'p, 'o> {
    pub fn new(
        request: &'p Request,
        operation: &'o SystemUpgradeOperation,
        system_service: Arc<SystemService>,
    ) -> Self {
        Self {
            request,
            operation,
            system_service,
        }
    }
}

#[async_trait]
impl Execute for SystemUpgradeRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        match self.operation.input.target {
            SystemUpgradeTarget::UpgradeStation => {
                self.system_service
                    .set_self_upgrade_request(self.request.id);

                let default_arg = Encode!(&()).unwrap();
                let arg = self.operation.input.arg.as_ref().unwrap_or(&default_arg);
                let out = self
                    .system_service
                    .upgrade_station(
                        &self.operation.input.module,
                        &self.operation.input.module_extra_chunks,
                        arg,
                        self.operation.input.take_backup_snapshot,
                    )
                    .await
                    .map_err(|err| RequestExecuteError::Failed {
                        reason: format!("failed to upgrade station: {}", err),
                    });

                if out.is_err() {
                    self.system_service.clear_self_upgrade_request();
                }

                out?;

                Ok(RequestExecuteStage::Processing(
                    self.request.operation.clone(),
                ))
            }

            SystemUpgradeTarget::UpgradeUpgrader => {
                self.system_service
                    .upgrade_upgrader(
                        &self.operation.input.module,
                        &self.operation.input.module_extra_chunks,
                        self.operation.input.arg.clone(),
                        self.operation
                            .input
                            .take_backup_snapshot
                            .unwrap_or_default(),
                    )
                    .await
                    .map_err(|err| RequestExecuteError::Failed {
                        reason: format!("failed to upgrade upgrader: {} ({:?})", err, err.details),
                    })?;

                let mut operation = self.request.operation.clone();
                if let RequestOperation::SystemUpgrade(operation) = &mut operation {
                    // Clears the module when the operation is completed, this helps to reduce memory usage.
                    operation.input.module = Vec::new();
                }

                Ok(RequestExecuteStage::Completed(operation))
            }
        }
    }
}
