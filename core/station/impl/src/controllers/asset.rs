use crate::{
    core::middlewares::{authorize, call_context},
    mappers::HelperMapper,
    models::resource::{Resource, ResourceAction},
    services::AssetService,
};
use ic_cdk_macros::query;
use lazy_static::lazy_static;
use orbit_essentials::api::ApiResult;
use orbit_essentials::with_middleware;
use station_api::{
    AssetCallerPrivilegesDTO, GetAssetInput, GetAssetResponse, ListAssetsInput, ListAssetsResponse,
};

#[query(name = "get_asset")]
async fn get_asset(input: GetAssetInput) -> ApiResult<GetAssetResponse> {
    CONTROLLER.get_asset(input).await
}

#[query(name = "list_assets")]
async fn list_assets(input: ListAssetsInput) -> ApiResult<ListAssetsResponse> {
    CONTROLLER.list_assets(input).await
}

lazy_static! {
    static ref CONTROLLER: AssetController = AssetController::new(AssetService::default());
}

#[derive(Debug)]
pub struct AssetController {
    asset_service: AssetService,
}

impl AssetController {
    pub fn new(asset_service: AssetService) -> Self {
        Self { asset_service }
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::from(&input)]))]
    async fn get_asset(&self, input: GetAssetInput) -> ApiResult<GetAssetResponse> {
        let ctx = call_context();
        let asset = self
            .asset_service
            .get(HelperMapper::to_uuid(input.asset_id)?.as_bytes())?;
        let privileges = self
            .asset_service
            .get_caller_privileges_for_asset(&asset.id, &ctx)
            .await?;

        Ok(GetAssetResponse {
            asset: asset.into(),
            privileges: privileges.into(),
        })
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::Asset(ResourceAction::List)]))]
    async fn list_assets(&self, input: ListAssetsInput) -> ApiResult<ListAssetsResponse> {
        let ctx = call_context();
        let result = self.asset_service.list(input, Some(&ctx))?;
        let mut privileges = Vec::new();

        for asset in &result.items {
            let asset_privileges = self
                .asset_service
                .get_caller_privileges_for_asset(&asset.id, &ctx)
                .await?;

            privileges.push(AssetCallerPrivilegesDTO::from(asset_privileges));
        }

        Ok(ListAssetsResponse {
            assets: result.items.into_iter().map(Into::into).collect(),
            next_offset: result.next_offset,
            total: result.total,
            privileges,
        })
    }
}
