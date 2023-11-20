use super::WalletAsset;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct WalletFeatures {
    /// The list of assets that are supported by the wallet canister (e.g. `ICP`, `BTC`, `ETH`, etc.)
    pub supported_assets: Vec<WalletAsset>,
}
