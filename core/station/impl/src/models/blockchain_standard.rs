use candid::CandidType;
use ic_canister_macros::storable;
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
