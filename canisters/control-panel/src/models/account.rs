use super::{AccountBank, AccountIdentity};
use crate::errors::AccountError;
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_canister_core::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use ic_canister_macros::stable_object;

pub type AccountId = UUID;

/// The key used to store an account identity in stable memory.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AccountKey {
    /// The UUID that identifies the account.
    pub id: AccountId,
}

/// The identity of an account.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Account {
    /// The UUID that identifies the account.
    pub id: UUID,
    /// The name of the account (if any).
    pub name: Option<String>,
    /// The shared bank to use for the account.
    pub main_bank: Option<Principal>,
    /// The status of the identity.
    pub banks: Vec<AccountBank>,
    /// The identifies associated with the account.
    pub identities: Vec<AccountIdentity>,
    /// The unconfirmed identifies associated with the account.
    pub unconfirmed_identities: Vec<AccountIdentity>,
    /// Last time the identity was updated.
    pub last_update_timestamp: Timestamp,
}

impl Account {
    pub fn key(account_id: &UUID) -> AccountKey {
        AccountKey { id: *account_id }
    }

    pub fn to_key(&self) -> AccountKey {
        Account::key(&self.id)
    }
}

pub struct AccountValidator<'model> {
    model: &'model Account,
}

impl<'model> AccountValidator<'model> {
    pub const IDENTITIES_RANGE: (u8, u8) = (1, 10);
    pub const MAX_UNCONFIRMED_IDENTITIES: u8 = 9;
    pub const MAX_BANKS: u8 = 10;

    pub fn new(model: &'model Account) -> Self {
        Self { model }
    }

    pub fn validate_identities(&self) -> ModelValidatorResult<AccountError> {
        if self.model.identities.len() < Self::IDENTITIES_RANGE.0 as usize {
            return Err(AccountError::ValidationError {
                info: format!(
                    "Too little identities, expected at least {} but got {}",
                    Self::IDENTITIES_RANGE.0,
                    self.model.identities.len()
                ),
            });
        }

        if self.model.identities.len() > Self::IDENTITIES_RANGE.1 as usize {
            return Err(AccountError::ValidationError {
                info: format!(
                    "Too many identities, expected at most {} but got {}",
                    Self::IDENTITIES_RANGE.1,
                    self.model.identities.len()
                ),
            });
        }

        Ok(())
    }

    pub fn validate_unconfirmed_identities(&self) -> ModelValidatorResult<AccountError> {
        if self.model.unconfirmed_identities.len() > Self::MAX_UNCONFIRMED_IDENTITIES as usize {
            return Err(AccountError::ValidationError {
                info: format!(
                    "Too many unconfirmed identities, expected at most {} but got {}",
                    Self::MAX_UNCONFIRMED_IDENTITIES,
                    self.model.unconfirmed_identities.len()
                ),
            });
        }

        Ok(())
    }

    pub fn validate_banks(&self) -> ModelValidatorResult<AccountError> {
        if self.model.banks.len() > Self::MAX_BANKS as usize {
            return Err(AccountError::ValidationError {
                info: format!(
                    "Too many banks, expected at most {} but got {}",
                    Self::MAX_BANKS,
                    self.model.banks.len()
                ),
            });
        }

        Ok(())
    }

    pub fn validate_main_bank(&self) -> ModelValidatorResult<AccountError> {
        if let Some(main_bank) = &self.model.main_bank {
            if !self
                .model
                .banks
                .iter()
                .any(|bank| &bank.canister_id == main_bank)
            {
                return Err(AccountError::ValidationError {
                    info: format!(
                        "Main bank {} is not in the list of banks {:?}",
                        main_bank, self.model.banks
                    ),
                });
            }
        }

        Ok(())
    }

    pub fn validate(&self) -> ModelValidatorResult<AccountError> {
        self.validate_identities()?;
        self.validate_unconfirmed_identities()?;
        self.validate_banks()?;
        self.validate_main_bank()?;

        Ok(())
    }
}

impl ModelValidator<AccountError> for Account {
    fn validate(&self) -> ModelValidatorResult<AccountError> {
        AccountValidator::new(self).validate()
    }
}
