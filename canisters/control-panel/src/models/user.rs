use super::{UserBank, UserIdentity};
use crate::errors::UserError;
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_canister_core::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use ic_canister_macros::stable_object;

pub type UserId = UUID;

/// The key used to store an user identity in stable memory.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UserKey {
    /// The UUID that identifies the user.
    pub id: UserId,
}

/// The identity of an user.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct User {
    /// The UUID that identifies the user.
    pub id: UUID,
    /// The name of the user (if any).
    pub name: Option<String>,
    /// The shared bank to use for the user.
    pub main_bank: Option<Principal>,
    /// The status of the identity.
    pub banks: Vec<UserBank>,
    /// The identifies associated with the user.
    pub identities: Vec<UserIdentity>,
    /// The unconfirmed identifies associated with the user.
    pub unconfirmed_identities: Vec<UserIdentity>,
    /// Last time the identity was updated.
    pub last_update_timestamp: Timestamp,
}

impl User {
    pub fn key(user_id: &UUID) -> UserKey {
        UserKey { id: *user_id }
    }

    pub fn to_key(&self) -> UserKey {
        User::key(&self.id)
    }
}

pub struct UserValidator<'model> {
    model: &'model User,
}

impl<'model> UserValidator<'model> {
    pub const IDENTITIES_RANGE: (u8, u8) = (1, 10);
    pub const NAME_LEN_RANGE: (u8, u8) = (1, 100);
    pub const MAX_UNCONFIRMED_IDENTITIES: u8 = 9;
    pub const MAX_BANKS: u8 = 10;

    pub fn new(model: &'model User) -> Self {
        Self { model }
    }

    pub fn validate_identities(&self) -> ModelValidatorResult<UserError> {
        if self.model.identities.len() < Self::IDENTITIES_RANGE.0 as usize {
            return Err(UserError::ValidationError {
                info: format!(
                    "Too little identities, expected at least {} but got {}",
                    Self::IDENTITIES_RANGE.0,
                    self.model.identities.len()
                ),
            });
        }

        if self.model.identities.len() > Self::IDENTITIES_RANGE.1 as usize {
            return Err(UserError::ValidationError {
                info: format!(
                    "Too many identities, expected at most {} but got {}",
                    Self::IDENTITIES_RANGE.1,
                    self.model.identities.len()
                ),
            });
        }

        Ok(())
    }

    pub fn validate_unconfirmed_identities(&self) -> ModelValidatorResult<UserError> {
        if self.model.unconfirmed_identities.len() > Self::MAX_UNCONFIRMED_IDENTITIES as usize {
            return Err(UserError::ValidationError {
                info: format!(
                    "Too many unconfirmed identities, expected at most {} but got {}",
                    Self::MAX_UNCONFIRMED_IDENTITIES,
                    self.model.unconfirmed_identities.len()
                ),
            });
        }

        Ok(())
    }

    pub fn validate_banks(&self) -> ModelValidatorResult<UserError> {
        if self.model.banks.len() > Self::MAX_BANKS as usize {
            return Err(UserError::ValidationError {
                info: format!(
                    "Too many banks, expected at most {} but got {}",
                    Self::MAX_BANKS,
                    self.model.banks.len()
                ),
            });
        }

        Ok(())
    }

    pub fn validate_main_bank(&self) -> ModelValidatorResult<UserError> {
        if let Some(main_bank) = &self.model.main_bank {
            if !self
                .model
                .banks
                .iter()
                .any(|bank| &bank.canister_id == main_bank)
            {
                return Err(UserError::ValidationError {
                    info: format!(
                        "Main bank {} is not in the list of banks {:?}",
                        main_bank, self.model.banks
                    ),
                });
            }
        }

        Ok(())
    }

    pub fn validate_name(&self) -> ModelValidatorResult<UserError> {
        if let Some(name) = &self.model.name {
            if (name.trim().len() < Self::NAME_LEN_RANGE.0 as usize)
                || (name.trim().len() > Self::NAME_LEN_RANGE.1 as usize)
            {
                return Err(UserError::ValidationError {
                    info: format!(
                        "User name length must be between {} and {}",
                        Self::NAME_LEN_RANGE.0,
                        Self::NAME_LEN_RANGE.1
                    ),
                });
            }

            if name.starts_with(' ') || name.ends_with(' ') {
                return Err(UserError::ValidationError {
                    info: "User name cannot start or end with a space".to_string(),
                });
            }
        }

        Ok(())
    }

    pub fn validate(&self) -> ModelValidatorResult<UserError> {
        self.validate_identities()?;
        self.validate_unconfirmed_identities()?;
        self.validate_banks()?;
        self.validate_main_bank()?;
        self.validate_name()?;

        Ok(())
    }
}

