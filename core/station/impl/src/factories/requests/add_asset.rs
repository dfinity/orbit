use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{AddAssetOperation, Request, RequestOperation},
    services::AssetService,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;

pub struct AddAssetRequestCreate {}

#[async_trait]
impl Create<station_api::AddAssetOperationInput> for AddAssetRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: station_api::CreateRequestInput,
        operation_input: station_api::AddAssetOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::from_request_creation_input(
            request_id,
            requested_by_user,
            input,
            RequestOperation::AddAsset(AddAssetOperation {
                asset_id: None,
                input: operation_input.into(),
            }),
            "Add asset".to_string(),
        );

        Ok(request)
    }
}

pub struct AddAssetRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o AddAssetOperation,
    asset_service: AssetService,
}

impl<'p, 'o> AddAssetRequestExecute<'p, 'o> {
    pub fn new(request: &'p Request, operation: &'o AddAssetOperation) -> Self {
        Self {
            request,
            operation,
            asset_service: AssetService::default(),
        }
    }
}

#[async_trait]
impl Execute for AddAssetRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        let asset = self
            .asset_service
            .create(self.operation.input.clone(), None)
            .map_err(|e| RequestExecuteError::Failed {
                reason: format!("Failed to create asset: {}", e),
            })?;

        let mut operation = self.request.operation.clone();

        if let RequestOperation::AddAsset(ref mut operation) = operation {
            operation.asset_id = Some(asset.id);
        }

        Ok(RequestExecuteStage::Completed(operation))
    }
}
