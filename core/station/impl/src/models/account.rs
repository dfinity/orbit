use super::{AccountBalance, AssetId};
use crate::core::validation::{EnsureAsset, EnsureIdExists};
use crate::core::ACCOUNT_BALANCE_FRESHNESS_IN_MS;
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
#[storable(skip_deserialize = true)]
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

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccountAsset {
    pub asset_id: AssetId,
    pub balance: Option<AccountBalance>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccountAddress {
    pub address: String,
    pub format: AddressFormat,
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
            AddressFormat::ICRC1Account => {
                icrc_ledger_types::icrc1::account::Account::from_str(address)
                    .map_err(|_| AccountError::InvalidAddress {
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

impl AccountAddress {
    const ADDRESS_RANGE: (u8, u8) = (1, 255);
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

fn validate_account_name(name: &str) -> ModelValidatorResult<AccountError> {
    if (name.len() < Account::NAME_RANGE.0 as usize)
        || (name.len() > Account::NAME_RANGE.1 as usize)
    {
        return Err(AccountError::InvalidNameLength {
            min_length: Account::NAME_RANGE.0,
            max_length: Account::NAME_RANGE.1,
        });
    }

    Ok(())
}

impl ModelValidator<AccountError> for Account {
    fn validate(&self) -> ModelValidatorResult<AccountError> {
        self.metadata.validate()?;

        validate_account_name(&self.name)?;

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
    pub const NAME_RANGE: (u8, u8) = (1, 64);
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

pub enum BalanceQueryState {
    StaleRefreshing,
    Stale,
    Fresh,
}

impl fmt::Display for BalanceQueryState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BalanceQueryState::StaleRefreshing => write!(f, "stale_refreshing"),
            BalanceQueryState::Stale => write!(f, "stale"),
            BalanceQueryState::Fresh => write!(f, "fresh"),
        }
    }
}

impl From<&AccountBalance> for BalanceQueryState {
    fn from(balance: &AccountBalance) -> Self {
        let balance_age_ms = crate::core::ic_cdk::api::time()
            .saturating_sub(balance.last_modification_timestamp)
            / 1_000_000;
        if balance_age_ms <= ACCOUNT_BALANCE_FRESHNESS_IN_MS {
            BalanceQueryState::Fresh
        } else {
            BalanceQueryState::Stale
        }
    }
}

#[cfg(test)]
mod tests {
    use super::account_test_utils::mock_account;
    use super::*;

    const VALID_ACCOUNT_IDENTIFIER: &str =
        "5c76bc95e544204de4928e4d901e52b49df248b9c346807040e7af75aa61f4b3";

    #[test]
    fn fail_address_format_invalid() {
        let format = AddressFormat::ICPAccountIdentifier;

        format
            .validate_address("foo")
            .expect_err("foo is not a valid AccountIdentifier");

        format
            .validate_address(VALID_ACCOUNT_IDENTIFIER)
            .expect("The address is valid");
    }
    #[test]
    fn fail_address_length_invalid() {
        let mut account_address: AccountAddress = AccountAddress {
            address: "".to_string(),
            format: AddressFormat::ICPAccountIdentifier,
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
    use candid::Principal;
    use ic_ledger_types::Subaccount;
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
            }],

            addresses: vec![AccountAddress {
                address: AccountIdentifier::new(&Principal::anonymous(), &Subaccount([0; 32]))
                    .to_hex(),
                format: AddressFormat::ICPAccountIdentifier,
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
