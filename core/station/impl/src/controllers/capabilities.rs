use crate::{
    core::{
        middlewares::{authorize, call_context},
        read_system_info, SUPPORTED_BLOCKCHAINS,
    },
    models::resource::{Resource, SystemResourceAction},
    repositories::ASSET_REPOSITORY,
    SYSTEM_VERSION,
};
use ic_cdk_macros::query;
use lazy_static::lazy_static;
use orbit_essentials::with_middleware;
use orbit_essentials::{api::ApiResult, repository::Repository};
use station_api::{CapabilitiesDTO, CapabilitiesResponse, StandardDataDTO, SupportedBlockchainDTO};

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
        let system = read_system_info();

        Ok(CapabilitiesResponse {
            capabilities: CapabilitiesDTO {
                name: system.get_name().to_string(),
                version: SYSTEM_VERSION.to_string(),
                supported_assets: ASSET_REPOSITORY
                    .list()
                    .into_iter()
                    .map(|asset| asset.into())
                    .collect(),
                supported_blockchains: SUPPORTED_BLOCKCHAINS
                    .iter()
                    .map(|suported_blockchain| SupportedBlockchainDTO {
                        blockchain: suported_blockchain.blockchain.to_string(),
                        supported_standards: suported_blockchain
                            .supported_standards
                            .iter()
                            .map(|standard| StandardDataDTO {
                                required_metadata_fields: standard.get_required_metadata(),
                                standard: standard.to_string(),
                                supported_operations: standard
                                    .get_supported_operations()
                                    .iter()
                                    .map(|operation| operation.to_string())
                                    .collect(),
                                supported_address_formats: standard
                                    .get_info()
                                    .address_formats
                                    .iter()
                                    .map(|format| format.to_string())
                                    .collect(),
                            })
                            .collect(),
                    })
                    .collect(),
            },
        })
    }
}
