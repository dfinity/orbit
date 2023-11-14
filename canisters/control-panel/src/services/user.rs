use crate::{
    core::{canister_config, generate_uuid_v4, CallContext},
    errors::UserError,
    mappers::UserMapper,
    models::{User, UserId, UserWallet},
    repositories::UserRepository,
    transport::{ManageUserInput, RegisterUserInput},
};
use candid::Principal;
use ic_canister_core::repository::Repository;
use ic_canister_core::{
    api::{ApiError, ServiceResult},
    model::ModelValidator,
};
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Returns the user associated with the given user id.
    pub fn get_user(&self, user_id: &UserId, ctx: &CallContext) -> ServiceResult<User> {
        let user = self
            .user_repository
            .get(&User::key(user_id))
            .ok_or(UserError::NotFound {
                user: Uuid::from_bytes(user_id.to_owned())
                    .hyphenated()
                    .to_string(),
            })?;

        self.assert_user_access(&user, ctx)?;

        Ok(user)
    }

    /// Returns the user associated with the given user identity.
    pub fn get_user_by_identity(
        &self,
        identity: &Principal,
        ctx: &CallContext,
    ) -> ServiceResult<User> {
        let user = self.user_repository.find_user_by_identity(identity).ok_or(
            UserError::AssociatedUserIdentityNotFound {
                identity: identity.to_text(),
            },
        )?;

        self.assert_user_access(&user, ctx)?;

        Ok(user)
    }

    pub fn get_main_wallet(&self, ctx: &CallContext) -> ServiceResult<Option<UserWallet>> {
        let user = self.get_user_by_identity(&ctx.caller(), ctx)?;

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

    /// Associates the caller identity with the given user if it exists.
    pub async fn associate_identity_with_user(
        &self,
        user_id: UserId,
        ctx: &CallContext,
    ) -> ServiceResult<User, ApiError> {
        let caller = ctx.caller();
        self.assert_identity_is_unregistered(&caller)?;
        let mut user = self.get_user(&user_id, ctx)?;

        let unconfirmed_identity = user
            .unconfirmed_identities
            .clone()
            .into_iter()
            .find(|identity| identity.identity == caller)
            .ok_or(UserError::Forbidden {
                user: Uuid::from_bytes(user_id).hyphenated().to_string(),
            })?;

        let unconfirmed_identities = user
            .unconfirmed_identities
            .iter()
            .filter(|identity| identity.identity != caller)
            .map(|identity| identity.to_owned())
            .collect();

        user.unconfirmed_identities = unconfirmed_identities;
        user.identities.push(unconfirmed_identity);

        user.validate()?;
        self.user_repository.insert(user.to_key(), user.clone());

        Ok(user)
    }

    /// Registers a new user for the caller identity.
    pub async fn register_user(
        &self,
        input: RegisterUserInput,
        ctx: &CallContext,
    ) -> ServiceResult<User, ApiError> {
        self.assert_identity_is_unregistered(&ctx.caller())?;

        let user_id = generate_uuid_v4().await;
        let user = UserMapper::from_register_input(
            input.clone(),
            *user_id.as_bytes(),
            ctx.caller(),
            canister_config().shared_wallet_canister,
        );

        user.validate()?;
        self.user_repository.insert(user.to_key(), user.clone());

        Ok(user)
    }

    pub async fn remove_user(&self, user_id: &UserId, ctx: &CallContext) -> ServiceResult<User> {
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
        let mut user = self.get_user_by_identity(&ctx.caller(), ctx)?;

        user.update_with(input, &ctx.caller())?;
        user.validate()?;

        self.user_repository.insert(user.to_key(), user.clone());

        Ok(user)
    }

    /// Checks if the caller has access to the given user.
    ///
    /// Admins have access to all users.
    fn assert_user_access(&self, user: &User, ctx: &CallContext) -> ServiceResult<()> {
        let is_user_owner = user
            .identities
            .iter()
            .any(|identity| identity.identity == ctx.caller())
            || user
                .unconfirmed_identities
                .iter()
                .any(|identity| identity.identity == ctx.caller());
        if !is_user_owner && !ctx.is_admin() {
            Err(UserError::Forbidden {
                user: Uuid::from_bytes(user.id).hyphenated().to_string(),
            })?
        }

        Ok(())
    }

    /// Validates that the given identity has no associated user.
    ///
    /// If the identity has an associated user, an error is returned.
    pub fn assert_identity_is_unregistered(&self, identity: &Principal) -> ServiceResult<()> {
        let maybe_user = self.user_repository.find_user_by_identity(identity);

        if let Some(user) = maybe_user {
            Err(UserError::IdentityAlreadyHasUser {
                user: Uuid::from_bytes(user.id).hyphenated().to_string(),
            })?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{models::UserIdentity, transport::RegisterUserWalletInput};

    #[test]
    fn get_user_returns_not_found_err() {
        let ctx = CallContext::default();
        let service = UserService::new();
        let user_id = *Uuid::new_v4().as_bytes();

        let result = service.get_user(&user_id, &ctx);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ApiError::from(UserError::NotFound {
                user: Uuid::from_bytes(user_id).hyphenated().to_string()
            })
        );
    }

    #[test]
    fn success_fetch_existing_user() {
        let ctx = CallContext::default();
        let service = UserService::new();
        let user_id = *Uuid::new_v4().as_bytes();
        let user = User {
            id: user_id,
            identities: vec![UserIdentity {
                identity: Principal::anonymous(),
                name: None,
            }],
            unconfirmed_identities: vec![],
            wallets: vec![],
            main_wallet: None,
            last_update_timestamp: 0,
            name: None,
        };

        service.user_repository.insert(user.to_key(), user.clone());

        let result = service.get_user(&user_id, &ctx);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), user);
    }

    #[test]
    fn success_fetch_existing_user_by_identity() {
        let ctx = CallContext::default();
        let service = UserService::new();
        let user_id = *Uuid::new_v4().as_bytes();
        let user = User {
            id: user_id,
            identities: vec![UserIdentity {
                identity: Principal::anonymous(),
                name: None,
            }],
            unconfirmed_identities: vec![],
            wallets: vec![],
            main_wallet: None,
            last_update_timestamp: 0,
            name: None,
        };

        service.user_repository.insert(user.to_key(), user.clone());

        let result = service.get_user_by_identity(&Principal::anonymous(), &ctx);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), user);
    }

    #[tokio::test]
    async fn success_register_new_user() {
        crate::core::test_utils::init_canister_config();

        let ctx = CallContext::default();
        let service = UserService::new();
        let input = RegisterUserInput {
            name: Some("User".to_string()),
            wallet: RegisterUserWalletInput::PrivateWallet {
                id: Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
                use_shared_wallet: None,
            },
        };

        let result = service.register_user(input.clone(), &ctx).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, input.name);
    }

    #[tokio::test]
    async fn failed_registering_new_user_with_same_identity() {
        crate::core::test_utils::init_canister_config();

        let ctx = CallContext::default();
        let service = UserService::new();
        let input = RegisterUserInput {
            name: Some("User".to_string()),
            wallet: RegisterUserWalletInput::PrivateWallet {
                id: Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
                use_shared_wallet: None,
            },
        };
        let duplicated_user_input = RegisterUserInput {
            name: Some("User 2".to_string()),
            wallet: RegisterUserWalletInput::PrivateWallet {
                id: Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
                use_shared_wallet: None,
            },
        };

        let result = service.register_user(input.clone(), &ctx).await;
        let duplicated_user_result = service
            .register_user(duplicated_user_input.clone(), &ctx)
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, input.name);
        assert!(duplicated_user_result.is_err());
    }

    #[tokio::test]
    async fn correctly_associates_identity_with_user() {
        crate::core::test_utils::init_canister_config();
        let ctx = CallContext::new(Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap());
        let service = UserService::new();
        let user_id = *Uuid::new_v4().as_bytes();
        let user = User {
            id: user_id,
            identities: vec![],
            unconfirmed_identities: vec![UserIdentity {
                identity: Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
                name: None,
            }],
            wallets: vec![],
            main_wallet: None,
            last_update_timestamp: 0,
            name: None,
        };

        service.user_repository.insert(user.to_key(), user.clone());

        let result = service.associate_identity_with_user(user_id, &ctx).await;

        assert!(result.is_ok());
        let user = result.unwrap();

        assert_eq!(user.identities.len(), 1);
        assert_eq!(user.unconfirmed_identities.len(), 0);
    }

    #[tokio::test]
    async fn can_remove_user() {
        crate::core::test_utils::init_canister_config();
        let ctx = CallContext::new(Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap());
        let service = UserService::new();

        let user_id = *Uuid::new_v4().as_bytes();
        let user = User {
            id: user_id,
            identities: vec![UserIdentity {
                identity: Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
                name: None,
            }],
            unconfirmed_identities: vec![],
            wallets: vec![],
            main_wallet: None,
            last_update_timestamp: 0,
            name: None,
        };

        service.user_repository.insert(user.to_key(), user.clone());

        let result = service.remove_user(&user_id, &ctx).await;

        assert!(result.is_ok());
        assert!(service.user_repository.get(&User::key(&user_id)).is_none());
    }
}
