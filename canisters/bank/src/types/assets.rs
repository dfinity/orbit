use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    hash::{Hash, Hasher},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AssetStandard {
    ICRC1,
    ERC20,
}

impl Display for AssetStandard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetStandard::ERC20 => write!(f, "erc20"),
            AssetStandard::ICRC1 => write!(f, "icrc1"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Blockchain {
    InternetComputer,
    Ethereum,
    Bitcoin,
}

impl Display for Blockchain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Blockchain::InternetComputer => write!(f, "icp"),
            Blockchain::Ethereum => write!(f, "eth"),
            Blockchain::Bitcoin => write!(f, "btc"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BankAsset {
    /// The blockchain identifier (e.g., `ethereum`, `bitcoin`, `icp`, etc.)
    pub blockchain: Blockchain,
    /// The asset symbol (e.g. `ICP`, `BTC`, `ETH`, etc.)
    pub symbol: String,
    /// The asset standard (e.g. `icrc1`, `erc20`, etc.)
    pub standards: Vec<AssetStandard>,
    /// The asset name (e.g. `Internet Computer`, `Bitcoin`, `Ethereum`, etc.)
    pub name: String,
    /// The asset decimals (e.g. `8` for `BTC`, `18` for `ETH`, etc.)
    pub decimals: u8,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asset_standards_match_string_representation() {
        assert_eq!(AssetStandard::ERC20.to_string(), "erc20");
        assert_eq!(AssetStandard::ICRC1.to_string(), "icrc1");
    }

    #[test]
    fn blockchain_match_string_representation() {
        assert_eq!(Blockchain::InternetComputer.to_string(), "icp");
        assert_eq!(Blockchain::Ethereum.to_string(), "eth");
        assert_eq!(Blockchain::Bitcoin.to_string(), "btc");
    }
}