impl ModelValidator<UserError> for User {
    fn validate(&self) -> ModelValidatorResult<UserError> {
        UserValidator::new(self).validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ic_stable_structures::Storable;
    use rstest::rstest;

    #[test]
    fn valid_model_serialization() {
        let model = User {
            id: [u8::MAX; 16],
            identities: vec![UserIdentity {
                identity: Principal::anonymous(),
                name: None,
            }],
            unconfirmed_identities: vec![],
            banks: vec![],
            main_bank: None,
            last_update_timestamp: 10,
            name: Some("Treasury".to_string()),
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = User::from_bytes(serialized_model);

        assert_eq!(model.id, deserialized_model.id);
        assert_eq!(model.identities, deserialized_model.identities);
        assert_eq!(
            model.last_update_timestamp,
            deserialized_model.last_update_timestamp
        );
        assert_eq!(model.name, deserialized_model.name);
    }

    #[rstest]
    #[case::empty_name(&"")]
    #[case::empty_name_with_space(&" ")]
    #[case::starts_with_space(&" Vital")]
    #[case::ends_with_space(&"Vital ")]
    #[case::name_too_big(&"amkyMJuUzYRXmxJuyUFeetxXbkMKmfCBwQnSazukXXGuxmwXJEcxxSxAMqLzZWSzaYpdfKCnKDTjfrkfYvRhhmCrTrVmqUUkbgdMK")]
    fn invalid_user_name(#[case] name: &str) {
        let user = User {
            id: [u8::MAX; 16],
            identities: vec![],
            unconfirmed_identities: vec![],
            banks: vec![],
            main_bank: None,
            last_update_timestamp: 10,
            name: Some(name.to_string()),
        };
        let validator = UserValidator::new(&user);

        assert!(validator.validate_name().is_err());
    }

    #[rstest]
    #[case::short_name(&"A")]
    #[case::common_name(&"Vital")]
    #[case::large_name(&"amkyMJuUzYRXmxJuyUFeetxXbkMKmfCBwQnSazukXXGuxmwXJEcxxSxAMqLzZWSzaYpdfKCnKDTjfrkfYvRhhmCrTrVmqUUkbgdM")]
    fn valid_user_name(#[case] name: &str) {
        let user = User {
            id: [u8::MAX; 16],
            identities: vec![],
            unconfirmed_identities: vec![],
            banks: vec![],
            main_bank: None,
            last_update_timestamp: 10,
            name: Some(name.to_string()),
        };
        let validator = UserValidator::new(&user);

        assert!(validator.validate_name().is_ok());
    }

    #[test]
    fn check_identities_validation() {
        let user = User {
            id: [u8::MAX; 16],
            identities: vec![],
            unconfirmed_identities: vec![],
            banks: vec![],
            main_bank: None,
            last_update_timestamp: 0,
            name: None,
        };

        let user_with_no_identities = user.clone();
        let mut user_with_one_identity = user.clone();
        let mut user_with_too_many_identities = user.clone();

        user_with_one_identity.identities.push(UserIdentity {
            identity: Principal::anonymous(),
            name: None,
        });

        for _ in 0..=UserValidator::IDENTITIES_RANGE.1 {
            user_with_too_many_identities.identities.push(UserIdentity {
                identity: Principal::anonymous(),
                name: None,
            });
        }

        assert!(UserValidator::new(&user_with_no_identities)
            .validate_identities()
            .is_err());
        assert!(UserValidator::new(&user_with_one_identity)
            .validate_identities()
            .is_ok());
        assert!(UserValidator::new(&user_with_too_many_identities)
            .validate_identities()
            .is_err());
    }

    #[test]
    fn check_unconfirmed_identities_validation() {
        let user = User {
            id: [u8::MAX; 16],
            identities: vec![],
            unconfirmed_identities: vec![],
            banks: vec![],
            main_bank: None,
            last_update_timestamp: 0,
            name: None,
        };

        let user_with_no_identities = user.clone();
        let mut user_with_one_identity = user.clone();
        let mut user_with_too_many_identities = user.clone();

        user_with_one_identity
            .unconfirmed_identities
            .push(UserIdentity {
                identity: Principal::anonymous(),
                name: None,
            });

        for _ in 0..=UserValidator::MAX_UNCONFIRMED_IDENTITIES {
            user_with_too_many_identities
                .unconfirmed_identities
                .push(UserIdentity {
                    identity: Principal::anonymous(),
                    name: None,
                });
        }

        assert!(UserValidator::new(&user_with_no_identities)
            .validate_unconfirmed_identities()
            .is_ok());
        assert!(UserValidator::new(&user_with_one_identity)
            .validate_unconfirmed_identities()
            .is_ok());
        assert!(UserValidator::new(&user_with_too_many_identities)
            .validate_unconfirmed_identities()
            .is_err());
    }

    #[test]
    fn check_banks_validation() {
        let user = User {
            id: [u8::MAX; 16],
            identities: vec![],
            unconfirmed_identities: vec![],
            banks: vec![],
            main_bank: None,
            last_update_timestamp: 0,
            name: None,
        };

        let user_with_no_banks = user.clone();
        let mut user_with_one_bank = user.clone();
        let mut user_with_too_many_banks = user.clone();

        user_with_one_bank.banks.push(UserBank {
            canister_id: Principal::anonymous(),
            name: None,
        });

        for _ in 0..=UserValidator::MAX_BANKS {
            user_with_too_many_banks.banks.push(UserBank {
                canister_id: Principal::anonymous(),
                name: None,
            });
        }

        assert!(UserValidator::new(&user_with_no_banks)
            .validate_banks()
            .is_ok());
        assert!(UserValidator::new(&user_with_one_bank)
            .validate_banks()
            .is_ok());
        assert!(UserValidator::new(&user_with_too_many_banks)
            .validate_banks()
            .is_err());
    }

    #[test]
    fn valid_main_bank() {
        let user = User {
            id: [u8::MAX; 16],
            identities: vec![],
            unconfirmed_identities: vec![],
            banks: vec![UserBank {
                canister_id: Principal::anonymous(),
                name: None,
            }],
            main_bank: Some(Principal::anonymous()),
            last_update_timestamp: 0,
            name: None,
        };

        let validator = UserValidator::new(&user);

        assert!(validator.validate_main_bank().is_ok());
    }

    #[test]
    fn invalid_main_bank() {
        let user = User {
            id: [u8::MAX; 16],
            identities: vec![],
            unconfirmed_identities: vec![],
            banks: vec![UserBank {
                canister_id: Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
                name: None,
            }],
            main_bank: Some(Principal::anonymous()),
            last_update_timestamp: 0,
            name: None,
        };

        let validator = UserValidator::new(&user);

        assert!(validator.validate_main_bank().is_err());
    }
}
