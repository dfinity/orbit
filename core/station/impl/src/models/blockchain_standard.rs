use candid::CandidType;
use orbit_essentials::storable;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[storable]
#[derive(CandidType, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BlockchainStandard {
    Native,
    ICRC1,
    ERC20,
}

pub enum StandardOperation {
    Balance,
    Transfer,
    ListTransfers,
}
impl std::fmt::Display for StandardOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StandardOperation::Balance => write!(f, "balance"),
            StandardOperation::Transfer => write!(f, "transfer"),
            StandardOperation::ListTransfers => write!(f, "list_transfers"),
        }
    }
}

impl BlockchainStandard {
    pub fn get_required_metadata(&self) -> Vec<String> {
        match self {
            BlockchainStandard::Native => vec![
                "ledger_canister_id".to_string(),
                "index_canister_id".to_string(),
            ],
            BlockchainStandard::ICRC1 => vec![
                "ledger_canister_id".to_string(),
                "index_canister_id".to_string(),
            ],
            BlockchainStandard::ERC20 => vec!["contract_address".to_string()],
        }
    }

    pub fn get_supported_operations(&self) -> Vec<StandardOperation> {
        match self {
            BlockchainStandard::Native => vec![
                StandardOperation::Balance,
                StandardOperation::Transfer,
                StandardOperation::ListTransfers,
            ],
            BlockchainStandard::ICRC1 => vec![],
            BlockchainStandard::ERC20 => vec![],
        }
    }
}

impl FromStr for BlockchainStandard {
    type Err = ();

    fn from_str(variant: &str) -> Result<BlockchainStandard, Self::Err> {
        match variant {
            "native" => Ok(BlockchainStandard::Native),
            "icrc1" => Ok(BlockchainStandard::ICRC1),
            "erc20" => Ok(BlockchainStandard::ERC20),
            _ => Err(()),
        }
    }
}

impl Display for BlockchainStandard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockchainStandard::Native => write!(f, "native"),
            BlockchainStandard::ERC20 => write!(f, "erc20"),
            BlockchainStandard::ICRC1 => write!(f, "icrc1"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blockchain_standard_match_string_representation() {
        assert_eq!(BlockchainStandard::Native.to_string(), "native");
        assert_eq!(
            BlockchainStandard::from_str("native").unwrap(),
            BlockchainStandard::Native
        );
        assert_eq!(BlockchainStandard::ICRC1.to_string(), "icrc1");
        assert_eq!(
            BlockchainStandard::from_str("icrc1").unwrap(),
            BlockchainStandard::ICRC1
        );
        assert_eq!(BlockchainStandard::ERC20.to_string(), "erc20");
        assert_eq!(
            BlockchainStandard::from_str("erc20").unwrap(),
            BlockchainStandard::ERC20
        );
    }
}
