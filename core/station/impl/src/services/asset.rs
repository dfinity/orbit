use std::sync::Arc;

use crate::{
    errors::AssetError,
    models::{
        AddAssetOperationInput, Asset, AssetId, EditAssetOperationInput, RemoveAssetOperationInput,
    },
    repositories::{AssetRepository, ASSET_REPOSITORY},
};
use lazy_static::lazy_static;
use orbit_essentials::{
    api::ServiceResult, model::ModelValidator, repository::Repository, utils::generate_uuid_v4,
};
use uuid::Uuid;

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

    pub fn get(&self, asset_id: &AssetId) -> ServiceResult<Asset> {
        let asset = self
            .asset_repository
            .get(asset_id)
            .ok_or(AssetError::NotFound {
                id: Uuid::from_bytes(*asset_id).hyphenated().to_string(),
            })?;

        Ok(asset)
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

    pub fn edit(&self, input: EditAssetOperationInput) -> ServiceResult<Asset> {
        let mut asset = self.get(&input.asset_id)?;

        if let Some(name) = input.name {
            asset.name = name;
        }

        if let Some(symbol) = input.symbol {
            asset.symbol = symbol;
        }

        if let Some(decimals) = input.decimals {
            asset.decimals = decimals;
        }

        if let Some(change_metadata) = input.change_metadata {
            asset.metadata.change(change_metadata);
        }

        if let Some(blockchain) = input.blockchain {
            asset.blockchain = blockchain;
        }

        if let Some(standards) = input.standards {
            asset.standards = standards.into_iter().collect();
        }

        asset.validate()?;

        self.asset_repository.insert(asset.id, asset.clone());

        Ok(asset)
    }

    pub fn remove(&self, input: RemoveAssetOperationInput) -> ServiceResult<Asset> {
        let asset = self.get(&input.asset_id)?;

        self.asset_repository.remove(&input.asset_id);

        Ok(asset)
    }
}

#[cfg(test)]
mod tests {}
