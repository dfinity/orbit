use crate::errors::UserError;
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_canister_core::model::{ModelValidator, ModelValidatorResult};
use ic_canister_macros::stable_object;

/// The identity of an user.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct UserIdentity {
    /// The principal ID of the identity.
    pub identity: Principal,
    /// The name of the identity (if any).
    pub name: Option<String>,
}

pub struct UserIdentityValidator<'model> {
    model: &'model UserIdentity,
}

impl<'model> UserIdentityValidator<'model> {
    pub const NAME_LEN_RANGE: (u8, u8) = (1, 100);

    pub fn new(model: &'model UserIdentity) -> Self {
        Self { model }
    }

    pub fn validate_name(&self) -> ModelValidatorResult<UserError> {
        if let Some(name) = &self.model.name {
            if (name.trim().len() < Self::NAME_LEN_RANGE.0 as usize)
                || (name.trim().len() > Self::NAME_LEN_RANGE.1 as usize)
            {
                return Err(UserError::ValidationError {
                    info: format!(
                        "User identity name length must be between {} and {}",
                        Self::NAME_LEN_RANGE.0,
                        Self::NAME_LEN_RANGE.1
                    ),
                });
            }

            if name.starts_with(' ') || name.ends_with(' ') {
                return Err(UserError::ValidationError {
                    info: "Identity name cannot start or end with a space".to_string(),
                });
            }
        }

        Ok(())
    }

    pub fn validate(&self) -> ModelValidatorResult<UserError> {
        self.validate_name()?;

        Ok(())
    }
}

impl ModelValidator<UserError> for UserIdentity {
    fn validate(&self) -> ModelValidatorResult<UserError> {
        UserIdentityValidator::new(self).validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ic_stable_structures::Storable;
    use rstest::rstest;

    #[test]
    fn valid_model_serialization() {
        let model = UserIdentity {
            identity: Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
            name: Some("Bank 1".to_string()),
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = UserIdentity::from_bytes(serialized_model);

        assert_eq!(model.identity, deserialized_model.identity);
        assert_eq!(model.name, deserialized_model.name);
    }

    #[rstest]
    #[case::empty_name(&"")]
    #[case::empty_name_with_space(&" ")]
    #[case::starts_with_space(&" Main")]
    #[case::ends_with_space(&"Main ")]
    #[case::name_too_big(&"amkyMJuUzYRXmxJuyUFeetxXbkMKmfCBwQnSazukXXGuxmwXJEcxxSxAMqLzZWSzaYpdfKCnKDTjfrkfYvRhhmCrTrmqUUkbgkbgg")]
    fn invalid_identity_name(#[case] name: &str) {
        let user_bank = UserIdentity {
            identity: Principal::anonymous(),
            name: Some(String::from(name)),
        };
        let validator = UserIdentityValidator::new(&user_bank);

        assert!(validator.validate_name().is_err());
    }

    #[rstest]
    #[case::no_name(None)]
    #[case::short_name(Some(String::from("A")))]
    #[case::short_number_name(Some(String::from("1")))]
    #[case::common_name(Some(String::from("Main")))]
    #[case::long_name(Some(String::from("amkyMJuUzYRXmxJuyUFeetxXbkMKmfCBwQnSazukXXGuxmwXJEcxxSxAMqLzZWSzaYpdfKCnKDTjfrkfYvRhhmCrTrmqUUkbgkbg")))]
    fn valid_user_bank_name(#[case] name: Option<String>) {
        let user_bank = UserIdentity {
            identity: Principal::anonymous(),
            name,
        };
        let validator = UserIdentityValidator::new(&user_bank);

        assert!(validator.validate_name().is_ok());
    }
}
