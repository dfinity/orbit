use std::sync::Arc;

use crate::{
    models::{AddAssetOperationInput, Asset},
    repositories::{AssetRepository, ASSET_REPOSITORY},
};
use lazy_static::lazy_static;
use orbit_essentials::{
    api::ServiceResult, model::ModelValidator, repository::Repository, utils::generate_uuid_v4,
};

lazy_static! {
    pub static ref ASSET_SERVICE: Arc<AssetService> =
        Arc::new(AssetService::new(Arc::clone(&ASSET_REPOSITORY),));
}

#[derive(Default, Debug)]
pub struct AssetService {
    asset_repository: Arc<AssetRepository>,
}

impl AssetService {
    pub const DEFAULT_ENTRIES_LIMIT: u16 = 100;
    pub const MAX_LIST_ENTRIES_LIMIT: u16 = 1000;

    pub fn new(asset_repository: Arc<AssetRepository>) -> Self {
        Self { asset_repository }
    }

    pub async fn create(&self, input: AddAssetOperationInput) -> ServiceResult<Asset> {
        let id = generate_uuid_v4().await;
        let asset = Asset {
            id: *id.as_bytes(),
            blockchain: input.blockchain,
            standards: input.standards.into_iter().collect(),
            symbol: input.symbol,
            name: input.name,
            decimals: input.decimals,
            metadata: input.metadata,
        };

        asset.validate()?;

        self.asset_repository.insert(asset.id, asset.clone());

        Ok(asset)
    }
}

#[cfg(test)]
mod tests {}
