use super::{Blockchain, BlockchainStandard};
use std::hash::{Hash, Hasher};
use wallet_api::AssetMetadataDTO;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WalletAsset {
    /// The blockchain identifier (e.g., `ethereum`, `bitcoin`, `icp`, etc.)
    pub blockchain: Blockchain,
    /// The asset symbol (e.g. `ICP`, `BTC`, `ETH`, etc.)
    pub symbol: String,
    // The asset standard that is supported (e.g. `erc20`, `native`, etc.), canonically
    // represented as a lowercase string with spaces replaced with underscores.
    //
    // If empty, then only the native blockchain asset is supported.
    pub standards: Vec<BlockchainStandard>,
    /// The asset name (e.g. `Internet Computer`, `Bitcoin`, `Ethereum`, etc.)
    pub name: String,
    /// The asset metadata (e.g. `{"logo": "https://example.com/logo.png"}`),
    /// also, in the case of non-native assets, it can contain other required
    /// information (e.g. `{"address": "0x1234"}`).
    pub metadata: Vec<AssetMetadataDTO>,
}

impl Hash for WalletAsset {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.blockchain.hash(state);
        self.standards.hash(state);
        self.symbol.hash(state);
        self.name.hash(state);

        // For HashMap we need to sort the keys first to ensure that the hash is stable.
        let mut keys: Vec<&String> = self.metadata.iter().map(|kv| &kv.key).collect();
        keys.sort();
        keys.hash(state);
    }
}
