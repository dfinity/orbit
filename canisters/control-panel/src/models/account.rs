use super::{AccountBank, AccountIdentity};
use crate::{
    core::{MAX_BYTE_SIZE_PRINCIPAL, MAX_BYTE_SIZE_UUID},
    errors::AccountError,
};
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_canister_core::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use ic_canister_macros::stable_object;

pub type AccountId = UUID;

/// The key used to store an account identity in stable memory.
#[stable_object(size = AccountKey::MAX_BYTE_SIZE)]
#[derive(CandidType, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AccountKey {
    /// The UUID that identifies the account.
    pub id: AccountId,
}

/// The identity of an account.
#[stable_object(size = Account::MAX_BYTE_SIZE)]
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
    /// The maximum number of identities that can be associated with an account,
    /// this is limited to have a fixed size for the account in stable memory.
    pub const MAX_ACCOUNT_IDENTITIES: u32 = 10;

    /// The maximum number of unconfirmed identities at any given time with an account.
    pub const MAX_ACCOUNT_UNCONFIRMED_IDENTITIES: u32 = 5;

    /// The maximum number of banks that can be associated with an account,
    /// this is limited to have a fixed size for the account in stable memory.
    pub const MAX_ACCOUNT_BANKS: u32 = 10;

    /// The maximum size of each field in stable memory.
    pub const MAX_BYTE_SIZE_ID: u32 = MAX_BYTE_SIZE_UUID;
    pub const MAX_BYTE_SIZE_NAME: u32 = 150;
    pub const MAX_BYTE_SIZE_MAIN_BANK: u32 = MAX_BYTE_SIZE_PRINCIPAL;
    pub const MAX_BYTE_SIZE_BANKS: u32 = AccountBank::MAX_BYTE_SIZE * Self::MAX_ACCOUNT_BANKS;
    pub const MAX_BYTE_SIZE_IDENTITIES: u32 =
        AccountIdentity::MAX_BYTE_SIZE * Self::MAX_ACCOUNT_IDENTITIES;
    pub const MAX_BYTE_SIZE_UNCONFIRMED_IDENTITIES: u32 =
        AccountIdentity::MAX_BYTE_SIZE * Self::MAX_ACCOUNT_UNCONFIRMED_IDENTITIES;
    pub const MAX_BYTE_SIZE_LAST_UPDATE_TIMESTAMP: u32 = std::mem::size_of::<u64>() as u32;

    /// The maximum size of an AccountIdentity in stable memory.
    pub const MAX_BYTE_SIZE: u32 = 8096;

    /// The number of bytes that are not used by the account and could be used to add more fields to the account
    /// without breaking the stable memory layout, if this overflows then the stable memory layout will be broken.
    pub const SPARE_BYTES: u32 = Self::MAX_BYTE_SIZE
        - Self::MAX_BYTE_SIZE_ID
        - Self::MAX_BYTE_SIZE_NAME
        - Self::MAX_BYTE_SIZE_MAIN_BANK
        - Self::MAX_BYTE_SIZE_BANKS
        - Self::MAX_BYTE_SIZE_IDENTITIES
        - Self::MAX_BYTE_SIZE_UNCONFIRMED_IDENTITIES
        - Self::MAX_BYTE_SIZE_LAST_UPDATE_TIMESTAMP;

    pub fn key(account_id: &UUID) -> AccountKey {
        AccountKey { id: *account_id }
    }

    pub fn to_key(&self) -> AccountKey {
        Account::key(&self.id)
    }
}

impl AccountKey {
    /// The maximum size of each field in stable memory.
    pub const MAX_BYTE_SIZE_ID: u32 = MAX_BYTE_SIZE_UUID;

    /// The maximum size of an AccountKey in stable memory.
    pub const MAX_BYTE_SIZE: u32 = 48;

    /// The number of bytes that are not used by the account and could be used to add more fields to the account
    /// without breaking the stable memory layout, if this overflows then the stable memory layout will be broken.
    pub const SPARE_BYTES: u32 = Self::MAX_BYTE_SIZE - Self::MAX_BYTE_SIZE_ID;
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
