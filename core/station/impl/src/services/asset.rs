use std::sync::Arc;

use crate::{
    core::{authorization::Authorization, utils::retain_accessible_resources, CallContext},
    errors::AssetError,
    models::{
        resource::{Resource, ResourceAction, ResourceId},
        AddAssetOperationInput, Asset, AssetCallerPrivileges, AssetId, EditAssetOperationInput,
        RemoveAssetOperationInput,
    },
    repositories::{AssetRepository, ASSET_REPOSITORY},
};
use lazy_static::lazy_static;
use orbit_essentials::{
    api::ServiceResult,
    model::ModelValidator,
    pagination::{paginated_items, PaginatedData, PaginatedItemsArgs},
    repository::Repository,
    utils::generate_uuid_v4,
};
use station_api::ListAssetsInput;
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
    pub const DEFAULT_LIST_ASSETS_LIMIT: u16 = 100;
    pub const MAX_LIST_ASSETS_LIMIT: u16 = 1000;

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

    pub async fn get_caller_privileges_for_asset(
        &self,
        asset_id: &AssetId,
        ctx: &CallContext,
    ) -> ServiceResult<AssetCallerPrivileges> {
        Ok(AssetCallerPrivileges {
            id: *asset_id,
            can_edit: Authorization::is_allowed(
                ctx,
                &Resource::Asset(ResourceAction::Update(ResourceId::Id(*asset_id))),
            ),
            can_delete: Authorization::is_allowed(
                ctx,
                &Resource::Asset(ResourceAction::Delete(ResourceId::Id(*asset_id))),
            ),
        })
    }

    pub fn list(
        &self,
        input: ListAssetsInput,
        ctx: Option<&CallContext>,
    ) -> ServiceResult<PaginatedData<Asset>> {
        let mut assets = self.asset_repository.list();

        if let Some(ctx) = ctx {
            // filter out assets that the caller does not have access to read
            retain_accessible_resources(ctx, &mut assets, |asset| {
                Resource::Asset(crate::models::resource::ResourceAction::Read(
                    crate::models::resource::ResourceId::Id(asset.id),
                ))
            });
        }

        let result = paginated_items(PaginatedItemsArgs {
            offset: input.paginate.to_owned().and_then(|p| p.offset),
            limit: input.paginate.and_then(|p| p.limit),
            default_limit: Some(Self::DEFAULT_LIST_ASSETS_LIMIT),
            max_limit: Some(Self::MAX_LIST_ASSETS_LIMIT),
            items: &assets,
        })?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {}
