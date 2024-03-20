use std::sync::Arc;

use crate::{
    core::CallContext,
    errors::UserError,
    mappers::UserMapper,
    models::{User, UserKey, UserWallet},
    repositories::{UserRepository, USER_REPOSITORY},
};
use candid::Principal;
use control_panel_api::{ManageUserInput, RegisterUserInput};
use ic_canister_core::repository::Repository;
use ic_canister_core::{
    api::{ApiError, ServiceResult},
    model::ModelValidator,
};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref USER_SERVICE: Arc<UserService> =
        Arc::new(UserService::new(Arc::clone(&USER_REPOSITORY)));
}

#[derive(Default, Debug)]
pub struct UserService {
    user_repository: Arc<UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        Self { user_repository }
    }

    /// Returns the user associated with the given user id.
    pub fn get_user(&self, user_id: &Principal, ctx: &CallContext) -> ServiceResult<User> {
        let user = self
            .user_repository
            .get(&UserKey(*user_id))
            .ok_or(UserError::NotFound {
                user: user_id.to_text(),
            })?;

        self.assert_user_access(&user, ctx)?;

        Ok(user)
    }

    pub fn get_main_wallet(&self, ctx: &CallContext) -> ServiceResult<Option<UserWallet>> {
        let user = self.get_user(&ctx.caller(), ctx)?;

        match user.main_wallet {
            Some(main_wallet) => {
                let main_wallet = user
                    .wallets
                    .into_iter()
                    .find(|wallet| wallet.canister_id == main_wallet)
                    .ok_or(UserError::MainWalletNotFound)?;

                Ok(Some(main_wallet))
            }
            None => Ok(None),
        }
    }

    /// Registers a new user for the caller identity.
    pub async fn register_user(
        &self,
        input: RegisterUserInput,
        ctx: &CallContext,
    ) -> ServiceResult<User, ApiError> {
        self.assert_identity_is_unregistered(&ctx.caller())?;

        let user_id = ctx.caller();
        let user = UserMapper::from_register_input(input.clone(), user_id);

        user.validate()?;
        self.user_repository.insert(UserKey(user.id), user.clone());

        Ok(user)
    }

    pub async fn remove_user(&self, user_id: &Principal, ctx: &CallContext) -> ServiceResult<User> {
        let user = self.get_user(user_id, ctx)?;

        self.assert_user_access(&user, ctx)?;

        self.user_repository.remove(&user.to_key());

        Ok(user)
    }

    pub async fn manage_user(
        &self,
        input: ManageUserInput,
        ctx: &CallContext,
    ) -> ServiceResult<User> {
        let mut user = self.get_user(&ctx.caller(), ctx)?;

        user.update_with(input)?;
        user.validate()?;

        self.user_repository.insert(user.to_key(), user.clone());

        Ok(user)
    }

    pub async fn add_deployed_wallet(
        &self,
        wallet_canister_id: Principal,
        ctx: &CallContext,
    ) -> ServiceResult<User> {
        let mut user = self.get_user(&ctx.caller(), ctx)?;

        user.deployed_wallets.push(wallet_canister_id);

        user.validate()?;

        self.user_repository.insert(user.to_key(), user.clone());

        Ok(user)
    }

    /// Checks if the caller has access to the given user.
    ///
    /// Admins have access to all users.
    fn assert_user_access(&self, user: &User, ctx: &CallContext) -> ServiceResult<()> {
        let is_user_owner = user.id == ctx.caller();
        if !is_user_owner && !ctx.is_admin() {
            Err(UserError::Forbidden {
                user: user.id.to_text(),
            })?
        }

        Ok(())
    }

    /// Validates that the given identity has no associated user.
    ///
    /// If the identity has an associated user, an error is returned.
    pub fn assert_identity_is_unregistered(&self, identity: &Principal) -> ServiceResult<()> {
        let maybe_user = self.user_repository.get(&UserKey(*identity));

        if let Some(user) = maybe_user {
            Err(UserError::IdentityAlreadyHasUser {
                user: user.id.to_text(),
            })?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::UserAuthorizationStatus;

    #[test]
    fn get_user_returns_not_found_err() {
        let ctx = CallContext::default();
        let service = UserService::default();
        let user_id = Principal::from_slice(&[u8::MAX; 29]);

        let result = service.get_user(&user_id, &ctx);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ApiError::from(UserError::NotFound {
                user: user_id.to_text()
            })
        );
    }

    #[test]
    fn success_fetch_existing_user() {
        let user_id = Principal::from_slice(&[u8::MAX; 29]);
        let ctx = CallContext::new(user_id);
        let service = UserService::default();
        let user = User {
            id: user_id,
            email: Some("john@example.com".to_string()),
            authorization_status: UserAuthorizationStatus::Unauthorized,
            wallets: vec![],
            deployed_wallets: vec![],
            main_wallet: None,
            last_update_timestamp: 0,
        };

        service.user_repository.insert(user.to_key(), user.clone());

        let result = service.get_user(&user_id, &ctx);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), user);
    }

    #[test]
    fn success_fetch_existing_user_with_identity() {
        let user_id = Principal::from_slice(&[u8::MAX; 29]);
        let ctx = CallContext::new(user_id);
        let service = UserService::default();
        let user = User {
            id: user_id,
            email: Some("john@example.com".to_string()),
            authorization_status: UserAuthorizationStatus::Unauthorized,
            wallets: vec![],
            deployed_wallets: vec![],
            main_wallet: None,
            last_update_timestamp: 0,
        };

        service.user_repository.insert(user.to_key(), user.clone());

        let result = service.get_user(&user_id, &ctx);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), user);
    }

    #[tokio::test]
    async fn success_register_new_user() {
        crate::core::test_utils::init_canister_config();

        let ctx = CallContext::default();
        let service = UserService::default();
        let input = RegisterUserInput {
            wallet_id: Some(Principal::from_slice(&[2; 29])),
            email: Some("john@example.com".to_string()),
        };

        let result = service.register_user(input.clone(), &ctx).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn failed_registering_new_user_with_same_identity() {
        crate::core::test_utils::init_canister_config();

        let ctx = CallContext::default();
        let service = UserService::default();
        let input = RegisterUserInput {
            wallet_id: None,
            email: Some("john@example.com".to_string()),
        };
        let duplicated_user_input = RegisterUserInput {
            wallet_id: None,
            email: Some("john@example.com".to_string()),
        };

        let result = service.register_user(input.clone(), &ctx).await;
        let duplicated_user_result = service
            .register_user(duplicated_user_input.clone(), &ctx)
            .await;

        assert!(result.is_ok());
        assert!(duplicated_user_result.is_err());
    }

    #[tokio::test]
    async fn can_remove_user() {
        crate::core::test_utils::init_canister_config();
        let user_id = Principal::from_slice(&[u8::MAX; 29]);
        let ctx = CallContext::new(user_id);
        let service = UserService::default();

        let user_id = Principal::from_slice(&[u8::MAX; 29]);
        let user = User {
            id: user_id,
            email: Some("john@example.com".to_string()),
            authorization_status: UserAuthorizationStatus::Unauthorized,
            wallets: vec![],
            deployed_wallets: vec![],
            main_wallet: None,
            last_update_timestamp: 0,
        };

        service.user_repository.insert(user.to_key(), user.clone());

        let result = service.remove_user(&user_id, &ctx).await;

        assert!(result.is_ok());
        assert!(service.user_repository.get(&UserKey(user_id)).is_none());
    }
}
