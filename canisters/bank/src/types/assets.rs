use crate::models::{Blockchain, BlockchainStandard};
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BankAsset {
    /// The blockchain identifier (e.g., `ethereum`, `bitcoin`, `icp`, etc.)
    pub blockchain: Blockchain,
    /// The asset symbol (e.g. `ICP`, `BTC`, `ETH`, etc.)
    pub symbol: String,
    /// The asset standard (e.g. `icrc1`, `erc20`, etc.)
    pub standards: Vec<BlockchainStandard>,
    /// The asset name (e.g. `Internet Computer`, `Bitcoin`, `Ethereum`, etc.)
    pub name: String,
    /// The asset decimals (e.g. `8` for `BTC`, `18` for `ETH`, etc.)
    pub decimals: u32,
    /// The asset metadata (e.g. `{"logo": "https://example.com/logo.png"}`).
    pub metadata: HashMap<String, String>,
}

impl Hash for BankAsset {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.blockchain.hash(state);
        self.standards.hash(state);
        self.symbol.hash(state);
        self.name.hash(state);
        self.decimals.hash(state);

        // For HashMap we need to sort the keys first to ensure that the hash is stable.
        let mut keys: Vec<&String> = self.metadata.keys().collect();
        keys.sort();
        keys.hash(state);
    }
}
