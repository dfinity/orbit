use crate::{
    core::{canister_config, generate_uuid_v4, CallContext, WithCallContext},
    errors::UserError,
    mappers::UserMapper,
    models::{User, UserBank, UserId},
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

#[derive(Default)]
pub struct UserService {
    call_context: CallContext,
    user_repository: UserRepository,
}

impl WithCallContext for UserService {
    fn with_call_context(call_context: CallContext) -> Self {
        Self {
            call_context: call_context.clone(),
            ..Default::default()
        }
    }
}

impl UserService {
    /// Returns the user associated with the given user id.
    pub fn get_user(&self, user_id: &UserId) -> ServiceResult<User> {
        let user = self
            .user_repository
            .get(&User::key(user_id))
            .ok_or(UserError::NotFound {
                user: Uuid::from_bytes(user_id.to_owned())
                    .hyphenated()
                    .to_string(),
            })?;

        self.assert_user_access(&user)?;

        Ok(user)
    }

    /// Returns the user associated with the given user identity.
    pub fn get_user_by_identity(&self, identity: &Principal) -> ServiceResult<User> {
        let user = self.user_repository.find_user_by_identity(identity).ok_or(
            UserError::AssociatedUserIdentityNotFound {
                identity: identity.to_text(),
            },
        )?;

        self.assert_user_access(&user)?;

        Ok(user)
    }

    pub fn get_main_bank(&self) -> ServiceResult<Option<UserBank>> {
        let user = self.get_user_by_identity(&self.call_context.caller())?;

        match user.main_bank {
            Some(main_bank) => {
                let main_bank = user
                    .banks
                    .into_iter()
                    .find(|bank| bank.canister_id == main_bank)
                    .ok_or(UserError::MainBankNotFound)?;

                Ok(Some(main_bank))
            }
            None => Ok(None),
        }
    }

    /// Associates the caller identity with the given user if it exists.
    pub async fn associate_identity_with_user(
        &self,
        user_id: UserId,
    ) -> ServiceResult<User, ApiError> {
        let caller = self.call_context.caller();
        self.assert_identity_is_unregistered(&caller)?;
        let mut user = self.get_user(&user_id)?;

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
    pub async fn register_user(&self, input: RegisterUserInput) -> ServiceResult<User, ApiError> {
        self.assert_identity_is_unregistered(&self.call_context.caller())?;

        let user_id = generate_uuid_v4().await;
        let user = UserMapper::from_register_input(
            input.clone(),
            *user_id.as_bytes(),
            self.call_context.caller(),
            canister_config().shared_bank_canister,
        );

        user.validate()?;
        self.user_repository.insert(user.to_key(), user.clone());

        Ok(user)
    }

    pub async fn remove_user(&self, user_id: &UserId) -> ServiceResult<User> {
        let user = self.get_user(user_id)?;

        self.assert_user_access(&user)?;

        self.user_repository.remove(&user.to_key());

        Ok(user)
    }

    pub async fn manage_user(&self, input: ManageUserInput) -> ServiceResult<User> {
        let mut user = self.get_user_by_identity(&self.call_context.caller())?;

        user.update_with(input, &self.call_context.caller())?;
        user.validate()?;

        self.user_repository.insert(user.to_key(), user.clone());

        Ok(user)
    }

    /// Checks if the caller has access to the given user.
    ///
    /// Admins have access to all users.
    fn assert_user_access(&self, user: &User) -> ServiceResult<()> {
        let is_user_owner = user
            .identities
            .iter()
            .any(|identity| identity.identity == self.call_context.caller())
            || user
                .unconfirmed_identities
                .iter()
                .any(|identity| identity.identity == self.call_context.caller());
        if !is_user_owner && !self.call_context.is_admin() {
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
    use crate::{models::UserIdentity, transport::RegisterUserBankInput};

    #[test]
    fn get_user_returns_not_found_err() {
        let service = UserService::default();
        let user_id = *Uuid::new_v4().as_bytes();

        let result = service.get_user(&user_id);

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
        let service = UserService::default();
        let user_id = *Uuid::new_v4().as_bytes();
        let user = User {
            id: user_id,
            identities: vec![UserIdentity {
                identity: Principal::anonymous(),
                name: None,
            }],
            unconfirmed_identities: vec![],
            banks: vec![],
            main_bank: None,
            last_update_timestamp: 0,
            name: None,
        };

        service.user_repository.insert(user.to_key(), user.clone());

        let result = service.get_user(&user_id);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), user);
    }

    #[test]
    fn success_fetch_existing_user_by_identity() {
        let service = UserService::default();
        let user_id = *Uuid::new_v4().as_bytes();
        let user = User {
            id: user_id,
            identities: vec![UserIdentity {
                identity: Principal::anonymous(),
                name: None,
            }],
            unconfirmed_identities: vec![],
            banks: vec![],
            main_bank: None,
            last_update_timestamp: 0,
            name: None,
        };

        service.user_repository.insert(user.to_key(), user.clone());

        let result = service.get_user_by_identity(&Principal::anonymous());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), user);
    }

    #[tokio::test]
    async fn success_register_new_user() {
        crate::core::test_utils::init_canister_config();

        let service = UserService::default();
        let input = RegisterUserInput {
            name: Some("User".to_string()),
            bank: RegisterUserBankInput::PrivateBank {
                id: Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
                use_shared_bank: None,
            },
        };

        let result = service.register_user(input.clone()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, input.name);
    }

    #[tokio::test]
    async fn failed_registering_new_user_with_same_identity() {
        crate::core::test_utils::init_canister_config();

        let service = UserService::default();
        let input = RegisterUserInput {
            name: Some("User".to_string()),
            bank: RegisterUserBankInput::PrivateBank {
                id: Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
                use_shared_bank: None,
            },
        };
        let duplicated_user_input = RegisterUserInput {
            name: Some("User 2".to_string()),
            bank: RegisterUserBankInput::PrivateBank {
                id: Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
                use_shared_bank: None,
            },
        };

        let result = service.register_user(input.clone()).await;
        let duplicated_user_result = service.register_user(duplicated_user_input.clone()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, input.name);
        assert!(duplicated_user_result.is_err());
    }

    #[tokio::test]
    async fn correctly_associates_identity_with_user() {
        crate::core::test_utils::init_canister_config();
        let service = UserService {
            call_context: CallContext::new(
                Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
            ),
            ..Default::default()
        };

        let user_id = *Uuid::new_v4().as_bytes();
        let user = User {
            id: user_id,
            identities: vec![],
            unconfirmed_identities: vec![UserIdentity {
                identity: Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
                name: None,
            }],
            banks: vec![],
            main_bank: None,
            last_update_timestamp: 0,
            name: None,
        };

        service.user_repository.insert(user.to_key(), user.clone());

        let result = service.associate_identity_with_user(user_id).await;

        assert!(result.is_ok());
        let user = result.unwrap();

        assert_eq!(user.identities.len(), 1);
        assert_eq!(user.unconfirmed_identities.len(), 0);
    }

    #[tokio::test]
    async fn can_remove_user() {
        crate::core::test_utils::init_canister_config();
        let service = UserService {
            call_context: CallContext::new(
                Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
            ),
            ..Default::default()
        };

        let user_id = *Uuid::new_v4().as_bytes();
        let user = User {
            id: user_id,
            identities: vec![UserIdentity {
                identity: Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap(),
                name: None,
            }],
            unconfirmed_identities: vec![],
            banks: vec![],
            main_bank: None,
            last_update_timestamp: 0,
            name: None,
        };

        service.user_repository.insert(user.to_key(), user.clone());

        let result = service.remove_user(&user_id).await;

        assert!(result.is_ok());
        assert!(service.user_repository.get(&User::key(&user_id)).is_none());
    }
}
