use candid::Principal;
use orbit_essentials::{
    model::{ModelKey, ModelValidator, ModelValidatorResult},
    storable,
    types::UUID,
};

use super::{Blockchain, TokenStandard};
use crate::{
    errors::AssetError,
    models::{ChangeMetadata, Metadata},
    repositories::ASSET_REPOSITORY,
};
use std::{
    collections::BTreeSet,
    hash::{Hash, Hasher},
};

pub type AssetId = UUID;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Asset {
    pub id: AssetId,
    /// The blockchain identifier (e.g., `ethereum`, `bitcoin`, `icp`, etc.)
    pub blockchain: Blockchain,
    // The asset standard that is supported (e.g. `erc20`, `icp_native`, etc.), canonically
    // represented as a lowercase string with spaces replaced with underscores.
    pub standards: BTreeSet<TokenStandard>,
    /// The asset symbol (e.g. `ICP`, `BTC`, `ETH`, etc.)
    pub symbol: String,
    /// The asset name (e.g. `Internet Computer`, `Bitcoin`, `Ethereum`, etc.)
    pub name: String,
    /// The number of decimal places that the asset supports (e.g. `8` for `BTC`, `18` for `ETH`, etc.)
    pub decimals: u32,
    /// The asset metadata (e.g. `{"logo": "https://example.com/logo.png"}`).
    pub metadata: Metadata,
}

impl Asset {
    pub const DECIMALS_RANGE: (u32, u32) = (0, 18);
    pub const SYMBOL_RANGE: (u16, u16) = (1, 32);
    pub const NAME_RANGE: (u16, u16) = (1, 64);

    pub fn ledger_canister_id(&self) -> Option<String> {
        self.metadata
            .get(TokenStandard::METADATA_KEY_LEDGER_CANISTER_ID)
    }

    /// Whether applying `change_metadata` would repoint or drop a ledger canister id that is
    /// already set. Setting one for the first time is not a change.
    pub fn changes_ledger_canister_id(&self, change_metadata: &ChangeMetadata) -> bool {
        let before = self.ledger_canister_id();
        if before.is_none() {
            return false;
        }

        let mut metadata = self.metadata.clone();
        metadata.change(change_metadata.clone());

        before != metadata.get(TokenStandard::METADATA_KEY_LEDGER_CANISTER_ID)
    }
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

    if !symbol.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(AssetError::InvalidSymbol);
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

/// The ledger canister id is only parsed when a transfer or balance read resolves it, and it
/// cannot be corrected in place once set, so an unparseable value is rejected at write time.
fn validate_ledger_canister_id(asset: &Asset) -> ModelValidatorResult<AssetError> {
    let Some(ledger_canister_id) = asset.ledger_canister_id() else {
        return Ok(());
    };

    Principal::from_text(&ledger_canister_id).map_err(|_| AssetError::InvalidLedgerCanisterId {
        ledger_canister_id: ledger_canister_id.clone(),
    })?;

    Ok(())
}

impl ModelValidator<AssetError> for Asset {
    fn validate(&self) -> ModelValidatorResult<AssetError> {
        validate_symbol(&self.symbol)?;
        validate_name(&self.name)?;
        validate_decimals(self.decimals)?;
        validate_uniqueness(&self.id, &self.symbol, &self.blockchain)?;
        validate_ledger_canister_id(self)?;

        self.metadata.validate()?;

        Ok(())
    }
}

#[cfg(any(test, feature = "canbench"))]
pub mod asset_test_utils {

    use std::collections::{BTreeMap, BTreeSet};

    use crate::models::{Blockchain, Metadata, TokenStandard};

    use super::Asset;

    pub fn mock_asset() -> Asset {
        Asset {
            id: [0; 16],
            blockchain: Blockchain::InternetComputer,
            standards: BTreeSet::from([TokenStandard::InternetComputerNative]),
            symbol: "ICP".to_string(),
            name: "Internet Computer".to_string(),
            metadata: Metadata::new(BTreeMap::from([
                (
                    "ledger_canister_id".to_string(),
                    "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(),
                ),
                (
                    "index_canister_id".to_string(),
                    "qhbym-qaaaa-aaaaa-aaafq-cai".to_string(),
                ),
            ])),
            decimals: 8,
        }
    }

    pub fn mock_asset_b() -> Asset {
        Asset {
            id: [1; 16],
            blockchain: Blockchain::InternetComputer,
            standards: BTreeSet::from([TokenStandard::InternetComputerNative]),
            symbol: "TEST".to_string(),
            name: "Other Test Asset".to_string(),
            decimals: 8,
            metadata: Metadata::default(),
        }
    }
}

#[cfg(test)]
mod test {

    use orbit_essentials::repository::Repository;
    use std::collections::BTreeMap;

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

    fn with_ledger_canister_id(value: &str) -> Asset {
        let mut asset = asset_test_utils::mock_asset();
        asset
            .metadata
            .change(ChangeMetadata::OverrideSpecifiedBy(BTreeMap::from([(
                TokenStandard::METADATA_KEY_LEDGER_CANISTER_ID.to_string(),
                value.to_string(),
            )])));

        asset
    }

    #[test]
    fn test_validate_ledger_canister_id_accepts_a_principal() {
        let asset = with_ledger_canister_id("ryjl3-tyaaa-aaaaa-aaaba-cai");

        assert!(asset.validate().is_ok());
    }

    #[test]
    fn test_validate_ledger_canister_id_rejects_a_non_principal() {
        let asset = with_ledger_canister_id("not-a-principal");

        assert!(matches!(
            asset
                .validate()
                .expect_err("An unparseable ledger canister id must be rejected"),
            AssetError::InvalidLedgerCanisterId { .. }
        ));
    }

    #[test]
    fn test_validate_allows_an_asset_without_a_ledger_canister_id() {
        let asset = asset_test_utils::mock_asset_b();
        assert!(asset.ledger_canister_id().is_none());

        assert!(asset.validate().is_ok());
    }
}
