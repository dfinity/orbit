use super::{Blockchain, BlockchainStandard};
use crate::models::Metadata;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Asset {
    /// The blockchain identifier (e.g., `ethereum`, `bitcoin`, `icp`, etc.)
    pub blockchain: Blockchain,
    // The asset standard that is supported (e.g. `erc20`, `native`, etc.), canonically
    // represented as a lowercase string with spaces replaced with underscores.
    pub standard: BlockchainStandard,
    /// The asset symbol (e.g. `ICP`, `BTC`, `ETH`, etc.)
    pub symbol: String,
    /// The asset name (e.g. `Internet Computer`, `Bitcoin`, `Ethereum`, etc.)
    pub name: String,
    /// The asset metadata (e.g. `{"logo": "https://example.com/logo.png"}`),
    /// also, in the case of non-native assets, it can contain other required
    /// information (e.g. `{"address": "0x1234"}`).
    pub metadata: Metadata,
}

impl Hash for Asset {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.blockchain.hash(state);
        self.standard.hash(state);
        self.symbol.hash(state);
        self.name.hash(state);

        // For HashMap we need to sort the keys first to ensure that the hash is stable.
        let mut keys: Vec<&String> = self.metadata.keys();
        keys.sort();
        keys.hash(state);
    }
}
