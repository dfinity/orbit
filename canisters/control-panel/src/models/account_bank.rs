use crate::errors::AccountError;
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_canister_core::model::{ModelValidator, ModelValidatorResult};
use ic_canister_macros::stable_object;

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct AccountBank {
    pub canister_id: Principal,
    pub name: Option<String>,
}

pub struct AccountBankValidator<'model> {
    model: &'model AccountBank,
}

impl<'model> AccountBankValidator<'model> {
    pub const NAME_LEN_RANGE: (u8, u8) = (1, 150);

    pub fn new(model: &'model AccountBank) -> Self {
        Self { model }
    }

    pub fn validate_name(&self) -> ModelValidatorResult<AccountError> {
        if let Some(name) = &self.model.name {
            if (name.trim().len() < Self::NAME_LEN_RANGE.0 as usize)
                || (name.trim().len() > Self::NAME_LEN_RANGE.1 as usize)
            {
                return Err(AccountError::ValidationError {
                    info: format!(
                        "Bank name length must be between {} and {}",
                        Self::NAME_LEN_RANGE.0,
                        Self::NAME_LEN_RANGE.1
                    ),
                });
            }

            if name.starts_with(' ') || name.ends_with(' ') {
                return Err(AccountError::ValidationError {
                    info: "Bank name cannot start or end with a space".to_string(),
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

impl ModelValidator<AccountError> for AccountBank {
    fn validate(&self) -> ModelValidatorResult<AccountError> {
        AccountBankValidator::new(self).validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ic_stable_structures::Storable;
    use rstest::rstest;

    #[test]
    fn valid_model_serialization() {
        let account_bank = AccountBank {
            canister_id: Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
            name: Some("Bank 1".to_string()),
        };

        let serialized_model = account_bank.to_bytes();
        let deserialized_model = AccountBank::from_bytes(serialized_model);

        assert_eq!(account_bank.canister_id, deserialized_model.canister_id);
        assert_eq!(account_bank.name, deserialized_model.name);
    }

    #[rstest]
    #[case::empty_name(&"")]
    #[case::empty_name_with_space(&" ")]
    #[case::starts_with_space(&" Treasury")]
    #[case::ends_with_space(&"Treasury ")]
    #[case::name_too_big(&"amkyMJuUzYRXmxJuyUFeetxXbkMKmfCBwQnSazukXXGuxmwXJEcxxSxAMqLzZWSzaYpdfKCnKDTjfrkfYvRhhmCrTrVmqUUkbgdMKufYuimeCebnHWgQXeSzkeqcFLqSVxpdNeSGADkpvvjZHCYXLmM")]
    fn invalid_account_bank_name(#[case] name: &str) {
        let account_bank = AccountBank {
            canister_id: Principal::anonymous(),
            name: Some(String::from(name)),
        };
        let validator = AccountBankValidator::new(&account_bank);

        assert!(validator.validate_name().is_err());
    }

    #[rstest]
    #[case::no_name(None)]
    #[case::short_name(Some(String::from("A")))]
    #[case::short_number_name(Some(String::from("1")))]
    #[case::common_name(Some(String::from("Treasury")))]
    #[case::long_name(Some(String::from("amkyMJuUzYRXmxJuyUFeetxXbkMKmfCBwQnSazukXXGuxmwXJEcxxSxAMqLzZWSzaYpdfKCnKDTjfrkfYvRhhmCrTrVmqUUkbgdMKufYuimeCebnHWgQXeSzkeqcFLqSVxpdNeSGADkpvvjZHCYXLm")))]
    fn valid_account_bank_name(#[case] name: Option<String>) {
        let account_bank = AccountBank {
            canister_id: Principal::anonymous(),
            name,
        };
        let validator = AccountBankValidator::new(&account_bank);

        assert!(validator.validate_name().is_ok());
    }
}
