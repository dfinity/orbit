use crate::errors::UserError;
use candid::Principal;
use orbit_essentials::model::{ModelValidator, ModelValidatorResult};
use orbit_essentials::storable;

pub const NAME_LEN_RANGE: (u8, u8) = (1, 48);
pub const MAX_LABELS: usize = 25;
pub const MAX_LABEL_LEN: usize = 64;

#[storable]
#[derive(Clone, Debug, Ord, Eq, PartialOrd)]
pub struct UserStation {
    // The canister id of the station.
    pub canister_id: Principal,
    // The name of the station.
    pub name: String,
    // The labels associated with the station.
    #[serde(default)]
    pub labels: Vec<String>,
}

impl PartialEq for UserStation {
    fn eq(&self, other: &Self) -> bool {
        self.canister_id == other.canister_id
    }
}

fn validate_name(name: &str) -> ModelValidatorResult<UserError> {
    if (name.trim().len() < NAME_LEN_RANGE.0 as usize)
        || (name.trim().len() > NAME_LEN_RANGE.1 as usize)
    {
        return Err(UserError::ValidationError {
            info: format!(
                "Station name length must be between {} and {}",
                NAME_LEN_RANGE.0, NAME_LEN_RANGE.1
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

fn validate_labels(labels: &[String]) -> ModelValidatorResult<UserError> {
    if labels.len() > MAX_LABELS {
        return Err(UserError::StationHasTooManyLabels {
            max_labels: MAX_LABELS,
        });
    }

    for label in labels {
        if label.len() > MAX_LABEL_LEN {
            return Err(UserError::ValidationError {
                info: format!("Station label length cannot exceed {}", MAX_LABEL_LEN),
            });
        }

        if label.starts_with(' ') || label.ends_with(' ') {
            return Err(UserError::ValidationError {
                info: "Station label value cannot start or end with a space".to_string(),
            });
        }

        if label.is_empty() {
            return Err(UserError::ValidationError {
                info: "Station label value cannot be empty".to_string(),
            });
        }
    }

    Ok(())
}

impl ModelValidator<UserError> for UserStation {
    fn validate(&self) -> ModelValidatorResult<UserError> {
        validate_name(&self.name)?;
        validate_labels(&self.labels)?;

        Ok(())
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
            labels: Vec::new(),
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
    #[case::name_too_big(&"amkyMJuUzYRXmxJuyUFeetxXbkMKmfCBwQnSazukXXGuxmwX1")]
    fn invalid_user_station_name(#[case] name: &str) {
        let user_station = UserStation {
            canister_id: Principal::anonymous(),
            name: String::from(name),
            labels: Vec::new(),
        };

        assert!(validate_name(&user_station.name).is_err());
    }

    #[rstest]
    #[case::short_name("A")]
    #[case::short_number_name("1")]
    #[case::common_name("Treasury")]
    #[case::long_name("amkyMJuUzYRXmxJuyUFeetxXbkMKmfCBwQnSazukXXGuxmwX")]
    fn valid_user_station_name(#[case] name: &str) {
        let user_station = UserStation {
            canister_id: Principal::anonymous(),
            name: String::from(name),
            labels: Vec::new(),
        };

        assert!(validate_name(&user_station.name).is_ok());
    }

    #[test]
    fn empty_labels_should_be_ok() {
        let user_station = UserStation {
            canister_id: Principal::anonymous(),
            name: "Station".to_string(),
            labels: Vec::new(),
        };

        assert!(validate_labels(&user_station.labels).is_ok());
    }

    #[test]
    fn too_many_labels_should_fail() {
        let user_station = UserStation {
            canister_id: Principal::anonymous(),
            name: "Station".to_string(),
            labels: vec!["label".to_string(); MAX_LABELS + 1],
        };

        assert!(validate_labels(&user_station.labels).is_err());
    }

    #[test]
    fn should_pass_labels_validation() {
        let user_station = UserStation {
            canister_id: Principal::anonymous(),
            name: "Station".to_string(),
            labels: vec!["label".to_string(); MAX_LABELS],
        };

        assert!(validate_labels(&user_station.labels).is_ok());
    }

    #[rstest]
    #[case::empty_label(&[""])]
    #[case::too_long_label(&["a"; MAX_LABEL_LEN + 1])]
    #[case::label_with_space_at_end(&["label "])]
    #[case::label_with_space_at_start(&[" label"])]
    fn invalid_labels(#[case] labels: &[&str]) {
        let user_station = UserStation {
            canister_id: Principal::anonymous(),
            name: "Station".to_string(),
            labels: labels.iter().map(|l| l.to_string()).collect(),
        };

        assert!(validate_labels(&user_station.labels).is_err());
    }
}

#[cfg(test)]
pub mod user_station_model_utils {
    use uuid::Uuid;

    use super::UserStation;
    use crate::core::test_utils;

    pub fn mock_user_station() -> UserStation {
        let principal = test_utils::random_principal();
        let station_name = Uuid::new_v4().to_string();

        UserStation {
            canister_id: principal,
            name: station_name,
            labels: Vec::new(),
        }
    }
}
