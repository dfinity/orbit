use crate::errors::{AccountError, AddressFormatError};
use ic_ledger_types::AccountIdentifier;
use orbit_essentials::model::ModelValidatorResult;
use orbit_essentials::storable;
use std::fmt;
use std::str::FromStr;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AddressFormat {
    ICPAccountIdentifier,
    ICRC1Account,
    EthereumAddress,
    BitcoinAddressP2WPKH,
    BitcoinAddressP2TR,
}

impl fmt::Display for AddressFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddressFormat::ICPAccountIdentifier => write!(f, "icp_account_identifier"),
            AddressFormat::ICRC1Account => write!(f, "icrc1_account"),
            AddressFormat::EthereumAddress => write!(f, "ethereum_address"),
            AddressFormat::BitcoinAddressP2WPKH => write!(f, "bitcoin_address_p2wpkh"),
            AddressFormat::BitcoinAddressP2TR => write!(f, "bitcoin_address_p2tr"),
        }
    }
}

impl FromStr for AddressFormat {
    type Err = AccountError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "icp_account_identifier" => Ok(AddressFormat::ICPAccountIdentifier),
            "icrc1_account" => Ok(AddressFormat::ICRC1Account),
            "ethereum_address" => Ok(AddressFormat::EthereumAddress),
            "bitcoin_address_p2wpkh" => Ok(AddressFormat::BitcoinAddressP2WPKH),
            "bitcoin_address_p2tr" => Ok(AddressFormat::BitcoinAddressP2TR),
            _ => Err(AccountError::UnknownAddressFormat {
                address_format: s.to_string(),
            }),
        }
    }
}

impl AddressFormat {
    pub fn validate_address(&self, address: &str) -> ModelValidatorResult<AddressFormatError> {
        match self {
            AddressFormat::ICPAccountIdentifier => AccountIdentifier::from_hex(address)
                .map_err(|_| AddressFormatError::InvalidAddressFormat {
                    address: address.to_string(),
                    address_format: self.to_string(),
                })
                .map(|_| ()),
            AddressFormat::ICRC1Account => {
                icrc_ledger_types::icrc1::account::Account::from_str(address)
                    .map_err(|_| AddressFormatError::InvalidAddressFormat {
                        address: address.to_string(),
                        address_format: self.to_string(),
                    })
                    .map(|_| ())
            }
            AddressFormat::EthereumAddress => todo!(),
            AddressFormat::BitcoinAddressP2WPKH => todo!(),
            AddressFormat::BitcoinAddressP2TR => todo!(),
        }
    }
}

#[cfg(test)]
pub mod address_format_test_utils {

    pub const VALID_ACCOUNT_IDENTIFIER: &str =
        "5c76bc95e544204de4928e4d901e52b49df248b9c346807040e7af75aa61f4b3";

    pub const VALID_ICRC1_ADDRESS: &str = "wmzac-nabae-aqcai-baeaq-caiba-eaqca-ibaea-qcaib-aeaqc-aibae-aqc-haltvua.102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";
}

#[cfg(test)]
mod tests {
    use super::{
        address_format_test_utils::{VALID_ACCOUNT_IDENTIFIER, VALID_ICRC1_ADDRESS},
        *,
    };

    #[test]
    fn fail_address_format_invalid() {
        let format = AddressFormat::ICPAccountIdentifier;

        format
            .validate_address("foo")
            .expect_err("foo is not a valid AccountIdentifier");

        format
            .validate_address(VALID_ACCOUNT_IDENTIFIER)
            .expect("The address is valid");

        let format = AddressFormat::ICRC1Account;
        format
            .validate_address(VALID_ICRC1_ADDRESS)
            .expect("The address is valid");
    }
}
