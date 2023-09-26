use candid::{CandidType, Deserialize};
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct BankAssetDTO {
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
    /// The asset decimals (e.g. `8` for `BTC`, `18` for `ETH`, etc.)
    pub decimals: u8,
    /// The asset metadata (e.g. `{"logo": "https://example.com/logo.png"}`),
    /// also, in the case of non-native assets, it can contain other required
    /// information (e.g. `{"address": "0x1234"}`).
    pub metadata: HashMap<String, String>,
}

/// Bank details data transfer object (DTO).
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct BankDetailsDTO {
    /// The list of assets that are supported by the bank canister (e.g. `ICP`, `BTC`, `ETH`, etc.)
    pub supported_assets: Vec<BankAssetDTO>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct BankDetailsResponse {
    pub details: BankDetailsDTO,
}
