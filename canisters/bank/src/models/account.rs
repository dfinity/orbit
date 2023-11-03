use super::{AccountBalance, AccountPolicy, Blockchain, BlockchainStandard, UserId};
use crate::errors::AccountError;
use candid::{CandidType, Deserialize};
use ic_canister_core::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use ic_canister_macros::stable_object;
use std::{collections::HashMap, hash::Hash};

/// The account metadata key for the asset symbol;
pub const ACCOUNT_METADATA_SYMBOL_KEY: &str = "symbol";

/// The account id, which is a UUID.
pub type AccountId = UUID;

/// Represents a account in the system.
///
/// A account can be associated with one or more users and can only hold one type of asset,
/// which is defined by the blockchain, standard and symbol.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    pub name: Option<String>,
    /// The account owners, which are a list of user ids.
    ///
    /// If the account has no owners, it means that it is a system account and
    /// only admins of the system can operate on it.
    pub owners: Vec<UserId>,
    /// The account balance, which is the amount of the asset that the account holds.
    pub balance: Option<AccountBalance>,
    /// The account policies, which define the rules for the account.
    pub policies: Vec<AccountPolicy>,
    /// The account metadata, which is a list of key-value pairs,
    /// where the key is unique and the first entry in the tuple,
    /// and the value is the second entry in the tuple.
    pub metadata: Vec<(String, String)>,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccountKey {
    /// The account id, which is a UUID.
    pub id: AccountId,
}

pub struct AccountValidator<'model> {
    account: &'model Account,
}

impl<'model> AccountValidator<'model> {
    pub const OWNERS_RANGE: (u8, u8) = (1, 10);
    pub const ADDRESS_RANGE: (u8, u8) = (1, 255);
    pub const SYMBOL_RANGE: (u8, u8) = (1, 8);
    pub const MAX_POLICIES: u8 = 10;
    pub const MAX_METADATA: u8 = 10;
    pub const MAX_METADATA_KEY_LEN: u8 = 24;
    pub const MAX_METADATA_VALUE_LEN: u8 = 255;

    pub fn new(account: &'model Account) -> AccountValidator {
        AccountValidator { account }
    }

    pub fn validate_policies(&self) -> ModelValidatorResult<AccountError> {
        if self.account.policies.len() > Self::MAX_POLICIES as usize {
            return Err(AccountError::ValidationError {
                info: format!(
                    "Account policies count exceeds the maximum allowed: {}",
                    Self::MAX_POLICIES
                ),
            });
        }

        Ok(())
    }

    pub fn validate_metadata(&self) -> ModelValidatorResult<AccountError> {
        if self.account.metadata.len() > Self::MAX_METADATA as usize {
            return Err(AccountError::ValidationError {
                info: format!(
                    "Account metadata count exceeds the maximum allowed: {}",
                    Self::MAX_METADATA
                ),
            });
        }

        for (key, value) in self.account.metadata.iter() {
            if key.len() > Self::MAX_METADATA_KEY_LEN as usize {
                return Err(AccountError::ValidationError {
                    info: format!(
                        "Account metadata key length exceeds the maximum allowed: {}",
                        Self::MAX_METADATA_KEY_LEN
                    ),
                });
            }

            if value.len() > Self::MAX_METADATA_VALUE_LEN as usize {
                return Err(AccountError::ValidationError {
                    info: format!(
                        "Account metadata value length exceeds the maximum allowed: {}",
                        Self::MAX_METADATA_VALUE_LEN
                    ),
                });
            }
        }

        Ok(())
    }

    pub fn validate_symbol(&self) -> ModelValidatorResult<AccountError> {
        if (self.account.symbol.len() < Self::SYMBOL_RANGE.0 as usize)
            || (self.account.symbol.len() > Self::SYMBOL_RANGE.1 as usize)
        {
            return Err(AccountError::ValidationError {
                info: format!(
                    "Account symbol length must be between {} and {}",
                    Self::SYMBOL_RANGE.0,
                    Self::SYMBOL_RANGE.1
                ),
            });
        }

        Ok(())
    }

    pub fn validate_owners(&self) -> ModelValidatorResult<AccountError> {
        if (self.account.owners.len() < Self::OWNERS_RANGE.0 as usize)
            || (self.account.owners.len() > Self::OWNERS_RANGE.1 as usize)
        {
            return Err(AccountError::InvalidOwnersRange {
                min_owners: Self::OWNERS_RANGE.0,
                max_owners: Self::OWNERS_RANGE.1,
            });
        }

        Ok(())
    }

    pub fn validate_address(&self) -> ModelValidatorResult<AccountError> {
        if (self.account.address.len() < Self::ADDRESS_RANGE.0 as usize)
            || (self.account.address.len() > Self::ADDRESS_RANGE.1 as usize)
        {
            return Err(AccountError::InvalidAddressLength {
                min_length: Self::ADDRESS_RANGE.0,
                max_length: Self::ADDRESS_RANGE.1,
            });
        }

        Ok(())
    }

