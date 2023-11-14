use crate::core::ic_cdk::api::time;
use crate::{
    errors::UserError,
    models::{User, UserWallet, UserId, UserIdentity},
    transport::{
        ManageUserInput, RegisterUserWalletInput, RegisterUserInput, UserWalletDTO, UserDTO,
        UserIdentityDTO,
    },
};
use candid::Principal;
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Default)]
pub struct UserMapper {}

impl UserMapper {
    /// Maps the registration input to an user entity.
    pub fn from_register_input(
        input: RegisterUserInput,
        user_id: UserId,
        identity: Principal,
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
            name: input.name,
            wallets: wallets
                .into_iter()
                .map(|canister_id| UserWallet {
                    canister_id,
                    name: None,
                })
                .collect(),
            unconfirmed_identities: vec![],
            identities: vec![UserIdentity {
                identity,
                name: None,
            }],
            last_update_timestamp: time(),
            main_wallet: Some(main_wallet),
        }
    }
}

impl From<User> for UserDTO {
    fn from(user: User) -> Self {
        UserDTO {
            id: Uuid::from_bytes(user.id).hyphenated().to_string(),
            name: user.name,
            main_wallet: user.main_wallet,
            wallets: user.wallets.into_iter().map(UserWalletDTO::from).collect(),
            identities: user
                .identities
                .into_iter()
                .map(UserIdentityDTO::from)
                .collect(),
            unconfirmed_identities: user
                .unconfirmed_identities
                .into_iter()
                .map(UserIdentityDTO::from)
                .collect(),
        }
    }
}

impl User {
    pub fn update_with(
        &mut self,
        input: ManageUserInput,
        caller_identity: &Principal,
    ) -> Result<(), UserError> {
        if let Some(new_identities) = input.identities {
            if !new_identities
                .iter()
                .any(|i| i.identity == *caller_identity)
            {
                Err(UserError::SelfLocked)?
            }

            let mut confirmed_identities: HashSet<UserIdentity> = HashSet::new();
            let mut unconfirmed_identities: HashSet<UserIdentity> = HashSet::new();
            for new_identity in &new_identities {
                match self
                    .identities
                    .iter()
                    .any(|i| i.identity == new_identity.identity)
                {
                    true => {
                        confirmed_identities.insert(UserIdentity::from(new_identity.clone()));
                    }
                    false => {
                        unconfirmed_identities.insert(UserIdentity::from(new_identity.clone()));
                    }
                }
            }

            self.identities = confirmed_identities.into_iter().collect();
            self.unconfirmed_identities = unconfirmed_identities.into_iter().collect();
        }

        if let Some(name) = input.name {
            self.name = Some(name);
        }

        if let Some(wallet) = input.main_wallet {
            self.main_wallet = Some(wallet);
        }

        if let Some(wallets) = input.wallets {
            self.wallets = wallets.iter().map(|b| UserWallet::from(b.clone())).collect();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::transport::RegisterUserWalletSharedInput;

    use super::*;

    #[test]
    fn mapped_user_registration_with_shared_wallet() {
        let user_id = *Uuid::new_v4().as_bytes();
        let identity = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();
        let global_shared_wallet_canister_id =
            Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let input = RegisterUserInput {
            name: Some("User".to_string()),
            wallet: RegisterUserWalletInput::SharedWallet,
        };

        let user = UserMapper::from_register_input(
            input,
            user_id,
            identity,
            global_shared_wallet_canister_id,
        );

        assert_eq!(user.id, user_id);
        assert_eq!(user.name, Some("User".to_string()));
        assert_eq!(user.main_wallet, Some(global_shared_wallet_canister_id));
        assert_eq!(user.wallets.len(), 1);
        assert_eq!(user.wallets[0].canister_id, global_shared_wallet_canister_id);
        assert_eq!(user.wallets[0].name, None);
        assert_eq!(user.identities.len(), 1);
        assert_eq!(user.identities[0].identity, identity);
        assert_eq!(user.identities[0].name, None);
        assert_eq!(user.unconfirmed_identities.len(), 0);
    }

    #[test]
    fn mapped_user_registration_with_private_wallet() {
        let user_id = *Uuid::new_v4().as_bytes();
        let identity = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();
        let global_shared_wallet_canister_id = Principal::anonymous();
        let main_wallet = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let input = RegisterUserInput {
            name: Some("User".to_string()),
            wallet: RegisterUserWalletInput::PrivateWallet {
                id: main_wallet,
                use_shared_wallet: None,
            },
        };

        let user = UserMapper::from_register_input(
            input,
            user_id,
            identity,
            global_shared_wallet_canister_id,
        );

        assert_eq!(user.id, user_id);
        assert_eq!(user.name, Some("User".to_string()));
        assert_eq!(user.main_wallet, Some(main_wallet));
        assert_eq!(user.wallets.len(), 1);
        assert_eq!(user.wallets[0].canister_id, main_wallet);
        assert_eq!(user.wallets[0].name, None);
        assert_eq!(user.identities.len(), 1);
        assert_eq!(user.identities[0].identity, identity);
        assert_eq!(user.identities[0].name, None);
        assert_eq!(user.unconfirmed_identities.len(), 0);
    }

    #[test]
    fn mapped_user_registration_with_private_wallet_and_shared() {
        let user_id = *Uuid::new_v4().as_bytes();
        let identity = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();
        let global_shared_wallet_canister_id = Principal::anonymous();
        let main_wallet = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let input = RegisterUserInput {
            name: Some("User".to_string()),
            wallet: RegisterUserWalletInput::PrivateWallet {
                id: main_wallet,
                use_shared_wallet: Some(RegisterUserWalletSharedInput { is_main: false }),
            },
        };

        let user = UserMapper::from_register_input(
            input,
            user_id,
            identity,
            global_shared_wallet_canister_id,
        );

        assert_eq!(user.id, user_id);
        assert_eq!(user.name, Some("User".to_string()));
        assert_eq!(user.main_wallet, Some(main_wallet));
        assert_eq!(user.wallets.len(), 2);
        assert_eq!(user.wallets[0].canister_id, main_wallet);
        assert_eq!(user.wallets[0].name, None);
        assert_eq!(user.wallets[1].canister_id, global_shared_wallet_canister_id);
        assert_eq!(user.wallets[1].name, None);
        assert_eq!(user.identities.len(), 1);
        assert_eq!(user.identities[0].identity, identity);
        assert_eq!(user.identities[0].name, None);
        assert_eq!(user.unconfirmed_identities.len(), 0);
    }
}
