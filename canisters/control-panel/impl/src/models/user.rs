use super::UserWallet;
use crate::errors::UserError;
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_canister_core::{
    model::{ModelValidator, ModelValidatorResult},
    types::Timestamp,
};
use ic_canister_macros::stable_object;

/// The identity of an user.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct User {
    /// The UUID that identifies the user.
    pub id: Principal,
    /// The main wallet to use for the user, this is the wallet that will be used by default.
    pub main_wallet: Option<Principal>,
    /// All the wallets that the user has access to (including the main wallet).
    ///
    /// The user can optionally give a name to each wallet to make it easier to identify them.
    pub wallets: Vec<UserWallet>,
    /// Last time the identity was updated.
    pub last_update_timestamp: Timestamp,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UserKey(pub Principal);

impl User {
    pub const NAME_LEN_RANGE: (u8, u8) = (1, 100);
    pub const MAX_WALLETS: u8 = 10;

    pub fn to_key(&self) -> UserKey {
        UserKey(self.id)
    }
}

fn validate_wallets(wallets: &[UserWallet]) -> ModelValidatorResult<UserError> {
    if wallets.len() > User::MAX_WALLETS as usize {
        return Err(UserError::ValidationError {
            info: format!(
                "Too many wallets, expected at most {} but got {}",
                User::MAX_WALLETS,
                wallets.len()
            ),
        });
    }

    Ok(())
}

fn validate_main_wallet(
    main_wallet: &Option<Principal>,
    wallets: &Vec<UserWallet>,
) -> ModelValidatorResult<UserError> {
    if let Some(main_wallet) = main_wallet {
        if !wallets
            .iter()
            .any(|wallet| &wallet.canister_id == main_wallet)
        {
            return Err(UserError::ValidationError {
                info: format!(
                    "Main wallet {} is not in the list of wallets {:?}",
                    main_wallet, wallets
                ),
            });
        }
    }

    Ok(())
}

impl ModelValidator<UserError> for User {
    fn validate(&self) -> ModelValidatorResult<UserError> {
        validate_wallets(&self.wallets)?;
        validate_main_wallet(&self.main_wallet, &self.wallets)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ic_stable_structures::Storable;

    #[test]
    fn valid_model_serialization() {
        let model = User {
            id: Principal::from_slice(&[u8::MAX; 29]),
            wallets: vec![],
            main_wallet: None,
            last_update_timestamp: 10,
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = User::from_bytes(serialized_model);

        assert_eq!(model.id, deserialized_model.id);
        assert_eq!(
            model.last_update_timestamp,
            deserialized_model.last_update_timestamp
        );
    }

    #[test]
    fn check_wallets_validation() {
        let user = User {
            id: Principal::from_slice(&[u8::MAX; 29]),
            wallets: vec![],
            main_wallet: None,
            last_update_timestamp: 0,
        };

        let user_with_no_wallets = user.clone();
        let mut user_with_one_wallet = user.clone();
        let mut user_with_too_many_wallets = user.clone();

        user_with_one_wallet.wallets.push(UserWallet {
            canister_id: Principal::anonymous(),
            name: None,
        });

        for _ in 0..=User::MAX_WALLETS {
            user_with_too_many_wallets.wallets.push(UserWallet {
                canister_id: Principal::anonymous(),
                name: None,
            });
        }

        assert!(validate_wallets(&user_with_no_wallets.wallets).is_ok());
        assert!(validate_wallets(&user_with_one_wallet.wallets).is_ok());
        assert!(validate_wallets(&user_with_too_many_wallets.wallets).is_err());
    }

    #[test]
    fn valid_main_wallet() {
        let user = User {
            id: Principal::from_slice(&[u8::MAX; 29]),
            wallets: vec![UserWallet {
                canister_id: Principal::anonymous(),
                name: None,
            }],
            main_wallet: Some(Principal::anonymous()),
            last_update_timestamp: 0,
        };

        assert!(validate_main_wallet(&user.main_wallet, &user.wallets).is_ok());
    }

    #[test]
    fn invalid_main_wallet() {
        let user = User {
            id: Principal::from_slice(&[u8::MAX; 29]),
            wallets: vec![UserWallet {
                canister_id: Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
                name: None,
            }],
            main_wallet: Some(Principal::anonymous()),
            last_update_timestamp: 0,
        };

        assert!(validate_main_wallet(&user.main_wallet, &user.wallets).is_err());
    }
}
