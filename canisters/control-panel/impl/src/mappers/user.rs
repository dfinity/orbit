use crate::{
    core::ic_cdk::next_time,
    errors::UserError,
    models::{CanDeployWallet, User, UserLastActiveIntervals, UserSubscriptionStatus, UserWallet},
};
use candid::Principal;
use control_panel_api::{
    CanDeployWalletResponse, ManageUserInput, RegisterUserInput, SubscribedUserDTO, UserDTO,
    UserSubscriptionStatusDTO, UserWalletDTO,
};
use ic_canister_core::api::ApiError;
use ic_canister_core::types::UUID;
use ic_canister_core::utils::timestamp_to_rfc3339;

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
        let wallets = match input.wallet_id {
            Some(wallet_id) => vec![wallet_id],
            None => vec![],
        };
        // The order of the wallets is important, the first wallet is the main wallet for the user at this stage
        // so that it can be used to the `main_wallet` field of the user entity.
        let main_wallet = match wallets.is_empty() {
            true => None,
            false => Some(wallets[0]),
        };

        let registration_time = next_time();

        User {
            id: new_user_id,
            identity: user_identity,
            subscription_status: UserSubscriptionStatus::Unsubscribed,
            wallets: wallets
                .into_iter()
                .map(|canister_id| UserWallet {
                    canister_id,
                    name: None,
                })
                .collect(),
            deployed_wallets: vec![],
            main_wallet,
            last_active: registration_time,
            last_active_intervals: UserLastActiveIntervals {
                daily: registration_time,
                hourly: registration_time,
                monthly: registration_time,
                weekly: registration_time,
            },
            last_update_timestamp: registration_time,
        }
    }
}

impl From<User> for UserDTO {
    fn from(user: User) -> Self {
        UserDTO {
            identity: user.identity,
            main_wallet: user.main_wallet,
            wallets: user.wallets.into_iter().map(UserWalletDTO::from).collect(),
            subscription_status: user.subscription_status.into(),
            last_active: timestamp_to_rfc3339(&user.last_active),
        }
    }
}

impl User {
    pub fn update_with(&mut self, input: ManageUserInput) -> Result<(), UserError> {
        if let Some(wallet) = input.main_wallet {
            self.main_wallet = Some(wallet);
        }

        if let Some(wallets) = input.wallets {
            self.wallets = wallets
                .iter()
                .map(|b| UserWallet::from(b.clone()))
                .collect();
        }

        Ok(())
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

impl From<CanDeployWallet> for CanDeployWalletResponse {
    fn from(can_deploy_wallet: CanDeployWallet) -> Self {
        match can_deploy_wallet {
            CanDeployWallet::NotAllowed(user_subscription_status) => {
                CanDeployWalletResponse::NotAllowed(user_subscription_status.into())
            }
            CanDeployWallet::Allowed(remaining_wallets) => {
                CanDeployWalletResponse::Allowed(remaining_wallets)
            }
            CanDeployWallet::QuotaExceeded => CanDeployWalletResponse::QuotaExceeded,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mapped_user_registration_with_no_wallet() {
        let user_id = [u8::MAX; 16];
        let user_identity = Principal::from_slice(&[u8::MAX; 29]);
        let input = RegisterUserInput { wallet_id: None };

        let user = UserMapper::from_register_input(user_id, input, user_identity);

        assert_eq!(user.id, user_id);
        assert_eq!(user.identity, user_identity);
        assert_eq!(user.main_wallet, None);
        assert!(user.wallets.is_empty());
    }

    #[test]
    fn mapped_user_registration_with_wallet() {
        let user_id = [u8::MAX; 16];
        let user_identity = Principal::from_slice(&[u8::MAX; 29]);
        let main_wallet = Principal::from_slice(&[2; 29]);
        let input = RegisterUserInput {
            wallet_id: Some(main_wallet),
        };

        let user = UserMapper::from_register_input(user_id, input, user_identity);

        assert_eq!(user.id, user_id);
        assert_eq!(user.identity, user_identity);
        assert_eq!(user.main_wallet, Some(main_wallet));
        assert_eq!(user.wallets.len(), 1);
        assert_eq!(user.wallets[0].canister_id, main_wallet);
        assert_eq!(user.wallets[0].name, None);
    }
}
