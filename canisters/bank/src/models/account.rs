use super::AccessRole;
use crate::errors::AccountError;
use candid::{CandidType, Deserialize, Principal};
use ic_canister_core::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use ic_canister_macros::stable_object;

/// The account id, which is a UUID.
pub type AccountId = UUID;

/// Represents an account within the system.
///
/// An account can be associated with one or more identity.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Account {
    /// The account id, which is a UUID.
    pub id: AccountId,
    /// The identities associated with the account.
    pub identities: Vec<Principal>,
    /// The unconfirmed identities associated with the account.
    pub unconfirmed_identities: Vec<Principal>,
    /// The access roles associated with the account.
    pub access_roles: Vec<AccessRole>,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccountKey {
    /// The account id, which is a UUID.
    pub id: AccountId,
}

pub struct AccountValidator<'account> {
    account: &'account Account,
}

impl<'account> AccountValidator<'account> {
    pub const IDENTITIES_RANGE: (u8, u8) = (1, 10);
    pub const ACCESS_ROLES_RANGE: (u8, u8) = (1, 10);
    pub const MAX_UNCONFIRMED_IDENTITIES: u8 = 9;

    pub fn new(account: &'account Account) -> AccountValidator {
        AccountValidator { account }
    }

    pub fn validate_identities(&self) -> ModelValidatorResult<AccountError> {
        if self.account.identities.len() < Self::IDENTITIES_RANGE.0 as usize {
            return Err(AccountError::TooLittleIdentities);
        }

        if self.account.identities.len() > Self::IDENTITIES_RANGE.1 as usize {
            return Err(AccountError::TooManyIdentities {
                max_identities: Self::IDENTITIES_RANGE.1,
            });
        }

        Ok(())
    }

    pub fn validate_access_roles(&self) -> ModelValidatorResult<AccountError> {
        if self.account.access_roles.len() < Self::ACCESS_ROLES_RANGE.0 as usize {
            return Err(AccountError::TooLittleAccessRoles);
        }

        if self.account.access_roles.len() > Self::ACCESS_ROLES_RANGE.1 as usize {
            return Err(AccountError::TooManyAccessRoles {
                max_access_roles: Self::ACCESS_ROLES_RANGE.1,
            });
        }

        Ok(())
    }

    pub fn validate_unconfirmed_identities(&self) -> ModelValidatorResult<AccountError> {
        if self.account.unconfirmed_identities.len() > Self::MAX_UNCONFIRMED_IDENTITIES as usize {
            return Err(AccountError::TooManyUnconfirmedIdentities {
                max_identities: Self::MAX_UNCONFIRMED_IDENTITIES,
            });
        }

        Ok(())
    }

    pub fn validate(&self) -> ModelValidatorResult<AccountError> {
        self.validate_identities()?;
        self.validate_unconfirmed_identities()?;
        self.validate_access_roles()?;

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
        Account::key(self.id)
    }
}

#[cfg(test)]
pub mod tests {
    use super::account_test_utils::mock_account;
    use super::*;

    #[test]
    fn fail_account_too_little_identities() {
        let mut account = mock_account();
        account.identities = vec![];

        let result = AccountValidator::new(&account).validate_identities();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AccountError::TooLittleIdentities);
    }

    #[test]
    fn fail_account_too_many_identities() {
        let mut account = mock_account();
        account.identities =
            vec![Principal::anonymous(); AccountValidator::IDENTITIES_RANGE.1 as usize + 1];

        let result = AccountValidator::new(&account).validate_identities();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            AccountError::TooManyIdentities {
                max_identities: AccountValidator::IDENTITIES_RANGE.1
            }
        );
    }

    #[test]
    fn test_account_identities_validation() {
        let mut account = mock_account();
        account.identities = vec![Principal::anonymous(); 5];

        let result = AccountValidator::new(&account).validate_identities();

        assert!(result.is_ok());
    }

    #[test]
    fn fail_account_too_many_unconfirmed_identities() {
        let mut account = mock_account();
        account.unconfirmed_identities =
            vec![Principal::anonymous(); AccountValidator::MAX_UNCONFIRMED_IDENTITIES as usize + 1];

        let result = AccountValidator::new(&account).validate_unconfirmed_identities();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            AccountError::TooManyUnconfirmedIdentities {
                max_identities: AccountValidator::MAX_UNCONFIRMED_IDENTITIES
            }
        );
    }

    #[test]
    fn test_account_unconfirmed_identities_validation() {
        let mut account = mock_account();
        account.unconfirmed_identities =
            vec![Principal::anonymous(); AccountValidator::MAX_UNCONFIRMED_IDENTITIES as usize - 1];

        let result = AccountValidator::new(&account).validate_unconfirmed_identities();

        assert!(result.is_ok());
    }

    #[test]
    fn test_account_access_roles_validation() {
        let mut account = mock_account();
        account.access_roles = vec![AccessRole::User];

        let result = AccountValidator::new(&account).validate_access_roles();

        assert!(result.is_ok());
    }

    #[test]
    fn fail_account_access_roles_too_little() {
        let mut account = mock_account();
        account.access_roles = vec![];

        let result = AccountValidator::new(&account).validate_access_roles();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AccountError::TooLittleAccessRoles);
    }

    #[test]
    fn fail_account_access_roles_too_many() {
        let mut account = mock_account();
        account.access_roles =
            vec![AccessRole::User; AccountValidator::ACCESS_ROLES_RANGE.1 as usize + 1];

        let result = AccountValidator::new(&account).validate_access_roles();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            AccountError::TooManyAccessRoles {
                max_access_roles: AccountValidator::ACCESS_ROLES_RANGE.1
            }
        );
    }
}

#[cfg(test)]
pub mod account_test_utils {
    use super::*;

    pub fn mock_account() -> Account {
        Account {
            id: [0; 16],
            identities: vec![Principal::anonymous()],
            unconfirmed_identities: vec![],
            access_roles: vec![AccessRole::User],
            last_modification_timestamp: 0,
        }
    }
}
