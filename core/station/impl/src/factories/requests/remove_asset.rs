use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{RemoveAssetOperation, Request, RequestOperation},
    services::AssetService,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;

pub struct RemoveAssetRequestCreate {}

#[async_trait]
impl Create<station_api::RemoveAssetOperationInput> for RemoveAssetRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: station_api::CreateRequestInput,
        operation_input: station_api::RemoveAssetOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::from_request_creation_input(
            request_id,
            requested_by_user,
            input,
            RequestOperation::RemoveAsset(RemoveAssetOperation {
                input: operation_input.into(),
            }),
            "Remove asset".to_string(),
        );

        Ok(request)
    }
}

pub struct RemoveAssetRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o RemoveAssetOperation,
    asset_service: AssetService,
}

impl<'p, 'o> RemoveAssetRequestExecute<'p, 'o> {
    pub fn new(request: &'p Request, operation: &'o RemoveAssetOperation) -> Self {
        Self {
            request,
            operation,
            asset_service: AssetService::default(),
        }
    }
}

#[async_trait]
impl Execute for RemoveAssetRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        self.asset_service
            .remove(self.operation.input.clone())
            .map_err(|e| RequestExecuteError::Failed {
                reason: format!("Failed to remove asset: {}", e),
            })?;

        Ok(RequestExecuteStage::Completed(
            self.request.operation.clone(),
        ))
    }
}
