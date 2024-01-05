use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AssetMetadataDTO {
    pub name: String,
    pub key: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct WalletAssetDTO {
    /// The blockchain identifier (e.g., `ethereum`, `bitcoin`, `icp`, etc.)
    pub blockchain: String,
    /// The asset symbol (e.g. `ICP`, `BTC`, `ETH`, etc.)
    pub symbol: String,
    // The asset standard that is supported (e.g. `erc20`, etc.), canonically
    // represented as a lowercase string with spaces replaced with underscores.
    //
    // If empty, then only the native blockchain asset is supported.
    pub standards: Vec<String>,
    /// The asset name (e.g. `Internet Computer`, `Bitcoin`, `Ethereum`, etc.)
    pub name: String,
    /// The asset metadata (e.g. `{"logo": "https://example.com/logo.png"}`),
    /// also, in the case of non-native assets, it can contain other required
    /// information (e.g. `{"address": "0x1234"}`).
    pub metadata: Vec<AssetMetadataDTO>,
}

/// Wallet features data transfer object (DTO).
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct WalletFeaturesDTO {
    /// The list of assets that are supported by the wallet canister (e.g. `ICP`, `BTC`, `ETH`, etc.)
    pub supported_assets: Vec<WalletAssetDTO>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct WalletFeaturesResponse {
    pub features: WalletFeaturesDTO,
}
