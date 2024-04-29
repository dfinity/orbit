use crate::MetadataDTO;
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AssetDTO {
    /// The blockchain identifier (e.g., `ethereum`, `bitcoin`, `icp`, etc.)
    pub blockchain: String,
    /// The asset symbol (e.g. `ICP`, `BTC`, `ETH`, etc.)
    pub symbol: String,
    // The asset standard that is supported (e.g. `erc20`, etc.), canonically
    // represented as a lowercase string with spaces replaced with underscores.
    pub standard: String,
    /// The asset name (e.g. `Internet Computer`, `Bitcoin`, `Ethereum`, etc.)
    pub name: String,
    /// The asset metadata (e.g. `{"logo": "https://example.com/logo.png"}`),
    /// also, in the case of non-native assets, it can contain other required
    /// information (e.g. `{"address": "0x1234"}`).
    pub metadata: Vec<MetadataDTO>,
}

/// The capabilities of the canister.
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CapabilitiesDTO {
    /// The name of the canister.
    pub name: String,
    /// The current version of the canister.
    pub version: String,
    /// The list of assets that are supported by the canister (e.g. `ICP`, `BTC`, `ETH`, etc.)
    pub supported_assets: Vec<AssetDTO>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CapabilitiesResponse {
    pub capabilities: CapabilitiesDTO,
}
