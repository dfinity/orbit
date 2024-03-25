use super::{AccountBalance, Blockchain, BlockchainStandard};
use crate::errors::AccountError;
use crate::models::Metadata;
use candid::{CandidType, Deserialize};
use ic_canister_core::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use ic_canister_macros::storable;
use std::{collections::HashMap, hash::Hash};
use uuid::Uuid;

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
    pub transfer_approval_policy_id: UUID,
    /// The account update policy id, which is a UUID.
    ///
    /// This policy is non exaustive, this means that the account can have other policies that are enforced
    /// by the system that are globally defined.
    pub update_approval_policy_id: UUID,
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

fn validate_transfer_approval_policy(policy_id: &UUID) -> ModelValidatorResult<AccountError> {
    if *policy_id == *Uuid::nil().as_bytes() {
        return Err(AccountError::ValidationError {
            info: "Transfer approval policy id cannot be nil".to_string(),
        });
    }

    Ok(())
}

fn validate_update_approval_policy(policy_id: &UUID) -> ModelValidatorResult<AccountError> {
    if *policy_id == *Uuid::nil().as_bytes() {
        return Err(AccountError::ValidationError {
            info: "Update approval policy id cannot be nil".to_string(),
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

impl ModelValidator<AccountError> for Account {
    fn validate(&self) -> ModelValidatorResult<AccountError> {
        self.metadata.validate()?;
        validate_symbol(&self.symbol)?;
        validate_address(&self.address)?;
        validate_transfer_approval_policy(&self.transfer_approval_policy_id)?;
        validate_update_approval_policy(&self.update_approval_policy_id)?;

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
    fn fail_transfer_approval_policy_validation() {
        let mut account = mock_account();
        account.transfer_approval_policy_id = *Uuid::nil().as_bytes();

        let result = validate_transfer_approval_policy(&account.transfer_approval_policy_id);

        assert!(result.is_err());
    }

    #[test]
    fn test_transfer_approval_policy_validation() {
        let mut account = mock_account();
        account.transfer_approval_policy_id = *Uuid::new_v4().as_bytes();

        let result = validate_transfer_approval_policy(&account.transfer_approval_policy_id);

        assert!(result.is_ok());
    }

    #[test]
    fn fail_update_approval_policy_validation() {
        let mut account = mock_account();
        account.update_approval_policy_id = *Uuid::nil().as_bytes();

        let result = validate_update_approval_policy(&account.update_approval_policy_id);

        assert!(result.is_err());
    }

    #[test]
    fn test_update_approval_policy_validation() {
        let mut account = mock_account();
        account.update_approval_policy_id = *Uuid::new_v4().as_bytes();

        let result = validate_update_approval_policy(&account.update_approval_policy_id);

        assert!(result.is_ok());
    }
}

#[cfg(test)]
pub mod account_test_utils {
    use super::*;
    use crate::repositories::ACCOUNT_REPOSITORY;
    use ic_canister_core::repository::Repository;
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
            transfer_approval_policy_id: *Uuid::new_v4().as_bytes(),
            update_approval_policy_id: *Uuid::new_v4().as_bytes(),
        }
    }

    pub fn add_account(id: &UUID) -> Account {
        let mut account = mock_account();
        account.id = id.to_owned();
        ACCOUNT_REPOSITORY.insert(account.to_key(), account.to_owned());

        account
    }
}
