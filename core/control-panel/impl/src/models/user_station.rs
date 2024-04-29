use crate::errors::UserError;
use candid::Principal;
use orbit_essentials::model::{ModelValidator, ModelValidatorResult};
use orbit_essentials::storable;

#[storable(serializer = "candid")]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct UserStation {
    pub canister_id: Principal,
    pub name: String,
}

pub struct UserStationValidator<'model> {
    model: &'model UserStation,
}

impl<'model> UserStationValidator<'model> {
    pub const NAME_LEN_RANGE: (u8, u8) = (1, 48);

    pub fn new(model: &'model UserStation) -> Self {
        Self { model }
    }

    pub fn validate_name(&self) -> ModelValidatorResult<UserError> {
        let name = self.model.name.clone();

        if (name.trim().len() < Self::NAME_LEN_RANGE.0 as usize)
            || (name.trim().len() > Self::NAME_LEN_RANGE.1 as usize)
        {
            return Err(UserError::ValidationError {
                info: format!(
                    "Station name length must be between {} and {}",
                    Self::NAME_LEN_RANGE.0,
                    Self::NAME_LEN_RANGE.1
                ),
            });
        }

        if name.starts_with(' ') || name.ends_with(' ') {
            return Err(UserError::ValidationError {
                info: "Station name cannot start or end with a space".to_string(),
            });
        }

        Ok(())
    }

    pub fn validate(&self) -> ModelValidatorResult<UserError> {
        self.validate_name()?;

        Ok(())
    }
}

impl ModelValidator<UserError> for UserStation {
    fn validate(&self) -> ModelValidatorResult<UserError> {
        UserStationValidator::new(self).validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ic_stable_structures::Storable;
    use rstest::rstest;

    #[test]
    fn valid_model_serialization() {
        let user_station = UserStation {
            canister_id: Principal::from_text("wkt3w-3iaaa-aaaaa-774ba-cai").unwrap(),
            name: "Station 1".to_string(),
        };

        let serialized_model = user_station.to_bytes();
        let deserialized_model = UserStation::from_bytes(serialized_model);

        assert_eq!(user_station.canister_id, deserialized_model.canister_id);
        assert_eq!(user_station.name, deserialized_model.name);
    }

    #[rstest]
    #[case::empty_name(&"")]
    #[case::empty_name_with_space(&" ")]
    #[case::starts_with_space(&" Treasury")]
    #[case::ends_with_space(&"Treasury ")]
    #[case::name_too_big(&"amkyMJuUzYRXmxJuyUFeetxXbkMKmfCBwQnSazukXXGuxmwXJE1")]
    fn invalid_user_station_name(#[case] name: &str) {
        let user_station = UserStation {
            canister_id: Principal::anonymous(),
            name: String::from(name),
        };
        let validator = UserStationValidator::new(&user_station);

        assert!(validator.validate_name().is_err());
    }

    #[rstest]
    #[case::short_name("A")]
    #[case::short_number_name("1")]
    #[case::common_name("Treasury")]
    #[case::long_name("amkyMJuUzYRXmxJuyUFeetxXbkMKmfCBwQnSazukXXGuxmwXJE")]
    fn valid_user_station_name(#[case] name: &str) {
        let user_station = UserStation {
            canister_id: Principal::anonymous(),
            name: String::from(name),
        };
        let validator = UserStationValidator::new(&user_station);

        assert!(validator.validate_name().is_ok());
    }
}
