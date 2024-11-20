use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{
        ConfigureExternalCanisterOperation, ConfigureExternalCanisterOperationKind,
        ExternalCanister, Request, RequestOperation,
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
        let request = Request::from_request_creation_input(
            request_id,
            requested_by_user,
            input,
            RequestOperation::ConfigureExternalCanister(operation_input.into()),
            "Configure canister".to_string(),
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

    fn lookup_operation_external_canister(&self) -> Result<ExternalCanister, RequestExecuteError> {
        let external_canister = self
            .external_canister_service
            .get_external_canister_by_canister_id(&self.operation.canister_id)
            .map_err(|e| RequestExecuteError::Failed {
                reason: format!("External canister not found: {}", e),
            })?;

        Ok(external_canister)
    }
}

#[async_trait]
impl Execute for ConfigureExternalCanisterRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        match &self.operation.kind {
            // these operations require that the canister is managed by the station and an external canister entry
            ConfigureExternalCanisterOperationKind::Delete => {
                let external_canister = self.lookup_operation_external_canister()?;

                self.external_canister_service
                    .hard_delete_external_canister(&external_canister.id)
                    .await
                    .map_err(|e| RequestExecuteError::Failed {
                        reason: format!("Failed to delete canister: {}", e),
                    })?;
            }
            ConfigureExternalCanisterOperationKind::SoftDelete => {
                let external_canister = self.lookup_operation_external_canister()?;

                self.external_canister_service
                    .soft_delete_external_canister(&external_canister.id)
                    .map_err(|e| RequestExecuteError::Failed {
                        reason: format!("Failed to soft delete canister: {}", e),
                    })?;
            }
            ConfigureExternalCanisterOperationKind::Settings(settings) => {
                let external_canister = self.lookup_operation_external_canister()?;

                self.external_canister_service
                    .edit_external_canister(&external_canister.id, settings.clone())
                    .map_err(|e| RequestExecuteError::Failed {
                        reason: format!("Failed to configure settings: {}", e),
                    })?;
            }
            // these operations do not require an external canister entry
            ConfigureExternalCanisterOperationKind::NativeSettings(settings) => {
                self.external_canister_service
                    .change_canister_ic_settings(self.operation.canister_id, settings.clone())
                    .await
                    .map_err(|e| RequestExecuteError::Failed {
                        reason: format!("Failed to configure native settings: {}", e),
                    })?;
            }
        }

        Ok(RequestExecuteStage::Completed(
            self.request.operation.clone(),
        ))
    }
}
