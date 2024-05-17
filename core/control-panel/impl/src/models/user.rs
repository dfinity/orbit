use super::UserStation;
use crate::errors::UserError;
use candid::Principal;
use email_address::EmailAddress;
use orbit_essentials::storable;
use orbit_essentials::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use std::str::FromStr;

/// The user id, which is a UUID.
pub type UserId = UUID;

/// The subscription status of an user.
#[storable]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UserSubscriptionStatus {
    Unsubscribed,
    Pending(String), // e-mail address to push notification to
    Approved,
    Denylisted,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CanDeployStation {
    NotAllowed(UserSubscriptionStatus),
    Allowed(usize),
    QuotaExceeded,
}

impl std::fmt::Display for UserSubscriptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UserSubscriptionStatus::Unsubscribed => write!(f, "unsubscribed"),
            UserSubscriptionStatus::Pending(_) => write!(f, "pending"),
            UserSubscriptionStatus::Approved => write!(f, "approved"),
            UserSubscriptionStatus::Denylisted => write!(f, "denylisted"),
        }
    }
}

/// The identity of an user.
#[storable]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct User {
    /// The UUID that identifies the user.
    pub id: UserId,
    /// The identity of the user.
    pub identity: Principal,
    /// The subscription status of the user.
    pub subscription_status: UserSubscriptionStatus,
    /// All the stations that the user has access.
    ///
    /// The first station in the list is the main station of the user.
    pub stations: Vec<UserStation>,
    /// The stations that have ever been deployed for the user by the control panel.
    /// Used to bound the total number of stations a user could deploy via the control panel.
    pub deployed_stations: Vec<Principal>,
    /// The timestamp of last time the user was active.
    pub last_active: Timestamp,
    /// Last time the identity was updated.
    pub last_update_timestamp: Timestamp,
}

#[storable]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UserKey(pub UUID);

impl User {
    pub const NAME_LEN_RANGE: (u8, u8) = (1, 100);
    pub const EMAIL_LEN_RANGE: (u8, u8) = (1, 100);
    pub const MAX_STATIONS: u8 = 15;
    pub const MAX_DEPLOYED_STATIONS: u8 = 3;

    pub fn to_key(&self) -> UserKey {
        UserKey(self.id)
    }

    pub fn can_deploy_station(&self) -> CanDeployStation {
        match self.subscription_status {
            UserSubscriptionStatus::Approved => (),
            UserSubscriptionStatus::Unsubscribed
            | UserSubscriptionStatus::Pending(_)
            | UserSubscriptionStatus::Denylisted => {
                return CanDeployStation::NotAllowed(self.subscription_status.clone());
            }
        };
        let max_deployed_stations: usize = Self::MAX_DEPLOYED_STATIONS.into();
        if self.deployed_stations.len() >= max_deployed_stations {
            return CanDeployStation::QuotaExceeded;
        }
        CanDeployStation::Allowed(max_deployed_stations - self.deployed_stations.len())
    }
}

fn validate_email(email: &str) -> ModelValidatorResult<UserError> {
    if (email.len() < User::EMAIL_LEN_RANGE.0 as usize)
        || (email.len() > User::EMAIL_LEN_RANGE.1 as usize)
    {
        return Err(UserError::ValidationError {
            info: format!(
                "Email length must be between {} and {}",
                User::EMAIL_LEN_RANGE.0,
                User::EMAIL_LEN_RANGE.1,
            ),
        });
    }
    if let Err(e) = EmailAddress::from_str(email) {
        return Err(UserError::ValidationError {
            info: format!("Email validation failed: {}", e,),
        });
    }

    Ok(())
}

fn validate_stations(stations: &[UserStation]) -> ModelValidatorResult<UserError> {
    if stations.len() > User::MAX_STATIONS as usize {
        return Err(UserError::ValidationError {
            info: format!(
                "Too many stations, expected at most {} but got {}",
                User::MAX_STATIONS,
                stations.len()
            ),
        });
    }

    for station in stations.iter() {
        if let Err(e) = station.validate() {
            return Err(UserError::ValidationError {
                info: format!("Station validation failed: {:?}", e,),
            });
        }
    }

    Ok(())
}

impl ModelValidator<UserError> for User {
    fn validate(&self) -> ModelValidatorResult<UserError> {
        if let UserSubscriptionStatus::Pending(email) = &self.subscription_status {
            validate_email(email)?;
        }
        validate_stations(&self.stations)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ic_stable_structures::Storable;
    use rstest::rstest;
    use tests::user_model_utils::mock_user;

    #[test]
    fn valid_model_serialization() {
        let model = mock_user();

        let serialized_model = model.to_bytes();
        let deserialized_model = User::from_bytes(serialized_model);

        assert_eq!(model.identity, deserialized_model.identity);
        assert_eq!(
            model.subscription_status,
            deserialized_model.subscription_status
        );
        assert_eq!(model.stations, deserialized_model.stations);
        assert_eq!(
            model.deployed_stations,
            deserialized_model.deployed_stations
        );
        assert_eq!(
            model.last_update_timestamp,
            deserialized_model.last_update_timestamp
        );
    }

    #[test]
    fn check_stations_validation() {
        let mut user = mock_user();
        user.stations = Vec::new();
        user.deployed_stations = Vec::new();

        let user_with_no_stations = user.clone();
        let mut user_with_one_station = user.clone();
        let mut user_with_too_many_stations = user.clone();

        user_with_one_station.stations.push(UserStation {
            canister_id: Principal::anonymous(),
            name: "main".to_string(),
            labels: Vec::new(),
        });

        for _ in 0..=User::MAX_STATIONS {
            user_with_too_many_stations.stations.push(UserStation {
                canister_id: Principal::anonymous(),
                name: "main".to_string(),
                labels: Vec::new(),
            });
        }

        assert!(validate_stations(&user_with_no_stations.stations).is_ok());
        assert!(validate_stations(&user_with_one_station.stations).is_ok());
        assert!(validate_stations(&user_with_too_many_stations.stations).is_err());
    }

    #[rstest]
    #[case::empty_name(&"")]
    #[case::invalid_email(&"john")]
    #[case::name_too_big(&"amkyMJuUzYRXmxJuyUFeetxXbkMKmfCBwQnSazukXXGuxmwXJEcxxSxAMqLzZWSzaYpdfKCnKDTjfrkfYvRhhmCrTrVmqUUkbgdMKufYuimeCebnHWgQXeSzkeqcFLqSVxpdNeSGADkpvvjZHCYXLmM")]
    fn invalid_email(#[case] email: &str) {
        assert!(validate_email(email).is_err());
    }
}

#[cfg(test)]
pub mod user_model_utils {
    use super::{User, UserSubscriptionStatus};
    use crate::core::test_utils;
    use uuid::Uuid;

    pub fn mock_user() -> User {
        User {
            id: *Uuid::new_v4().as_bytes(),
            identity: test_utils::random_principal(),
            subscription_status: UserSubscriptionStatus::Unsubscribed,
            stations: vec![],
            deployed_stations: vec![],
            last_active: 0,
            last_update_timestamp: 0,
        }
    }
}
