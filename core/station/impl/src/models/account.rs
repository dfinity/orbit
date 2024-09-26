use super::{AccountBalance, AssetId};
use crate::core::validation::{EnsureAsset, EnsureIdExists};
use crate::errors::{AccountError, RecordValidationError};
use crate::models::Metadata;
use crate::repositories::request_policy::REQUEST_POLICY_REPOSITORY;
use candid::{CandidType, Deserialize};
use ic_ledger_types::AccountIdentifier;
use orbit_essentials::model::ModelKey;
use orbit_essentials::repository::Repository;
use orbit_essentials::storable;
use orbit_essentials::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use std::fmt;
use std::str::FromStr;
use std::{collections::HashMap, hash::Hash};

/// The account id, which is a UUID.
pub type AccountId = UUID;
pub type AccountSeed = UUID;

/// Represents a account in the system.
///
/// A account can be associated with one or more users and can only hold one type of asset,
/// which is defined by the blockchain, standard and symbol.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Account {
    /// The account id, which is a UUID.
    pub id: AccountId,
    /// The account name (e.g. `My Main Account`)
    pub name: String,
    /// The seed for address creation.
    pub seed: AccountSeed,
    /// The list of assets this account holds.
    pub assets: Vec<AccountAsset>,
    /// The list of addresses that belong to this account.
    pub addresses: Vec<AccountAddress>,
    /// The account metadata, which is a list of key-value pairs,
    /// where the key is unique and the first entry in the tuple,
    /// and the value is the second entry in the tuple.
    pub metadata: Metadata,
    /// The account transfer policy id, which is a UUID.
    ///
    /// This policy is non exaustive, this means that the account can have other policies that are enforced
    /// by the system that are globally defined.
    pub transfer_request_policy_id: Option<UUID>,
    /// The account update policy id, which is a UUID.
    ///
    /// This policy is non exaustive, this means that the account can have other policies that are enforced
    /// by the system that are globally defined.
    pub configs_request_policy_id: Option<UUID>,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccountKey {
    /// The account id, which is a UUID.
    pub id: AccountId,
}

impl ModelKey<AccountKey> for Account {
    fn key(&self) -> AccountKey {
        AccountKey { id: self.id }
    }
}

// #[storable]
// #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
// pub enum ExtraDataValue {
//     Nat(u128),
//     Blob(Vec<u8>),
// }

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExtraData {
    pub key: String,
    pub text_value: String,
    // pub data: Option<ExtraDataValue>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccountAsset {
    pub asset_id: AssetId,
    pub balance: Option<AccountBalance>,
    pub data: Vec<ExtraData>,
}

// #[storable]
// #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
// pub struct AccountAddressData {
//     pub key: String,
//     pub text_value: String,
// }

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccountAddress {
    pub address: String,
    pub format: AddressFormat,
    pub data: Vec<ExtraData>,
}

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
    pub fn validate_address(&self, address: &str) -> ModelValidatorResult<AccountError> {
        match self {
            AddressFormat::ICPAccountIdentifier => AccountIdentifier::from_hex(address)
                .map_err(|_| AccountError::InvalidAddress {
                    address: address.to_string(),
                    address_format: self.to_string(),
                })
                .map(|_| ()),
            AddressFormat::ICRC1Account => todo!(),
            AddressFormat::EthereumAddress => todo!(),
            AddressFormat::BitcoinAddressP2WPKH => todo!(),
            AddressFormat::BitcoinAddressP2TR => todo!(),
        }
    }
}

impl AccountAddress {
    const ADDRESS_RANGE: (u8, u8) = (1, 255);
    const DATA_KEY_RANGE: (u8, u8) = (1, 255);
    const DATA_TEXT_VALUE_RANGE: (u8, u8) = (1, 255);
}

