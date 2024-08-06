use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{
        ConfigureExternalCanisterOperation, ConfigureExternalCanisterOperationKind, Request,
        RequestExecutionPlan, RequestOperation,
    },
    services::ExternalCanisterService,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;
use station_api::{ConfigureExternalCanisterOperationInput, CreateRequestInput};
use std::sync::Arc;

pub struct ConfigureExternalCanisterRequestCreate;

#[async_trait]
impl Create<ConfigureExternalCanisterOperationInput> for ConfigureExternalCanisterRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: CreateRequestInput,
        operation_input: ConfigureExternalCanisterOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::new(
            request_id,
            requested_by_user,
            Request::default_expiration_dt_ns(),
            RequestOperation::ConfigureExternalCanister(operation_input.into()),
            input
                .execution_plan
                .map(Into::into)
                .unwrap_or(RequestExecutionPlan::Immediate),
            input
                .title
                .unwrap_or_else(|| "Configure canister".to_string()),
            input.summary,
        );

        Ok(request)
    }
}

pub struct ConfigureExternalCanisterRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o ConfigureExternalCanisterOperation,
    external_canister_service: Arc<ExternalCanisterService>,
}

impl<'p, 'o> ConfigureExternalCanisterRequestExecute<'p, 'o> {
    pub fn new(
        request: &'p Request,
        operation: &'o ConfigureExternalCanisterOperation,
        external_canister_service: Arc<ExternalCanisterService>,
    ) -> Self {
        Self {
            request,
            operation,
            external_canister_service,
        }
    }
}

#[async_trait]
impl Execute for ConfigureExternalCanisterRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        let external_canister = self
            .external_canister_service
            .get_external_canister_by_canister_id(&self.operation.canister_id)
            .map_err(|e| RequestExecuteError::Failed {
                reason: format!("External canister not found: {}", e),
            })?;

        match &self.operation.kind {
            ConfigureExternalCanisterOperationKind::Delete => {
                self.external_canister_service
                    .hard_delete_external_canister(&external_canister.id)
                    .await
                    .map_err(|e| RequestExecuteError::Failed {
                        reason: format!("Failed to delete canister: {}", e),
                    })?;
            }
            ConfigureExternalCanisterOperationKind::SoftDelete => {
                self.external_canister_service
                    .soft_delete_external_canister(&external_canister.id)
                    .map_err(|e| RequestExecuteError::Failed {
                        reason: format!("Failed to soft delete canister: {}", e),
                    })?;
            }
            ConfigureExternalCanisterOperationKind::TopUp(cycles) => {
                self.external_canister_service
                    .top_up_canister(external_canister.canister_id, *cycles as u128)
                    .await
                    .map_err(|e| RequestExecuteError::Failed {
                        reason: format!("Failed to top up canister: {}", e),
                    })?;
            }
            ConfigureExternalCanisterOperationKind::NativeSettings(settings) => {
                self.external_canister_service
                    .change_canister_ic_settings(external_canister.canister_id, settings.clone())
                    .await
                    .map_err(|e| RequestExecuteError::Failed {
                        reason: format!("Failed to configure native settings: {}", e),
                    })?;
            }
            ConfigureExternalCanisterOperationKind::Settings(settings) => {
                self.external_canister_service
                    .edit_external_canister(&external_canister.id, settings.clone())
                    .map_err(|e| RequestExecuteError::Failed {
                        reason: format!("Failed to configure settings: {}", e),
                    })?;
            }
        }

        Ok(RequestExecuteStage::Completed(
            self.request.operation.clone(),
        ))
    }
}
