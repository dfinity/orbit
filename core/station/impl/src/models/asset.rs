use orbit_essentials::{
    model::{ModelKey, ModelValidator, ModelValidatorResult},
    storable,
    types::UUID,
};

use super::{Blockchain, BlockchainStandard};
use crate::{errors::AssetError, models::Metadata, repositories::ASSET_REPOSITORY};
use std::{
    collections::BTreeSet,
    hash::{Hash, Hasher},
};

pub type AssetId = UUID;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Asset {
    pub id: AssetId,
    /// The blockchain identifier (e.g., `ethereum`, `bitcoin`, `icp`, etc.)
    pub blockchain: Blockchain,
    // The asset standard that is supported (e.g. `erc20`, `native`, etc.), canonically
    // represented as a lowercase string with spaces replaced with underscores.
    pub standards: BTreeSet<BlockchainStandard>,
    /// The asset symbol (e.g. `ICP`, `BTC`, `ETH`, etc.)
    pub symbol: String,
    /// The asset name (e.g. `Internet Computer`, `Bitcoin`, `Ethereum`, etc.)
    pub name: String,
    /// The number of decimal places that the asset supports (e.g. `8` for `BTC`, `18` for `ETH`, etc.)
    pub decimals: u32,
    /// The asset metadata (e.g. `{"logo": "https://example.com/logo.png"}`),
    /// also, in the case of non-native assets, it can contain other required
    /// information (e.g. `{"address": "0x1234"}`).
    pub metadata: Metadata,
}

impl Asset {
    pub const DECIMALS_RANGE: (u32, u32) = (0, 18);
    pub const SYMBOL_RANGE: (u16, u16) = (1, 32);
    pub const NAME_RANGE: (u16, u16) = (1, 64);
}

impl ModelKey<AssetId> for Asset {
    fn key(&self) -> AssetId {
        self.id
    }
}

impl Hash for Asset {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.blockchain.hash(state);
        self.standards.hash(state);
        self.symbol.hash(state);
        self.name.hash(state);

        // For HashMap we need to sort the keys first to ensure that the hash is stable.
        let mut keys: Vec<&String> = self.metadata.keys();
        keys.sort();
        keys.hash(state);
    }
}

#[derive(Debug, Clone)]
pub struct AssetCallerPrivileges {
    pub id: AssetId,
    pub can_edit: bool,
    pub can_delete: bool,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AssetEntryKey {
    /// The address book entry id, which is a UUID.
    pub id: AssetId,
}

fn validate_symbol(symbol: &str) -> ModelValidatorResult<AssetError> {
    if (symbol.len() < Asset::SYMBOL_RANGE.0 as usize)
        || (symbol.len() > Asset::SYMBOL_RANGE.1 as usize)
    {
        return Err(AssetError::InvalidSymbolLength {
            min_length: Asset::SYMBOL_RANGE.0,
            max_length: Asset::SYMBOL_RANGE.1,
        });
    }

    Ok(())
}

fn validate_name(name: &str) -> ModelValidatorResult<AssetError> {
    if (name.len() < Asset::NAME_RANGE.0 as usize) || (name.len() > Asset::NAME_RANGE.1 as usize) {
        return Err(AssetError::InvalidNameLength {
            min_length: Asset::NAME_RANGE.0,
            max_length: Asset::NAME_RANGE.1,
        });
    }

    Ok(())
}

fn validate_decimals(decimals: u32) -> ModelValidatorResult<AssetError> {
    if (decimals < Asset::DECIMALS_RANGE.0) || (decimals > Asset::DECIMALS_RANGE.1) {
        return Err(AssetError::InvalidDecimals {
            min: Asset::DECIMALS_RANGE.0,
            max: Asset::DECIMALS_RANGE.1,
        });
    }

    Ok(())
}

fn validate_uniqueness(
    asset_id: &AssetId,
    symbol: &str,
    blockchain: &Blockchain,
) -> ModelValidatorResult<AssetError> {
    if let Some(existing_asset_id) =
        ASSET_REPOSITORY.exists_unique(blockchain.to_string().as_str(), symbol)
    {
        if existing_asset_id != *asset_id {
            return Err(AssetError::AlreadyExists {
                symbol: symbol.to_string(),
                blockchain: blockchain.to_string(),
            });
        }
    }

    Ok(())
}

impl ModelValidator<AssetError> for Asset {
    fn validate(&self) -> ModelValidatorResult<AssetError> {
        validate_symbol(&self.symbol)?;
        validate_name(&self.name)?;
        validate_decimals(self.decimals)?;
        validate_uniqueness(&self.id, &self.symbol, &self.blockchain)?;

        self.metadata.validate()?;

        Ok(())
    }
}

#[cfg(any(test, feature = "canbench"))]
pub mod asset_test_utils {

    use std::collections::BTreeSet;

    use crate::models::{Blockchain, BlockchainStandard, Metadata};

    use super::Asset;

    pub fn mock_asset() -> Asset {
        Asset {
            id: [0; 16],
            blockchain: Blockchain::InternetComputer,
            standards: BTreeSet::from([BlockchainStandard::Native]),
            symbol: "ICP".to_string(),
            name: "Internet Computer".to_string(),
            decimals: 8,
            metadata: Metadata::default(),
        }
    }
}

#[cfg(test)]
mod test {

    use orbit_essentials::repository::Repository;

    use super::*;

    #[test]
    fn test_name_validation() {
        let mut asset = asset_test_utils::mock_asset();
        assert!(asset.validate().is_ok());

        asset.name = "".to_string();
        assert!(asset.validate().is_err());

        asset.name = "a".repeat(Asset::NAME_RANGE.1 as usize + 1);
        assert!(asset.validate().is_err());
    }

    #[test]
    fn test_symbol_validation() {
        let mut asset = asset_test_utils::mock_asset();
        assert!(asset.validate().is_ok());

        asset.symbol = "".to_string();
        assert!(asset.validate().is_err());

        asset.symbol = "a".repeat(Asset::SYMBOL_RANGE.1 as usize + 1);
        assert!(asset.validate().is_err());
    }

    #[test]
    fn test_decimals_validation() {
        let mut asset = asset_test_utils::mock_asset();
        assert!(asset.validate().is_ok());

        asset.decimals = Asset::DECIMALS_RANGE.1 + 1;
        assert!(asset.validate().is_err());
    }

    #[test]
    fn test_validate_uniqueness() {
        let mut asset = asset_test_utils::mock_asset();
        assert!(asset.validate().is_ok());

        ASSET_REPOSITORY.insert(asset.key(), asset.clone());

        // this passes uniqueness test because the asset id is the same
        assert!(asset.validate().is_ok());

        // this fails uniqueness test because the asset id is different
        asset.id = [1; 16];

        assert!(matches!(
            asset.validate().expect_err("Asset should not be unique"),
            AssetError::AlreadyExists { .. }
        ));
    }
}
