use super::TokenStandard;
use candid::CandidType;
use orbit_essentials::storable;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[storable]
#[derive(CandidType, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Blockchain {
    InternetComputer,
    Ethereum,
    Bitcoin,
}

impl Blockchain {
    /// The native symbol of the blockchain.
    pub fn native_symbol(&self) -> &'static str {
        match self {
            Blockchain::InternetComputer => "ICP",
            Blockchain::Ethereum => "ETH",
            Blockchain::Bitcoin => "BTC",
        }
    }

    /// The list of standards that the blockchain supports.
    pub fn supported_standards(&self) -> Vec<TokenStandard> {
        match self {
            Blockchain::InternetComputer => {
                vec![TokenStandard::InternetComputerNative, TokenStandard::ICRC1]
            }
            Blockchain::Ethereum => vec![],
            Blockchain::Bitcoin => vec![],
        }
    }
}

impl FromStr for Blockchain {
    type Err = ();

    fn from_str(variant: &str) -> Result<Blockchain, Self::Err> {
        match variant {
            "icp" => Ok(Blockchain::InternetComputer),
            "eth" => Ok(Blockchain::Ethereum),
            "btc" => Ok(Blockchain::Bitcoin),
            _ => Err(()),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blockchain_match_string_representation() {
        assert_eq!(Blockchain::InternetComputer.to_string(), "icp");
        assert_eq!(
            Blockchain::from_str("icp").unwrap(),
            Blockchain::InternetComputer
        );
        assert_eq!(Blockchain::Ethereum.to_string(), "eth");
        assert_eq!(Blockchain::from_str("eth").unwrap(), Blockchain::Ethereum);
        assert_eq!(Blockchain::Bitcoin.to_string(), "btc");
        assert_eq!(Blockchain::from_str("btc").unwrap(), Blockchain::Bitcoin);
    }

    #[test]
    fn match_native_symbols_successfully() {
        assert_eq!(Blockchain::InternetComputer.native_symbol(), "ICP");
        assert_eq!(Blockchain::Ethereum.native_symbol(), "ETH");
        assert_eq!(Blockchain::Bitcoin.native_symbol(), "BTC");
    }

    #[test]
    fn match_supported_standards() {
        assert!(Blockchain::InternetComputer
            .supported_standards()
            .contains(&TokenStandard::InternetComputerNative));
        assert!(Blockchain::InternetComputer
            .supported_standards()
            .contains(&TokenStandard::ICRC1));
    }
}
