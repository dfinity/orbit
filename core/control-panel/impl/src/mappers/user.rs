use crate::{
    core::ic_cdk::next_time,
    errors::UserError,
    models::{CanDeployStation, User, UserSubscriptionStatus},
};
use candid::Principal;
use control_panel_api::{
    CanDeployStationResponse, RegisterUserInput, SubscribedUserDTO, UserDTO,
    UserSubscriptionStatusDTO,
};
use orbit_essentials::api::ApiError;
use orbit_essentials::types::UUID;
use orbit_essentials::utils::timestamp_to_rfc3339;

pub type SubscribedUser = SubscribedUserDTO;

#[derive(Default)]
pub struct UserMapper {}

impl UserMapper {
    /// Maps the registration input to an user entity.
    pub fn from_register_input(
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
            last_active: registration_time,
            last_update_timestamp: registration_time,
        }
    }
}

impl From<User> for UserDTO {
    fn from(user: User) -> Self {
        UserDTO {
            identity: user.identity,
            subscription_status: user.subscription_status.into(),
            last_active: timestamp_to_rfc3339(&user.last_active),
        }
    }
}

impl From<UserSubscriptionStatus> for UserSubscriptionStatusDTO {
    fn from(authorization_status: UserSubscriptionStatus) -> Self {
        match authorization_status {
            UserSubscriptionStatus::Unsubscribed => UserSubscriptionStatusDTO::Unsubscribed,
            UserSubscriptionStatus::Pending(_) => UserSubscriptionStatusDTO::Pending,
            UserSubscriptionStatus::Approved => UserSubscriptionStatusDTO::Approved,
            UserSubscriptionStatus::Denylisted => UserSubscriptionStatusDTO::Denylisted,
        }
    }
}

impl TryFrom<UserSubscriptionStatusDTO> for UserSubscriptionStatus {
    type Error = ApiError;

    fn try_from(authorization_status: UserSubscriptionStatusDTO) -> Result<Self, Self::Error> {
        match authorization_status {
            UserSubscriptionStatusDTO::Unsubscribed => Ok(UserSubscriptionStatus::Unsubscribed),
            UserSubscriptionStatusDTO::Pending => Err(UserError::ValidationError {
                info: "Invalid user subscription status: Pending.".to_string(),
            }
            .into()),
            UserSubscriptionStatusDTO::Approved => Ok(UserSubscriptionStatus::Approved),
            UserSubscriptionStatusDTO::Denylisted => Ok(UserSubscriptionStatus::Denylisted),
        }
    }
}

impl From<CanDeployStation> for CanDeployStationResponse {
    fn from(can_deploy_station: CanDeployStation) -> Self {
        match can_deploy_station {
            CanDeployStation::NotAllowed(user_subscription_status) => {
                CanDeployStationResponse::NotAllowed(user_subscription_status.into())
            }
            CanDeployStation::Allowed(remaining_stations) => {
                CanDeployStationResponse::Allowed(remaining_stations)
            }
            CanDeployStation::QuotaExceeded => CanDeployStationResponse::QuotaExceeded,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use control_panel_api::UserStationDTO;

    #[test]
    fn mapped_user_registration_with_no_station() {
        let user_id = [u8::MAX; 16];
        let user_identity = Principal::from_slice(&[u8::MAX; 29]);
        let input = RegisterUserInput { station: None };

        let user = UserMapper::from_register_input(user_id, input, user_identity);

        assert_eq!(user.id, user_id);
        assert_eq!(user.identity, user_identity);
        assert!(user.stations.is_empty());
    }

    #[test]
    fn mapped_user_registration_with_station() {
        let user_id = [u8::MAX; 16];
        let user_identity = Principal::from_slice(&[u8::MAX; 29]);
        let main_station = Principal::from_slice(&[2; 29]);
        let input: RegisterUserInput = RegisterUserInput {
            station: Some(UserStationDTO {
                canister_id: main_station,
                name: "Main Station".to_string(),
                labels: Vec::new(),
            }),
        };

        let user = UserMapper::from_register_input(user_id, input, user_identity);

        assert_eq!(user.id, user_id);
        assert_eq!(user.identity, user_identity);
        assert_eq!(user.stations.len(), 1);
        assert_eq!(user.stations[0].canister_id, main_station);
        assert_eq!(user.stations[0].name, "Main Station".to_string());
    }
}
