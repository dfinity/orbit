use crate::{
    errors::UserError,
    models::{CanDeployStation, User, UserSubscriptionStatus},
};
use control_panel_api::{
    CanDeployStationResponse, SubscribedUserDTO, UserDTO, UserSubscriptionStatusDTO,
};
use orbit_essentials::api::ApiError;
use orbit_essentials::utils::timestamp_to_rfc3339;

pub type SubscribedUser = SubscribedUserDTO;

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
            CanDeployStation::Allowed => CanDeployStationResponse::Allowed,
            CanDeployStation::QuotaExceeded => CanDeployStationResponse::QuotaExceeded,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;
    use control_panel_api::{RegisterUserInput, UserStationDTO};

    #[test]
    fn mapped_user_registration_with_no_station() {
        let user_id = [u8::MAX; 16];
        let user_identity = Principal::from_slice(&[u8::MAX; 29]);
        let input = RegisterUserInput { station: None };

        let user = User::new_from_register_input(user_id, input, user_identity);

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

        let user = User::new_from_register_input(user_id, input, user_identity);

        assert_eq!(user.id, user_id);
        assert_eq!(user.identity, user_identity);
        assert_eq!(user.stations.len(), 1);
        assert_eq!(user.stations[0].canister_id, main_station);
        assert_eq!(user.stations[0].name, "Main Station".to_string());
    }
}
