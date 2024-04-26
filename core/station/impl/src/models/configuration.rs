use super::Asset;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Configuration {
    /// The list of assets that are supported by the canister (e.g. `ICP`, `BTC`, `ETH`, etc.)
    pub supported_assets: Vec<Asset>,
}
