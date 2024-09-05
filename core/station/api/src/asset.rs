use candid::CandidType;
use serde::Deserialize;

use crate::{ChangeMetadataDTO, MetadataDTO};

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct AssetDTO {
    /// The blockchain identifier (e.g., `ethereum`, `bitcoin`, `icp`, etc.)
    pub blockchain: String,
    /// The asset symbol (e.g. `ICP`, `BTC`, `ETH`, etc.)
    pub symbol: String,
    /// The number of decimal places that the asset supports (e.g. `8` for `BTC`, `18` for `ETH`, etc.)
    pub decimals: u32,
    // The asset standard that is supported (e.g. `erc20`, etc.), canonically
    // represented as a lowercase string with spaces replaced with underscores.
    pub standards: Vec<String>,
    /// The asset name (e.g. `Internet Computer`, `Bitcoin`, `Ethereum`, etc.)
    pub name: String,
    /// The asset metadata (e.g. `{"logo": "https://example.com/logo.png"}`),
    /// also, in the case of non-native assets, it can contain other required
    /// information (e.g. `{"address": "0x1234"}`).
    pub metadata: Vec<MetadataDTO>,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct AddAssetOperationDTO {
    pub asset: Option<AssetDTO>,
    pub input: AddAssetOperationInput,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct AddAssetOperationInput {
    pub name: String,
    pub blockchain: String,
    pub standards: Vec<String>,
    pub symbol: String,
    pub decimals: u32,
    pub metadata: Vec<MetadataDTO>,
}
#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct EditAssetOperationDTO {
    pub input: EditAssetOperationInput,
}

#[derive(CandidType, serde::Serialize, Deserialize, Clone, Debug)]
pub struct EditAssetOperationInput {
    pub asset_id: String,
    pub name: Option<String>,
    pub blockchain: Option<String>,
    pub standards: Option<Vec<String>>,
    pub symbol: Option<String>,
    pub decimals: Option<u32>,
    pub change_metadata: Option<ChangeMetadataDTO>,
}
