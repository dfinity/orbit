use super::BankAsset;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BankFeatures {
    /// The list of assets that are supported by the bank canister (e.g. `ICP`, `BTC`, `ETH`, etc.)
    pub supported_assets: Vec<BankAsset>,
}
