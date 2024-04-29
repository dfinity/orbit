use crate::{
    core::{
        middlewares::{authorize, call_context},
        read_system_info, ASSETS,
    },
    models::resource::{Resource, SystemResourceAction},
    SYSTEM_VERSION,
};
use ic_cdk_macros::query;
use lazy_static::lazy_static;
use orbit_essentials::api::ApiResult;
use orbit_essentials::with_middleware;
use station_api::{CapabilitiesDTO, CapabilitiesResponse};

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
        let assets = ASSETS.with(|asset| asset.borrow().clone());
        let system = read_system_info();

        Ok(CapabilitiesResponse {
            capabilities: CapabilitiesDTO {
                name: system.get_name().to_string(),
                version: SYSTEM_VERSION.to_string(),
                supported_assets: assets.into_iter().map(|asset| asset.into()).collect(),
            },
        })
    }
}
