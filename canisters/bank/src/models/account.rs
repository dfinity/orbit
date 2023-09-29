use super::AccessRole;
use crate::errors::AccountError;
use candid::{CandidType, Deserialize, Principal};
use ic_canister_core::types::{Timestamp, UUID};
use ic_canister_macros::stable_object;

/// The account id, which is a UUID.
pub type AccountId = UUID;

/// Represents an account within the system.
///
/// An account can be associated with one or more identity.
#[stable_object(size = 1024)]
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

#[stable_object(size = 64)]
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

    pub fn validate_identities(&self) -> Result<(), AccountError> {
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

    pub fn validate_access_roles(&self) -> Result<(), AccountError> {
        if self.account.identities.len() < Self::ACCESS_ROLES_RANGE.0 as usize {
            return Err(AccountError::TooLittleAccessRoles);
        }

        if self.account.identities.len() > Self::ACCESS_ROLES_RANGE.1 as usize {
            return Err(AccountError::TooManyAccessRoles {
                max_access_roles: Self::ACCESS_ROLES_RANGE.1,
            });
        }

        Ok(())
    }

    pub fn validate_unconfirmed_identities(&self) -> Result<(), AccountError> {
        if self.account.identities.len() > Self::MAX_UNCONFIRMED_IDENTITIES as usize {
            return Err(AccountError::TooManyUnconfirmedIdentities {
                max_identities: Self::MAX_UNCONFIRMED_IDENTITIES,
            });
        }

        Ok(())
    }

    pub fn validate(&self) -> Result<(), AccountError> {
        self.validate_identities()?;
        self.validate_unconfirmed_identities()?;
        self.validate_access_roles()?;

        Ok(())
    }
}

impl Account {
    /// Creates a new account key from the given key components.
    pub fn key(id: AccountId) -> AccountKey {
        AccountKey { id }
    }

    pub fn as_key(&self) -> AccountKey {
        Account::key(self.id)
    }

    pub fn validate(&self) -> Result<(), AccountError> {
        AccountValidator::new(self).validate()?;

        Ok(())
    }
}