impl ModelValidator<AccountError> for AccountAddress {
    fn validate(&self) -> ModelValidatorResult<AccountError> {
        if (self.address.len() < AccountAddress::ADDRESS_RANGE.0 as usize)
            || (self.address.len() > AccountAddress::ADDRESS_RANGE.1 as usize)
        {
            return Err(AccountError::InvalidAddressLength {
                min_length: AccountAddress::ADDRESS_RANGE.0,
                max_length: AccountAddress::ADDRESS_RANGE.1,
            });
        }

        self.format.validate_address(&self.address)?;

        for data in &self.data {
            if (data.key.len() < AccountAddress::DATA_KEY_RANGE.0 as usize)
                || (data.key.len() > AccountAddress::DATA_KEY_RANGE.1 as usize)
            {
                return Err(AccountError::ValidationError {
                    info: "Account address data key length must be between 1 and 255".to_string(),
                });
            }

            if (data.text_value.len() < AccountAddress::DATA_TEXT_VALUE_RANGE.0 as usize)
                || (data.text_value.len() > AccountAddress::DATA_TEXT_VALUE_RANGE.1 as usize)
            {
                return Err(AccountError::ValidationError {
                    info: "Account address data text value length must be between 1 and 255"
                        .to_string(),
                });
            }
        }

        Ok(())
    }
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AccountCallerPrivileges {
    pub id: UUID,
    pub can_edit: bool,
    pub can_transfer: bool,
}

fn validate_policy_id(policy_id: &UUID, field_name: &str) -> ModelValidatorResult<AccountError> {
    REQUEST_POLICY_REPOSITORY
        .get(policy_id)
        .ok_or(AccountError::ValidationError {
            info: format!("The {} does not exist", field_name),
        })?;
    Ok(())
}

fn validate_asset_id(asset_id: &AssetId) -> ModelValidatorResult<AccountError> {
    EnsureAsset::id_exists(asset_id).map_err(|err| match err {
        RecordValidationError::NotFound { id, .. } => AccountError::AssetDoesNotExist { id },
    })?;

    Ok(())
}

impl ModelValidator<AccountError> for Account {
    fn validate(&self) -> ModelValidatorResult<AccountError> {
        self.metadata.validate()?;

        for asset in &self.assets {
            validate_asset_id(&asset.asset_id)?;
        }

        for address in &self.addresses {
            address.validate()?;
        }

        if let Some(transfer_request_policy_id) = &self.transfer_request_policy_id {
            validate_policy_id(transfer_request_policy_id, "transfer_request_policy_id")?;
        }
        if let Some(configs_request_policy_id) = &self.configs_request_policy_id {
            validate_policy_id(configs_request_policy_id, "configs_request_policy_id")?;
        }

        Ok(())
    }
}

impl Account {
    pub const OWNERS_RANGE: (u8, u8) = (1, 10);
    pub const ADDRESS_RANGE: (u8, u8) = (1, 255);
    pub const SYMBOL_RANGE: (u8, u8) = (1, 8);
    pub const MAX_POLICIES: u8 = 10;

    /// Creates a new account key from the given key components.
    pub fn key(id: AccountId) -> AccountKey {
        AccountKey { id }
    }

    pub fn to_key(&self) -> AccountKey {
        Self::key(self.id)
    }

    pub fn metadata_map(&self) -> HashMap<String, String> {
        self.metadata.map()
    }
}

#[cfg(test)]
mod tests {
    use super::account_test_utils::mock_account;
    use super::*;

    #[test]
    fn fail_address_length_invalid() {
        let mut account_address: AccountAddress = AccountAddress {
            address: "".to_string(),
            format: AddressFormat::ICPAccountIdentifier,
            data: vec![],
        };

        let result = account_address.validate();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            AccountError::InvalidAddressLength {
                min_length: 1,
                max_length: 255
            }
        );

        account_address.address = "a".repeat(Account::ADDRESS_RANGE.1 as usize + 1);

        let result = account_address.validate();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            AccountError::InvalidAddressLength {
                min_length: 1,
                max_length: 255
            }
        );

        account_address.address = "a".to_string();

        let result = account_address.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn fail_missing_policy_id() {
        let mut account = mock_account();
        account.transfer_request_policy_id = Some([0; 16]);

        let result = account.validate();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            AccountError::ValidationError {
                info: "The transfer_request_policy_id does not exist".to_string()
            }
        );

        account.transfer_request_policy_id = None;
        account.configs_request_policy_id = Some([0; 16]);

        let result = account.validate();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            AccountError::ValidationError {
                info: "The configs_request_policy_id does not exist".to_string()
            }
        );
    }
}

#[cfg(test)]
pub mod account_test_utils {
    use super::*;
    use crate::repositories::ACCOUNT_REPOSITORY;
    use orbit_essentials::repository::Repository;
    use uuid::Uuid;

    pub fn mock_account() -> Account {
        let id = *Uuid::new_v4().as_bytes();

        Account {
            id,
            name: "foo".to_string(),

            seed: id,

            assets: vec![AccountAsset {
                asset_id: [0; 16],
                balance: None,
                data: vec![ExtraData {
                    key: "foo".to_string(),
                    text_value: "bar".to_string(),
                    // data: None,
                }],
            }],

            addresses: vec![AccountAddress {
                address: "foo".to_string(),
                format: AddressFormat::ICPAccountIdentifier,
                data: vec![ExtraData {
                    key: "foo".to_string(),
                    text_value: "bar".to_string(),
                    // data: None,
                }],
            }],

            last_modification_timestamp: 0,
            metadata: Metadata::mock(),
            transfer_request_policy_id: None,
            configs_request_policy_id: None,
        }
    }

    pub fn add_account(id: &UUID) -> Account {
        let mut account = mock_account();
        account.id = id.to_owned();
        ACCOUNT_REPOSITORY.insert(account.to_key(), account.to_owned());

        account
    }
}
