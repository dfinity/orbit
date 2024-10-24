use std::sync::Arc;

use crate::{
    core::{authorization::Authorization, utils::retain_accessible_resources, CallContext},
    errors::AssetError,
    models::{
        resource::{Resource, ResourceAction, ResourceId},
        AddAssetOperationInput, Asset, AssetCallerPrivileges, AssetId, EditAssetOperationInput,
        RemoveAssetOperationInput,
    },
    repositories::{AssetRepository, ACCOUNT_REPOSITORY, ASSET_REPOSITORY},
};
use lazy_static::lazy_static;
use orbit_essentials::{
    api::ServiceResult,
    model::ModelValidator,
    pagination::{paginated_items, PaginatedData, PaginatedItemsArgs},
    repository::Repository,
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

    pub fn create(
        &self,
        input: AddAssetOperationInput,
        with_asset_id: Option<AssetId>,
    ) -> ServiceResult<Asset> {
        let id = with_asset_id.unwrap_or(*Uuid::new_v4().as_bytes());

        let asset = Asset {
            id,
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

        let accounts = ACCOUNT_REPOSITORY.list();

        for account in accounts {
            if account
                .assets
                .iter()
                .any(|account_asset| account_asset.asset_id == asset.id)
            {
                return Err(AssetError::AssetInUse {
                    id: Uuid::from_bytes(account.id).hyphenated().to_string(),
                    resource: "account".to_string(),
                })?;
            }
        }

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
mod tests {
    use orbit_essentials::repository::Repository;
    use station_api::ListAssetsInput;

    use crate::{
        models::{
            account_test_utils::mock_account, asset_test_utils::mock_asset, AddAssetOperationInput,
            TokenStandard,
        },
        repositories::{ACCOUNT_REPOSITORY, ASSET_REPOSITORY},
    };

    use super::AssetService;

    #[tokio::test]
    async fn test_asset_creation() {
        let service = AssetService::default();

        service
            .create(
                AddAssetOperationInput {
                    blockchain: crate::models::Blockchain::InternetComputer,
                    standards: vec![TokenStandard::InternetComputerNative],
                    decimals: 8,
                    metadata: Default::default(),
                    name: "ICP".to_string(),
                    symbol: "ICP".to_string(),
                },
                None,
            )
            .expect("Failed to create asset");

        let assets = ASSET_REPOSITORY.list();

        assert_eq!(assets.len(), 1);
        assert_eq!(assets[0].name, "ICP");
    }

    #[tokio::test]
    async fn test_asset_edit() {
        let service = AssetService::default();
        let mut mock_asset = mock_asset();
        mock_asset.name = "Bitcoin".to_string();
        ASSET_REPOSITORY.insert(mock_asset.id, mock_asset.clone());

        service
            .edit(crate::models::EditAssetOperationInput {
                asset_id: mock_asset.id,
                name: Some("Internet Computer".to_string()),
                symbol: Some("ICP".to_string()),
                decimals: Some(8),
                change_metadata: None,
                blockchain: None,
                standards: None,
            })
            .expect("Failed to edit asset");

        let assets = ASSET_REPOSITORY.list();

        assert_eq!(assets.len(), 1);
        assert_eq!(assets[0].name, "Internet Computer");
    }

    #[tokio::test]
    async fn test_unused_asset_remove() {
        let service = AssetService::default();
        let mock_asset = mock_asset();
        ASSET_REPOSITORY.insert(mock_asset.id, mock_asset.clone());

        service
            .remove(crate::models::RemoveAssetOperationInput {
                asset_id: mock_asset.id,
            })
            .expect("Failed to remove asset");

        let assets = ASSET_REPOSITORY.list();

        assert_eq!(assets.len(), 0);
    }

    #[tokio::test]
    async fn test_used_asset_remove_fails() {
        let service = AssetService::default();
        let mock_asset = mock_asset();
        ASSET_REPOSITORY.insert(mock_asset.id, mock_asset.clone());

        let mock_account = mock_account();

        ACCOUNT_REPOSITORY.insert(mock_account.to_key(), mock_account.clone());

        service
            .remove(crate::models::RemoveAssetOperationInput {
                asset_id: mock_asset.id,
            })
            .expect_err("Asset should not be removed");

        let assets = ASSET_REPOSITORY.list();

        assert_eq!(assets.len(), 1);
    }

    #[tokio::test]
    async fn test_asset_list() {
        let service = AssetService::default();
        let mock_asset = mock_asset();
        ASSET_REPOSITORY.insert(mock_asset.id, mock_asset.clone());

        let assets = service
            .list(ListAssetsInput { paginate: None }, None)
            .expect("Failed to list assets");

        assert_eq!(assets.items.len(), 1);
        assert_eq!(assets.items[0].name, "Internet Computer");
    }

    #[tokio::test]
    async fn test_asset_get() {
        let service = AssetService::default();
        let mock_asset = mock_asset();
        ASSET_REPOSITORY.insert(mock_asset.id, mock_asset.clone());

        let asset = service.get(&mock_asset.id).expect("Failed to get asset");

        assert_eq!(asset.name, "Internet Computer");
    }

    #[tokio::test]
    async fn test_asset_uniqueness() {
        let service = AssetService::default();

        service
            .create(
                AddAssetOperationInput {
                    blockchain: crate::models::Blockchain::InternetComputer,
                    standards: vec![TokenStandard::InternetComputerNative],
                    decimals: 8,
                    metadata: Default::default(),
                    name: "ICP".to_string(),
                    symbol: "ICP".to_string(),
                },
                None,
            )
            .expect("Failed to create asset");

        service
            .create(
                AddAssetOperationInput {
                    blockchain: crate::models::Blockchain::InternetComputer,
                    standards: vec![TokenStandard::InternetComputerNative],
                    decimals: 8,
                    metadata: Default::default(),
                    name: "ICP".to_string(),
                    symbol: "ICP".to_string(),
                },
                None,
            )
            .expect_err("Asset with the same symbol and blockchain should not be allowed");

        service
            .create(
                AddAssetOperationInput {
                    blockchain: crate::models::Blockchain::InternetComputer,
                    standards: vec![TokenStandard::InternetComputerNative],
                    decimals: 8,
                    metadata: Default::default(),
                    name: "ICP".to_string(),
                    symbol: "ICP2".to_string(),
                },
                None,
            )
            .expect("Failed to create asset");
    }
}
