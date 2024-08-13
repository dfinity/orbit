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

        if external_canister.is_archived() {
            match &self.operation.kind {
                ConfigureExternalCanisterOperationKind::Delete
                | ConfigureExternalCanisterOperationKind::SoftDelete
                | ConfigureExternalCanisterOperationKind::Settings(_) => {}
                _ => {
                    return Err(RequestExecuteError::Failed {
                        reason: "Canister is archived, please reactivate and try again."
                            .to_string(),
                    });
                }
            }
        }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        factories::requests::requests_test_utils::mock_request_api_input,
        models::{external_canister_test_utils::mock_external_canister, ExternalCanisterState},
        repositories::{EXTERNAL_CANISTER_REPOSITORY, REQUEST_REPOSITORY},
        services::EXTERNAL_CANISTER_SERVICE,
    };
    use configure_external_canister_test_utils::mock_operation_api_input;
    use orbit_essentials::repository::Repository;

    #[tokio::test]
    async fn execute_settings_change_fails_when_archived() {
        let operation_input = mock_operation_api_input();
        let mut external_canister = mock_external_canister();
        external_canister.state = ExternalCanisterState::Archived;
        external_canister.canister_id = operation_input.canister_id;

        EXTERNAL_CANISTER_REPOSITORY.insert(external_canister.to_key(), external_canister);

        let request_id = [0u8; 16];
        let requested_by_user = [1u8; 16];
        let request_input = mock_request_api_input(
            station_api::RequestOperationInput::ConfigureExternalCanister(operation_input.clone()),
        );
        let creator = Box::new(ConfigureExternalCanisterRequestCreate {});
        let request = creator
            .create(
                request_id,
                requested_by_user,
                request_input,
                operation_input,
            )
            .await
            .unwrap();

        REQUEST_REPOSITORY.insert(request.to_key(), request.to_owned());

        match &request.operation {
            RequestOperation::ConfigureExternalCanister(operation) => {
                let stage = ConfigureExternalCanisterRequestExecute::new(
                    &request,
                    operation,
                    Arc::clone(&EXTERNAL_CANISTER_SERVICE),
                )
                .execute()
                .await
                .unwrap_err();

                assert_eq!(
                    stage,
                    RequestExecuteError::Failed {
                        reason: "Canister is archived, please reactivate and try again."
                            .to_string()
                    }
                );
            }
            _ => panic!(
                "Expected ConfigureExternalCanister operation, got {:?}",
                request.operation
            ),
        }
    }
}

#[cfg(test)]
pub mod configure_external_canister_test_utils {
    use candid::Principal;

    pub fn mock_operation_api_input() -> station_api::ConfigureExternalCanisterOperationInput {
        station_api::ConfigureExternalCanisterOperationInput {
            canister_id: Principal::from_slice(&[1; 29]),
            kind: station_api::ConfigureExternalCanisterOperationKindDTO::NativeSettings(
                station_api::DefiniteCanisterSettingsInput {
                    controllers: Some(vec![Principal::from_slice(&[1; 29])]),
                    compute_allocation: None,
                    memory_allocation: None,
                    freezing_threshold: None,
                    reserved_cycles_limit: None,
                },
            ),
        }
    }
}
