use crate::errors::AccountError;
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_canister_core::model::{ModelValidator, ModelValidatorResult};
use ic_canister_macros::stable_object;

/// The identity of an account.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct AccountIdentity {
    /// The principal ID of the identity.
    pub identity: Principal,
    /// The name of the identity (if any).
    pub name: Option<String>,
}

pub struct AccountIdentityValidator<'model> {
    model: &'model AccountIdentity,
}

impl<'model> AccountIdentityValidator<'model> {
    pub const NAME_LEN_RANGE: (u8, u8) = (1, 100);

    pub fn new(model: &'model AccountIdentity) -> Self {
        Self { model }
    }

    pub fn validate_name(&self) -> ModelValidatorResult<AccountError> {
        if let Some(name) = &self.model.name {
            if (name.len() < Self::NAME_LEN_RANGE.0 as usize)
                || (name.len() > Self::NAME_LEN_RANGE.1 as usize)
            {
                return Err(AccountError::ValidationError {
                    info: format!(
                        "Account identity name length must be between {} and {}",
                        Self::NAME_LEN_RANGE.0,
                        Self::NAME_LEN_RANGE.1
                    ),
                });
            }
        }

        Ok(())
    }

    pub fn validate(&self) -> ModelValidatorResult<AccountError> {
        self.validate_name()?;

        Ok(())
    }
}

impl ModelValidator<AccountError> for AccountIdentity {
    fn validate(&self) -> ModelValidatorResult<AccountError> {
        AccountIdentityValidator::new(self).validate()
    }
}
