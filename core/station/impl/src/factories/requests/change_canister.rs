use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{
        ChangeCanisterOperation, ChangeCanisterTarget, Request, RequestExecutionPlan,
        RequestOperation,
    },
    services::{ChangeCanisterService, DisasterRecoveryService, SystemService},
};
use async_trait::async_trait;
use candid::Encode;
use orbit_essentials::types::UUID;
use sha2::{Digest, Sha256};
use station_api::{ChangeCanisterOperationInput, CreateRequestInput};
use std::sync::Arc;

pub struct ChangeCanisterRequestCreate;

#[async_trait]
impl Create<ChangeCanisterOperationInput> for ChangeCanisterRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: CreateRequestInput,
        operation_input: ChangeCanisterOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::new(
            request_id,
            requested_by_user,
            Request::default_expiration_dt_ns(),
            RequestOperation::ChangeCanister(ChangeCanisterOperation {
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
            input.title.unwrap_or_else(|| "ChangeCanister".to_string()),
            input.summary,
        );

        Ok(request)
    }
}

pub struct ChangeCanisterRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o ChangeCanisterOperation,
    system_service: Arc<SystemService>,
    change_canister_service: Arc<ChangeCanisterService>,
    disaster_recovery_service: Arc<DisasterRecoveryService>,
}

impl<'p, 'o> ChangeCanisterRequestExecute<'p, 'o> {
    pub fn new(
        request: &'p Request,
        operation: &'o ChangeCanisterOperation,
        system_service: Arc<SystemService>,
        change_canister_service: Arc<ChangeCanisterService>,
        disaster_recovery_service: Arc<DisasterRecoveryService>,
    ) -> Self {
        Self {
            request,
            operation,
            system_service,
            change_canister_service,
            disaster_recovery_service,
        }
    }
}

#[async_trait]
impl Execute for ChangeCanisterRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        match self.operation.input.target {
            ChangeCanisterTarget::UpgradeStation => {
                self.system_service
                    .set_self_upgrade_request(self.request.id);

                let default_arg = Encode!(&()).unwrap();
                let arg = self.operation.input.arg.as_ref().unwrap_or(&default_arg);
                let out = self
                    .change_canister_service
                    .upgrade_station(&self.operation.input.module, arg)
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

            ChangeCanisterTarget::UpgradeUpgrader => {
                self.change_canister_service
                    .upgrade_upgrader(
                        &self.operation.input.module,
                        self.operation.input.arg.clone(),
                    )
                    .await
                    .map_err(|err| RequestExecuteError::Failed {
                        reason: format!("failed to upgrade upgrader: {}", err),
                    })?;

                // The upgrader might have just gained the ability to perform disaster recovery, so sync it now.
                let disaster_recovery_service = Arc::clone(&self.disaster_recovery_service);
                crate::core::ic_cdk::spawn(async move {
                    disaster_recovery_service.sync_all().await;
                });

                let mut operation = self.request.operation.clone();
                if let RequestOperation::ChangeCanister(operation) = &mut operation {
                    // Clears the module when the operation is completed, this helps to reduce memory usage.
                    operation.input.module = Vec::new();
                }

                Ok(RequestExecuteStage::Completed(operation))
            }
        }
    }
}
