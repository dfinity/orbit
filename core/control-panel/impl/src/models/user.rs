use super::UserStation;
use crate::core::ic_cdk::api::time;
use crate::core::ic_cdk::next_time;
use crate::errors::UserError;
use candid::Principal;
use control_panel_api::RegisterUserInput;
use email_address::EmailAddress;
use orbit_essentials::model::ModelKey;
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

#[storable]
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct UserRateLimiter {
    pub unix_date: u64,
    pub num_deployed_stations: usize,
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
    deployed_stations: Vec<Principal>,
    /// Used to rate limit the number of deployed stations per user by the control panel.
    #[serde(default)]
    user_rate_limiter: UserRateLimiter,
    /// The timestamp of last time the user was active.
    pub last_active: Timestamp,
    /// Last time the identity was updated.
    pub last_update_timestamp: Timestamp,
}

#[storable]
#[derive(Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd)]
pub struct UserKey(pub UUID);

impl ModelKey<UserKey> for User {
    fn key(&self) -> UserKey {
        UserKey(self.id)
    }
}

impl User {
    pub const NAME_LEN_RANGE: (u8, u8) = (1, 100);
    pub const EMAIL_LEN_RANGE: (u8, u8) = (1, 100);
    pub const MAX_STATIONS: u8 = 15;
    pub const MAX_DEPLOYED_STATIONS_PER_DAY: usize = 2;

    pub fn to_key(&self) -> UserKey {
        UserKey(self.id)
    }

    pub fn get_num_deployed_stations(&self) -> usize {
        self.deployed_stations.len()
    }

    pub fn get_deployed_stations(&self) -> Vec<Principal> {
        self.deployed_stations.clone()
    }

    pub fn add_deployed_station(&mut self, station: Principal) {
        self.deployed_stations.push(station);
        let current_unix_date = time() / 86_400_000_000_000;
        if self.user_rate_limiter.unix_date == current_unix_date {
            self.user_rate_limiter.num_deployed_stations += 1;
        } else {
            self.user_rate_limiter.unix_date = current_unix_date;
            self.user_rate_limiter.num_deployed_stations = 1;
        }
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
        let current_unix_date = time() / 86_400_000_000_000;
        if self.user_rate_limiter.unix_date == current_unix_date {
            if self.user_rate_limiter.num_deployed_stations >= Self::MAX_DEPLOYED_STATIONS_PER_DAY {
                CanDeployStation::QuotaExceeded
            } else {
                CanDeployStation::Allowed(
                    Self::MAX_DEPLOYED_STATIONS_PER_DAY
                        - self.user_rate_limiter.num_deployed_stations,
                )
            }
        } else {
            CanDeployStation::Allowed(Self::MAX_DEPLOYED_STATIONS_PER_DAY)
        }
    }

    pub fn new_from_register_input(
        new_user_id: UUID,
        input: RegisterUserInput,
        user_identity: Principal,
    ) -> User {
        let registration_time = next_time();
        let stations = match input.station {
            Some(station) => vec![station],
            None => vec![],
        };

        User {
            id: new_user_id,
            identity: user_identity,
            subscription_status: UserSubscriptionStatus::Unsubscribed,
            stations: stations.into_iter().map(|station| station.into()).collect(),
            deployed_stations: vec![],
            user_rate_limiter: UserRateLimiter::default(),
            last_active: registration_time,
            last_update_timestamp: registration_time,
        }
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
    use crate::models::UserRateLimiter;
    use uuid::Uuid;

    pub fn mock_user() -> User {
        User {
            id: *Uuid::new_v4().as_bytes(),
            identity: test_utils::random_principal(),
            subscription_status: UserSubscriptionStatus::Unsubscribed,
            stations: vec![],
            deployed_stations: vec![],
            user_rate_limiter: UserRateLimiter::default(),
            last_active: 0,
            last_update_timestamp: 0,
        }
    }
}
