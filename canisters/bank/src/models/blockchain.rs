use candid::{CandidType, Deserialize};
use ic_stable_structures::{BoundedStorable, Storable};
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Blockchain {
    InternetComputer,
    Ethereum,
    Bitcoin,
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

impl Storable for Blockchain {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(self.to_string().as_bytes().to_vec())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let chain_id: String = String::from_bytes(bytes);
        Self::from_str(&chain_id).unwrap()
    }
}

impl BoundedStorable for Blockchain {
    const MAX_SIZE: u32 = 8;

    const IS_FIXED_SIZE: bool = false;
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
}