    pub fn validate(&self) -> ModelValidatorResult<AccountError> {
        self.validate_policies()?;
        self.validate_metadata()?;
        self.validate_symbol()?;
        self.validate_address()?;
        self.validate_owners()?;

        Ok(())
    }
}

impl ModelValidator<AccountError> for Account {
    fn validate(&self) -> ModelValidatorResult<AccountError> {
        AccountValidator::new(self).validate()
    }
}

impl Account {
    /// Creates a new account key from the given key components.
    pub fn key(id: AccountId) -> AccountKey {
        AccountKey { id }
    }

    pub fn to_key(&self) -> AccountKey {
        Self::key(self.id)
    }

    pub fn metadata_map(&self) -> HashMap<String, String> {
        self.metadata
            .iter()
            .map(|(key, value)| (key.to_owned(), value.to_owned()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::account_test_utils::mock_account;
    use super::*;
    use crate::models::ApprovalThresholdPolicy;

    #[test]
    fn fail_policies_validation() {
        let mut account = mock_account();
        account.policies =
            vec![
                AccountPolicy::ApprovalThreshold(ApprovalThresholdPolicy::FixedThreshold(1),);
                AccountValidator::MAX_POLICIES as usize + 1
            ];

        let result = AccountValidator::new(&account).validate_policies();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            AccountError::ValidationError {
                info: "Account policies count exceeds the maximum allowed: 10".to_string()
            }
        );
    }

    #[test]
    fn test_policies_validation() {
        let mut account = mock_account();
        account.policies =
            vec![
                AccountPolicy::ApprovalThreshold(ApprovalThresholdPolicy::FixedThreshold(1),);
                AccountValidator::MAX_POLICIES as usize - 1
            ];

        let result = AccountValidator::new(&account).validate_policies();

        assert!(result.is_ok());
    }

    #[test]
    fn fail_metadata_validation_too_many() {
        let mut account = mock_account();
        account.metadata =
            vec![("a".repeat(25), "b".repeat(25)); AccountValidator::MAX_METADATA as usize + 1];

        let result = AccountValidator::new(&account).validate_metadata();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            AccountError::ValidationError {
                info: "Account metadata count exceeds the maximum allowed: 10".to_string()
            }
        );
    }

    #[test]
    fn test_metadata_validation() {
        let mut account = mock_account();
        account.metadata =
            vec![("a".repeat(24), "b".repeat(24)); AccountValidator::MAX_METADATA as usize - 1];

        let result = AccountValidator::new(&account).validate_metadata();

        assert!(result.is_ok());
    }

    #[test]
    fn fail_symbol_validation_too_short() {
        let mut account = mock_account();
        account.symbol = "a".repeat(0);

        let result = AccountValidator::new(&account).validate_symbol();

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
        account.symbol = "a".repeat(9);

        let result = AccountValidator::new(&account).validate_symbol();

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

        let result = AccountValidator::new(&account).validate_symbol();

        assert!(result.is_ok());
    }

    #[test]
    fn fail_address_too_short() {
        let mut account = mock_account();
        account.address = "".to_string();

        let result = AccountValidator::new(&account).validate_address();

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
        account.address = "a".repeat(256);

        let result = AccountValidator::new(&account).validate_address();

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

        let result = AccountValidator::new(&account).validate_address();

        assert!(result.is_ok());
    }

    #[test]
    fn fail_owners_too_many_entries() {
        let mut account = mock_account();
        account.owners = vec![[0; 16]; AccountValidator::OWNERS_RANGE.1 as usize + 1];

        let result = AccountValidator::new(&account).validate_owners();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            AccountError::InvalidOwnersRange {
                min_owners: 1,
                max_owners: 10
            }
        );
    }

    #[test]
    fn fail_owners_too_little_entries() {
        let mut account = mock_account();
        account.owners = vec![[0; 16]; AccountValidator::OWNERS_RANGE.0 as usize - 1];

        let result = AccountValidator::new(&account).validate_owners();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            AccountError::InvalidOwnersRange {
                min_owners: 1,
                max_owners: 10
            }
        );
    }

    #[test]
    fn test_owners_validation() {
        let mut account = mock_account();
        account.owners = vec![[0; 16]];

        let result = AccountValidator::new(&account).validate_owners();

        assert!(result.is_ok());
    }
}

#[cfg(test)]
pub mod account_test_utils {
    use super::*;

    pub fn mock_account() -> Account {
        Account {
            id: [0; 16],
            address: "0x1234".to_string(),
            balance: None,
            blockchain: Blockchain::InternetComputer,
            decimals: 0u32,
            name: None,
            owners: vec![],
            policies: vec![],
            standard: BlockchainStandard::Native,
            last_modification_timestamp: 0,
            metadata: vec![
                ("a".repeat(24), "b".repeat(24));
                AccountValidator::MAX_METADATA as usize - 1
            ],
            symbol: "ICP".to_string(),
        }
    }
}
