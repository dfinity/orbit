use super::{AccountBalance, Blockchain, BlockchainStandard};
use crate::errors::AccountError;
use crate::models::Metadata;
use crate::repositories::policy::PROPOSAL_POLICY_REPOSITORY;
use candid::{CandidType, Deserialize};
use orbit_essentials::repository::Repository;
use orbit_essentials::storable;
use orbit_essentials::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use std::{collections::HashMap, hash::Hash};

/// The account metadata key for the asset symbol;
pub const ACCOUNT_METADATA_SYMBOL_KEY: &str = "symbol";

/// The account id, which is a UUID.
pub type AccountId = UUID;

/// Represents a account in the system.
///
/// A account can be associated with one or more users and can only hold one type of asset,
/// which is defined by the blockchain, standard and symbol.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Account {
    /// The account id, which is a UUID.
    pub id: AccountId,
    /// The blockchain type (e.g. `icp`, `eth`, `btc`)
    pub blockchain: Blockchain,
    /// The account address (e.g. `0x1234`, etc.)
    pub address: String,
    /// The blockchain standard (e.g. `native`, `icrc1`, `erc20`, etc.)
    pub standard: BlockchainStandard,
    /// The asset symbol (e.g. `ICP`, `ETH`, `BTC`, etc.)
    pub symbol: String,
    /// The asset decimals (e.g. `8` for `BTC`, `18` for `ETH`, etc.)
    pub decimals: u32,
    /// The account name (e.g. `My Main Account`)
    pub name: String,
    /// The account balance, which is the amount of the asset that the account holds.
    pub balance: Option<AccountBalance>,
    /// The account metadata, which is a list of key-value pairs,
    /// where the key is unique and the first entry in the tuple,
    /// and the value is the second entry in the tuple.
    pub metadata: Metadata,
    /// The account transfer policy id, which is a UUID.
    ///
    /// This policy is non exaustive, this means that the account can have other policies that are enforced
    /// by the system that are globally defined.
    pub transfer_approval_policy_id: Option<UUID>,
    /// The account update policy id, which is a UUID.
    ///
    /// This policy is non exaustive, this means that the account can have other policies that are enforced
    /// by the system that are globally defined.
    pub update_approval_policy_id: Option<UUID>,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccountKey {
    /// The account id, which is a UUID.
    pub id: AccountId,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AccountCallerPrivileges {
    pub id: UUID,
    pub can_edit: bool,
    pub can_transfer: bool,
}

fn validate_symbol(symbol: &str) -> ModelValidatorResult<AccountError> {
    if (symbol.len() < Account::SYMBOL_RANGE.0 as usize)
        || (symbol.len() > Account::SYMBOL_RANGE.1 as usize)
    {
        return Err(AccountError::ValidationError {
            info: format!(
                "Account symbol length must be between {} and {}",
                Account::SYMBOL_RANGE.0,
                Account::SYMBOL_RANGE.1
            ),
        });
    }

    Ok(())
}

fn validate_address(address: &str) -> ModelValidatorResult<AccountError> {
    if (address.len() < Account::ADDRESS_RANGE.0 as usize)
        || (address.len() > Account::ADDRESS_RANGE.1 as usize)
    {
        return Err(AccountError::InvalidAddressLength {
            min_length: Account::ADDRESS_RANGE.0,
            max_length: Account::ADDRESS_RANGE.1,
        });
    }

    Ok(())
}

fn validate_policy_id(policy_id: &UUID, field_name: &str) -> ModelValidatorResult<AccountError> {
    PROPOSAL_POLICY_REPOSITORY
        .get(policy_id)
        .ok_or(AccountError::ValidationError {
            info: format!("The {} does not exist", field_name),
        })?;
    Ok(())
}

impl ModelValidator<AccountError> for Account {
    fn validate(&self) -> ModelValidatorResult<AccountError> {
        self.metadata.validate()?;
        validate_symbol(&self.symbol)?;
        validate_address(&self.address)?;

        if let Some(transfer_approval_policy_id) = &self.transfer_approval_policy_id {
            validate_policy_id(transfer_approval_policy_id, "transfer_approval_policy_id")?;
        }
        if let Some(update_approval_policy_id) = &self.update_approval_policy_id {
            validate_policy_id(update_approval_policy_id, "update_approval_policy_id")?;
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
    fn fail_symbol_validation_too_short() {
        let mut account = mock_account();
        account.symbol = "a".repeat(0);

        let result = validate_symbol(&account.symbol);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            AccountError::ValidationError {
                info: "Account symbol length must be between 1 and 8".to_string()
            }
        );
    }

    #[test]
    fn fail_symbol_validation_too_long() {
        let mut account = mock_account();
        account.symbol = "a".repeat(Account::SYMBOL_RANGE.1 as usize + 1);

        let result = validate_symbol(&account.symbol);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            AccountError::ValidationError {
                info: "Account symbol length must be between 1 and 8".to_string()
            }
        );
    }

    #[test]
    fn test_symbol_validation() {
        let mut account = mock_account();
        account.symbol = "a".to_string();

        let result = validate_symbol(&account.symbol);

        assert!(result.is_ok());
    }

    #[test]
    fn fail_address_too_short() {
        let mut account = mock_account();
        account.address = "".to_string();

        let result = validate_address(&account.address);

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
    fn fail_address_too_long() {
        let mut account = mock_account();
        account.address = "a".repeat(Account::ADDRESS_RANGE.1 as usize + 1);

        let result = validate_address(&account.address);

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
    fn test_address_validation() {
        let mut account = mock_account();
        account.address = "a".to_string();

        let result = validate_address(&account.address);

        assert!(result.is_ok());
    }

    #[test]
    fn fail_missing_policy_id() {
        let mut account = mock_account();
        account.transfer_approval_policy_id = Some([0; 16]);

        let result = account.validate();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            AccountError::ValidationError {
                info: "The transfer_approval_policy_id does not exist".to_string()
            }
        );

        account.transfer_approval_policy_id = None;
        account.update_approval_policy_id = Some([0; 16]);

        let result = account.validate();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            AccountError::ValidationError {
                info: "The update_approval_policy_id does not exist".to_string()
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
        Account {
            id: *Uuid::new_v4().as_bytes(),
            address: "0x1234".to_string(),
            balance: None,
            blockchain: Blockchain::InternetComputer,
            decimals: 0u32,
            name: "foo".to_string(),
            standard: BlockchainStandard::Native,
            last_modification_timestamp: 0,
            metadata: Metadata::mock(),
            symbol: "ICP".to_string(),
            transfer_approval_policy_id: None,
            update_approval_policy_id: None,
        }
    }

    pub fn add_account(id: &UUID) -> Account {
        let mut account = mock_account();
        account.id = id.to_owned();
        ACCOUNT_REPOSITORY.insert(account.to_key(), account.to_owned());

        account
    }
}
