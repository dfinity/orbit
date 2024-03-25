use crate::{
    core::{
        middlewares::{authorize, call_context},
        WALLET_ASSETS,
    },
    models::resource::{Resource, SystemResourceAction},
    SYSTEM_VERSION,
};
use ic_canister_core::api::ApiResult;
use ic_canister_macros::with_middleware;
use ic_cdk_macros::query;
use lazy_static::lazy_static;
use wallet_api::{CapabilitiesDTO, CapabilitiesResponse};

#[query(name = "capabilities")]
async fn capabilities() -> ApiResult<CapabilitiesResponse> {
    CONTROLLER.capabilities().await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: CapabilitiesController = CapabilitiesController::new();
}

#[derive(Debug)]
pub struct CapabilitiesController {}

impl CapabilitiesController {
    fn new() -> Self {
        Self {}
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::System(SystemResourceAction::Capabilities)]))]
    async fn capabilities(&self) -> ApiResult<CapabilitiesResponse> {
        let assets = WALLET_ASSETS.with(|wallet_assets| wallet_assets.borrow().clone());

        Ok(CapabilitiesResponse {
            capabilities: CapabilitiesDTO {
                version: SYSTEM_VERSION.to_string(),
                supported_assets: assets.into_iter().map(|asset| asset.into()).collect(),
            },
        })
    }
}
