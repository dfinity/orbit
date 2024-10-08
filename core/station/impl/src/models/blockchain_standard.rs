use orbit_essentials::storable;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use super::AddressFormat;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TokenStandardInfo {
    pub name: String,
    pub address_formats: Vec<AddressFormat>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TokenStandard {
    InternetComputerNative,
    ICRC1,
    // ERC20,
}

impl TokenStandard {
    pub fn get_info(&self) -> TokenStandardInfo {
        match self {
            TokenStandard::InternetComputerNative => TokenStandardInfo {
                name: "icp_native".to_owned(),
                address_formats: vec![AddressFormat::ICPAccountIdentifier],
            },
            TokenStandard::ICRC1 => TokenStandardInfo {
                name: "icrc1".to_owned(),
                address_formats: vec![AddressFormat::ICRC1Account],
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

impl TokenStandard {
    pub const METADATA_KEY_LEDGER_CANISTER_ID: &'static str = "ledger_canister_id";
    pub const METADATA_KEY_INDEX_CANISTER_ID: &'static str = "index_canister_id";

    pub fn get_required_metadata(&self) -> Vec<String> {
        match self {
            TokenStandard::ICRC1 | TokenStandard::InternetComputerNative => vec![
                Self::METADATA_KEY_LEDGER_CANISTER_ID.to_string(),
                Self::METADATA_KEY_INDEX_CANISTER_ID.to_string(),
            ],
        }
    }

    pub fn get_supported_operations(&self) -> Vec<StandardOperation> {
        match self {
            TokenStandard::InternetComputerNative | TokenStandard::ICRC1 => vec![
                StandardOperation::Balance,
                StandardOperation::Transfer,
                StandardOperation::ListTransfers,
            ],
        }
    }
}

impl FromStr for TokenStandard {
    type Err = ();

    fn from_str(variant: &str) -> Result<TokenStandard, Self::Err> {
        match variant {
            "icp_native" => Ok(TokenStandard::InternetComputerNative),
            "icrc1" => Ok(TokenStandard::ICRC1),
            _ => Err(()),
        }
    }
}

impl Display for TokenStandard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenStandard::InternetComputerNative => write!(f, "icp_native"),
            TokenStandard::ICRC1 => write!(f, "icrc1"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blockchain_standard_match_string_representation() {
        assert_eq!(
            TokenStandard::InternetComputerNative.to_string(),
            "icp_native"
        );
        assert_eq!(
            TokenStandard::from_str("icp_native").unwrap(),
            TokenStandard::InternetComputerNative
        );
        assert_eq!(TokenStandard::ICRC1.to_string(), "icrc1");
        assert_eq!(
            TokenStandard::from_str("icrc1").unwrap(),
            TokenStandard::ICRC1
        );
    }
}
