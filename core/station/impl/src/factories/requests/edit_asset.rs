use super::{Create, Execute, RequestExecuteStage};
use crate::{
    errors::{RequestError, RequestExecuteError},
    models::{EditAssetOperation, Request, RequestOperation},
    services::AssetService,
};
use async_trait::async_trait;
use orbit_essentials::types::UUID;

pub struct EditAssetRequestCreate {}

#[async_trait]
impl Create<station_api::EditAssetOperationInput> for EditAssetRequestCreate {
    async fn create(
        &self,
        request_id: UUID,
        requested_by_user: UUID,
        input: station_api::CreateRequestInput,
        operation_input: station_api::EditAssetOperationInput,
    ) -> Result<Request, RequestError> {
        let request = Request::from_request_creation_input(
            request_id,
            requested_by_user,
            input,
            RequestOperation::EditAsset(EditAssetOperation {
                input: operation_input.into(),
            }),
            "Edit asset".to_string(),
        );

        Ok(request)
    }
}

pub struct EditAssetRequestExecute<'p, 'o> {
    request: &'p Request,
    operation: &'o EditAssetOperation,
    asset_service: AssetService,
}

impl<'p, 'o> EditAssetRequestExecute<'p, 'o> {
    pub fn new(request: &'p Request, operation: &'o EditAssetOperation) -> Self {
        Self {
            request,
            operation,
            asset_service: AssetService::default(),
        }
    }
}

#[async_trait]
impl Execute for EditAssetRequestExecute<'_, '_> {
    async fn execute(&self) -> Result<RequestExecuteStage, RequestExecuteError> {
        self.asset_service
            .edit(self.operation.input.clone())
            .map_err(|e| RequestExecuteError::Failed {
                reason: format!("Failed to edit asset: {}", e),
            })?;

        Ok(RequestExecuteStage::Completed(
            self.request.operation.clone(),
        ))
    }
}
