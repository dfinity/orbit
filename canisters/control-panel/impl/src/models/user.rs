use super::UserWallet;
use crate::errors::UserError;
use candid::Principal;
use email_address::EmailAddress;
use ic_canister_core::{
    model::{ModelValidator, ModelValidatorResult},
    types::Timestamp,
};
use ic_canister_macros::storable;
use std::str::FromStr;

/// The subscription status of an user.
#[storable(serializer = "candid")]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UserSubscriptionStatus {
    Unsubscribed,
    Pending(String), // e-mail address to push notification to
    Approved,
    Denylisted,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CanDeployWallet {
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
#[storable(serializer = "candid")]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct User {
    /// The UUID that identifies the user.
    pub identity: Principal,
    /// The subscription status of the user.
    pub subscription_status: UserSubscriptionStatus,
    /// All the wallets that the user has access to (including the main wallet).
    ///
    /// The user can optionally give a name to each wallet to make it easier to identify them.
    pub wallets: Vec<UserWallet>,
    /// The wallets that have ever been deployed for the user by the control panel.
    /// Used to bound the total number of wallets a user could deploy via the control panel.
    pub deployed_wallets: Vec<Principal>,
    /// The main wallet to use for the user, this is the wallet that will be used by default.
    pub main_wallet: Option<Principal>,
    /// Last time the identity was updated.
    pub last_update_timestamp: Timestamp,
}

#[storable(serializer = "candid")]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UserKey(pub Principal);

impl User {
    pub const NAME_LEN_RANGE: (u8, u8) = (1, 100);
    pub const EMAIL_LEN_RANGE: (u8, u8) = (1, 100);
    pub const MAX_WALLETS: u8 = 10;
    pub const MAX_DEPLOYED_WALLETS: u8 = 10;

    pub fn to_key(&self) -> UserKey {
        UserKey(self.identity)
    }

    pub fn can_deploy_wallet(&self) -> CanDeployWallet {
        match self.subscription_status {
            UserSubscriptionStatus::Approved => (),
            UserSubscriptionStatus::Unsubscribed
            | UserSubscriptionStatus::Pending(_)
            | UserSubscriptionStatus::Denylisted => {
                return CanDeployWallet::NotAllowed(self.subscription_status.clone());
            }
        };
        let max_deployed_wallets: usize = Self::MAX_DEPLOYED_WALLETS.into();
        if self.deployed_wallets.len() >= max_deployed_wallets {
            return CanDeployWallet::QuotaExceeded;
        }
        CanDeployWallet::Allowed(max_deployed_wallets - self.deployed_wallets.len())
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

    for wallet in wallets.iter() {
        if let Err(e) = wallet.validate() {
            return Err(UserError::ValidationError {
                info: format!("Wallet validation failed: {:?}", e,),
            });
        }
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
        if let UserSubscriptionStatus::Pending(email) = &self.subscription_status {
            validate_email(email)?;
        }
        validate_wallets(&self.wallets)?;
        validate_main_wallet(&self.main_wallet, &self.wallets)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ic_stable_structures::Storable;
    use rstest::rstest;

    #[test]
    fn valid_model_serialization() {
        let model = User {
            identity: Principal::from_slice(&[u8::MAX; 29]),
            subscription_status: UserSubscriptionStatus::Unsubscribed,
            wallets: vec![],
            deployed_wallets: vec![],
            main_wallet: None,
            last_update_timestamp: 10,
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = User::from_bytes(serialized_model);

        assert_eq!(model.identity, deserialized_model.identity);
        assert_eq!(
            model.subscription_status,
            deserialized_model.subscription_status
        );
        assert_eq!(model.wallets, deserialized_model.wallets);
        assert_eq!(model.deployed_wallets, deserialized_model.deployed_wallets);
        assert_eq!(model.main_wallet, deserialized_model.main_wallet);
        assert_eq!(
            model.last_update_timestamp,
            deserialized_model.last_update_timestamp
        );
    }

    #[test]
    fn check_wallets_validation() {
        let user = User {
            identity: Principal::from_slice(&[u8::MAX; 29]),
            subscription_status: UserSubscriptionStatus::Unsubscribed,
            wallets: vec![],
            deployed_wallets: vec![],
            main_wallet: None,
            last_update_timestamp: 10,
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
            identity: Principal::from_slice(&[u8::MAX; 29]),
            subscription_status: UserSubscriptionStatus::Unsubscribed,
            wallets: vec![UserWallet {
                canister_id: Principal::anonymous(),
                name: None,
            }],
            deployed_wallets: vec![],
            main_wallet: Some(Principal::anonymous()),
            last_update_timestamp: 0,
        };

        assert!(validate_main_wallet(&user.main_wallet, &user.wallets).is_ok());
    }

    #[test]
    fn invalid_main_wallet() {
        let user = User {
            identity: Principal::from_slice(&[u8::MAX; 29]),
            subscription_status: UserSubscriptionStatus::Unsubscribed,
            wallets: vec![UserWallet {
                canister_id: Principal::from_text("wkt3w-3iaaa-aaaaa-774ba-cai").unwrap(),
                name: None,
            }],
            deployed_wallets: vec![],
            main_wallet: Some(Principal::anonymous()),
            last_update_timestamp: 0,
        };

        assert!(validate_main_wallet(&user.main_wallet, &user.wallets).is_err());
    }

    #[rstest]
    #[case::empty_name(&"")]
    #[case::invalid_email(&"john")]
    #[case::name_too_big(&"amkyMJuUzYRXmxJuyUFeetxXbkMKmfCBwQnSazukXXGuxmwXJEcxxSxAMqLzZWSzaYpdfKCnKDTjfrkfYvRhhmCrTrVmqUUkbgdMKufYuimeCebnHWgQXeSzkeqcFLqSVxpdNeSGADkpvvjZHCYXLmM")]
    fn invalid_email(#[case] email: &str) {
        assert!(validate_email(email).is_err());
    }
}
