use crate::core::ic_cdk::api::time;
use crate::{
    errors::UserError,
    models::{User, UserWallet},
};
use candid::Principal;
use control_panel_api::{
    ManageUserInput, RegisterUserInput, RegisterUserWalletInput, UserDTO, UserWalletDTO,
};

#[derive(Default)]
pub struct UserMapper {}

impl UserMapper {
    /// Maps the registration input to an user entity.
    pub fn from_register_input(
        input: RegisterUserInput,
        user_id: Principal,
        global_shared_wallet_canister_id: Principal,
    ) -> User {
        let wallets = match input.wallet {
            RegisterUserWalletInput::PrivateWallet {
                id,
                use_shared_wallet,
            } => match use_shared_wallet {
                Some(shared_wallet) => {
                    if shared_wallet.is_main {
                        vec![global_shared_wallet_canister_id, id]
                    } else {
                        vec![id, global_shared_wallet_canister_id]
                    }
                }
                None => vec![id],
            },
            RegisterUserWalletInput::SharedWallet => {
                vec![global_shared_wallet_canister_id]
            }
        };
        // The order of the wallets is important, the first wallet is the main wallet for the user at this stage
        // so that it can be used to the `main_wallet` field of the user entity.
        let main_wallet = *wallets.first().unwrap();

        User {
            id: user_id,
            wallets: wallets
                .into_iter()
                .map(|canister_id| UserWallet {
                    canister_id,
                    name: None,
                })
                .collect(),
            last_update_timestamp: time(),
            main_wallet: Some(main_wallet),
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
    use control_panel_api::RegisterUserWalletSharedInput;

    use super::*;

    #[test]
    fn mapped_user_registration_with_shared_wallet() {
        let user_id = Principal::from_slice(&[u8::MAX; 29]);
        let global_shared_wallet_canister_id =
            Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let input = RegisterUserInput {
            wallet: RegisterUserWalletInput::SharedWallet,
        };

        let user =
            UserMapper::from_register_input(input, user_id, global_shared_wallet_canister_id);

        assert_eq!(user.id, user_id);
        assert_eq!(user.main_wallet, Some(global_shared_wallet_canister_id));
        assert_eq!(user.wallets.len(), 1);
        assert_eq!(
            user.wallets[0].canister_id,
            global_shared_wallet_canister_id
        );
        assert_eq!(user.wallets[0].name, None);
    }

    #[test]
    fn mapped_user_registration_with_private_wallet() {
        let user_id = Principal::from_slice(&[u8::MAX; 29]);
        let global_shared_wallet_canister_id = Principal::anonymous();
        let main_wallet = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let input = RegisterUserInput {
            wallet: RegisterUserWalletInput::PrivateWallet {
                id: main_wallet,
                use_shared_wallet: None,
            },
        };

        let user =
            UserMapper::from_register_input(input, user_id, global_shared_wallet_canister_id);

        assert_eq!(user.id, user_id);
        assert_eq!(user.main_wallet, Some(main_wallet));
        assert_eq!(user.wallets.len(), 1);
        assert_eq!(user.wallets[0].canister_id, main_wallet);
        assert_eq!(user.wallets[0].name, None);
    }

    #[test]
    fn mapped_user_registration_with_private_wallet_and_shared() {
        let user_id = Principal::from_slice(&[u8::MAX; 29]);
        let global_shared_wallet_canister_id = Principal::anonymous();
        let main_wallet = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let input = RegisterUserInput {
            wallet: RegisterUserWalletInput::PrivateWallet {
                id: main_wallet,
                use_shared_wallet: Some(RegisterUserWalletSharedInput { is_main: false }),
            },
        };

        let user =
            UserMapper::from_register_input(input, user_id, global_shared_wallet_canister_id);

        assert_eq!(user.id, user_id);
        assert_eq!(user.main_wallet, Some(main_wallet));
        assert_eq!(user.wallets.len(), 2);
        assert_eq!(user.wallets[0].canister_id, main_wallet);
        assert_eq!(user.wallets[0].name, None);
        assert_eq!(
            user.wallets[1].canister_id,
            global_shared_wallet_canister_id
        );
        assert_eq!(user.wallets[1].name, None);
    }
}
