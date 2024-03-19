use crate::core::ic_cdk::api::time;
use crate::{
    errors::UserError,
    models::{User, UserAuthorizationStatus, UserWallet},
};
use candid::Principal;
use control_panel_api::{ManageUserInput, RegisterUserInput, UserDTO, UserWalletDTO};

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
            email: input.email,
            authorization_status: UserAuthorizationStatus::Unauthorized,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mapped_user_registration_with_no_wallet() {
        let user_id = Principal::from_slice(&[u8::MAX; 29]);
        let input = RegisterUserInput {
            wallet_id: None,
            email: None,
        };

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
            email: Some("john@example.com".to_string()),
        };

        let user = UserMapper::from_register_input(input, user_id);

        assert_eq!(user.id, user_id);
        assert_eq!(user.main_wallet, Some(main_wallet));
        assert_eq!(user.wallets.len(), 1);
        assert_eq!(user.wallets[0].canister_id, main_wallet);
        assert_eq!(user.wallets[0].name, None);
    }
}
