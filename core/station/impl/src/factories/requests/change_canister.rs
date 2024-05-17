use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{
        ChangeCanisterOperation, ChangeCanisterTarget, Request, RequestExecutionPlan,
        RequestOperation,
    },
    services::{SystemService, CHANGE_CANISTER_SERVICE},
};
use async_trait::async_trait;
use candid::Encode;
use ic_cdk::api::management_canister::main::CanisterInstallMode;
use orbit_essentials::types::UUID;
use sha2::{Digest, Sha256};
use station_api::{ChangeCanisterOperationInput, CreateRequestInput};
use std::sync::Arc;

pub struct ChangeCanisterRequestCreate;

impl Create<ChangeCanisterOperationInput> for ChangeCanisterRequestCreate {
    fn create(
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
}

impl<'p, 'o> ChangeCanisterRequestExecute<'p, 'o> {
    pub fn new(
        request: &'p Request,
        operation: &'o ChangeCanisterOperation,
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
impl Execute for ChangeCanisterRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        match self.operation.input.target {
            ChangeCanisterTarget::UpgradeStation => {
                self.system_service
                    .set_self_upgrade_request(self.request.id);

                let default_arg = Encode!(&()).unwrap();
                let arg = self.operation.input.arg.as_ref().unwrap_or(&default_arg);
                let out = CHANGE_CANISTER_SERVICE
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
                CHANGE_CANISTER_SERVICE
                    .upgrade_upgrader(
                        &self.operation.input.module,
                        self.operation.input.arg.clone(),
                    )
                    .await
                    .map_err(|err| RequestExecuteError::Failed {
                        reason: format!("failed to upgrade upgrader: {}", err),
                    })?;

                Ok(RequestExecuteStage::Completed(
                    self.request.operation.clone(),
                ))
            }

            ChangeCanisterTarget::UpgradeCanister(canister_id) => {
                CHANGE_CANISTER_SERVICE
                    .install_canister(
                        canister_id,
                        CanisterInstallMode::Upgrade(None),
                        &self.operation.input.module,
                        self.operation.input.arg.clone(),
                    )
                    .await
                    .map_err(|err| RequestExecuteError::Failed {
                        reason: format!("failed to upgrade canister {}: {}", canister_id, err),
                    })?;

                Ok(RequestExecuteStage::Completed(
                    self.request.operation.clone(),
                ))
            }

            ChangeCanisterTarget::InstallCanister(ref input) => {
                CHANGE_CANISTER_SERVICE
                    .install_canister(
                        input.canister_id,
                        input.mode,
                        &self.operation.input.module,
                        self.operation.input.arg.clone(),
                    )
                    .await
                    .map_err(|err| RequestExecuteError::Failed {
                        reason: format!(
                            "failed to install canister {}: {}",
                            input.canister_id, err
                        ),
                    })?;

                Ok(RequestExecuteStage::Completed(
                    self.request.operation.clone(),
                ))
            }
        }
    }
}
