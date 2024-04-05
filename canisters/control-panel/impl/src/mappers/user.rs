use crate::core::ic_cdk::api::time;
use crate::{
    errors::UserError,
    models::{CanDeployWallet, User, UserSubscriptionStatus, UserWallet},
};
use candid::Principal;
use control_panel_api::{
    CanDeployWalletResponse, ManageUserInput, RegisterUserInput, SubscribedUserDTO, UserDTO, UserSubscriptionStatusDTO,
    UserWalletDTO,
};
use ic_canister_core::api::ApiError;

pub type SubscribedUser = SubscribedUserDTO;

#[derive(Default)]
pub struct UserMapper {}

impl UserMapper {
    /// Maps the registration input to an user entity.
    pub fn from_register_input(input: RegisterUserInput, user_id: Principal) -> User {
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

        User {
            id: user_id,
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
            last_update_timestamp: time(),
        }
    }
}

impl From<User> for UserDTO {
    fn from(user: User) -> Self {
        UserDTO {
            id: user.id,
            main_wallet: user.main_wallet,
            wallets: user.wallets.into_iter().map(UserWalletDTO::from).collect(),
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
        let user_id = Principal::from_slice(&[u8::MAX; 29]);
        let input = RegisterUserInput { wallet_id: None };

        let user = UserMapper::from_register_input(input, user_id);

        assert_eq!(user.id, user_id);
        assert_eq!(user.main_wallet, None);
        assert!(user.wallets.is_empty());
    }

    #[test]
    fn mapped_user_registration_with_wallet() {
        let user_id = Principal::from_slice(&[u8::MAX; 29]);
        let main_wallet = Principal::from_slice(&[2; 29]);
        let input = RegisterUserInput {
            wallet_id: Some(main_wallet),
        };

        let user = UserMapper::from_register_input(input, user_id);

        assert_eq!(user.id, user_id);
        assert_eq!(user.main_wallet, Some(main_wallet));
        assert_eq!(user.wallets.len(), 1);
        assert_eq!(user.wallets[0].canister_id, main_wallet);
        assert_eq!(user.wallets[0].name, None);
    }
}
