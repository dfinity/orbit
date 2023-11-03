use crate::{
    core::{generate_uuid_v4, CallContext, WithCallContext},
    errors::UserError,
    mappers::{HelperMapper, UserMapper},
    models::{AccessRole, User, UserId},
    repositories::UserRepository,
    transport::{ConfirmUserIdentityInput, EditUserInput, RegisterUserInput},
};
use candid::Principal;
use ic_canister_core::api::ServiceResult;
use ic_canister_core::model::ModelValidator;
use ic_canister_core::{repository::Repository, types::UUID};
use uuid::Uuid;

#[derive(Default, Debug)]
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
        let user =
            self.user_repository
                .get(&User::key(*user_id))
                .ok_or(UserError::NotFoundUser {
                    user: Uuid::from_bytes(user_id.to_owned())
                        .hyphenated()
                        .to_string(),
                })?;

        self.assert_user_access(&user)?;

        Ok(user)
    }

    /// Returns the user associated with the given user identity.
    pub fn get_user_by_identity(&self, identity: &Principal) -> ServiceResult<User> {
        let user = self.user_repository.find_by_identity(identity).ok_or(
            UserError::NotFoundUserIdentity {
                identity: identity.to_text(),
            },
        )?;

        self.assert_user_access(&user)?;

        Ok(user)
    }

    /// Removes the admin role from the given identity if it has an associated user.
    pub async fn remove_admin(&self, identity: &Principal) -> ServiceResult<()> {
        if self.call_context.caller() == *identity {
            Err(UserError::CannotRemoveOwnAdminRole)?
        }

        let user = self.user_repository.find_by_identity(identity);
        if let Some(mut user) = user {
            user.access_roles.retain(|role| *role != AccessRole::Admin);
            self.user_repository.insert(user.to_key(), user.to_owned());
        }

        Ok(())
    }

    /// Creates a new user for the selected user identities.
    ///
    /// If the caller is providing other identities than the caller identity, they will be
    /// added as unconfirmed identities if no user is associated with them.
    pub async fn register_user(
        &self,
        input: RegisterUserInput,
        mut roles: Vec<AccessRole>,
    ) -> ServiceResult<User> {
        let caller_identity = self.call_context.caller();
        self.assert_identity_has_no_associated_user(&caller_identity)?;
        let user_id = generate_uuid_v4().await;
        let identities = match input.identities.is_empty() {
            true => vec![caller_identity],
            false => {
                let mut identities = input.identities;
                if !identities.contains(&caller_identity) {
                    identities.push(caller_identity);
                }

                identities
            }
        };

        for new_identity in identities.iter() {
            self.assert_identity_has_no_associated_user(new_identity)?;
        }

        if !roles.contains(&AccessRole::User) {
            roles.push(AccessRole::User);
        }

        let mut user = UserMapper::from_roles(*user_id.as_bytes(), roles);

        user.update_with(Some(identities), &caller_identity)?;
        user.validate()?;

        self.user_repository.insert(user.to_key(), user.to_owned());

        Ok(user)
    }

    /// Confirms the identity associated with the given user id and returns the updated user.
    pub async fn confirm_user_identity(
        &self,
        input: ConfirmUserIdentityInput,
    ) -> ServiceResult<User> {
        let caller_identity = self.call_context.caller();
        self.assert_identity_has_no_associated_user(&caller_identity)?;

        let user_id = HelperMapper::to_uuid(input.user_id)?;
        let mut user = self.get_user(user_id.as_bytes())?;

        if !user.unconfirmed_identities.contains(&caller_identity) {
            Err(UserError::Forbidden {
                user: Uuid::from_bytes(user.id).hyphenated().to_string(),
            })?
        }

        user.unconfirmed_identities
            .retain(|i| *i != caller_identity);
        user.identities.push(caller_identity);
        user.validate()?;

        self.user_repository.insert(user.to_key(), user.to_owned());

        Ok(user)
    }

    /// Edits the user associated with the given user id and returns the updated user.
    pub async fn edit_user(&self, input: EditUserInput) -> ServiceResult<User> {
        let caller_identity = self.call_context.caller();
        let user_id = HelperMapper::to_uuid(input.user_id)?;
        let mut user = self.get_user(user_id.as_bytes())?;

        user.update_with(input.identities, &caller_identity)?;
        user.validate()?;

        self.user_repository.insert(user.to_key(), user.to_owned());

        Ok(user)
    }

    /// Asserts that the user exists from the given user id.
    pub fn assert_user_exists(&self, user_id: &UUID) -> ServiceResult<()> {
        self.user_repository
            .get(&User::key(*user_id))
            .ok_or(UserError::NotFoundUser {
                user: Uuid::from_bytes(*user_id).hyphenated().to_string(),
            })?;

        Ok(())
    }

    /// Checks if the caller has access to the given user.
    ///
    /// Admins have access to all users.
    fn assert_user_access(&self, user: &User) -> ServiceResult<()> {
        let is_user_owner = user.identities.contains(&self.call_context.caller())
            || user
                .unconfirmed_identities
                .contains(&self.call_context.caller());
        if !is_user_owner && !self.call_context.is_admin() {
            Err(UserError::Forbidden {
                user: Uuid::from_bytes(user.id).hyphenated().to_string(),
            })?
        }

        Ok(())
    }

    /// Asserts that the given identity does not have an associated user.
    fn assert_identity_has_no_associated_user(&self, identity: &Principal) -> ServiceResult<()> {
        let user = self.user_repository.find_by_identity(identity);

        if let Some(user) = user {
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
    use crate::{core::test_utils, models::user_test_utils};

    struct TestContext {
        service: UserService,
        repository: UserRepository,
    }

    fn setup() -> TestContext {
        test_utils::init_canister_config();

        let call_context = CallContext::new(Principal::from_slice(&[9; 29]));

        TestContext {
            repository: UserRepository::default(),
            service: UserService::with_call_context(call_context),
        }
    }

    #[test]
    fn get_user() {
        let ctx: TestContext = setup();
        let mut user = user_test_utils::mock_user();
        user.identities = vec![ctx.service.call_context.caller()];

        ctx.repository.insert(user.to_key(), user.clone());

        let result = ctx.service.get_user(&user.id);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), user);
    }

    #[test]
    fn get_user_by_identity() {
        let ctx: TestContext = setup();
        let mut user = user_test_utils::mock_user();
        user.identities = vec![ctx.service.call_context.caller()];

        ctx.repository.insert(user.to_key(), user.clone());

        let result = ctx.service.get_user_by_identity(&user.identities[0]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), user);
    }

    #[test]
    fn not_allowed_get_user_by_identity() {
        let ctx: TestContext = setup();
        let mut user = user_test_utils::mock_user();
        user.identities = vec![Principal::from_slice(&[255; 29])];

        ctx.repository.insert(user.to_key(), user.clone());

        let result = ctx.service.get_user_by_identity(&user.identities[0]);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn can_remove_admin() {
        let ctx: TestContext = setup();
        let mut caller = user_test_utils::mock_user();
        caller.identities = vec![ctx.service.call_context.caller()];
        caller.access_roles = vec![AccessRole::Admin];
        let mut admin = user_test_utils::mock_user();
        admin.identities = vec![Principal::from_slice(&[255; 29])];
        admin.access_roles = vec![AccessRole::Admin, AccessRole::User];

        ctx.repository.insert(caller.to_key(), caller.clone());
        ctx.repository.insert(admin.to_key(), admin.clone());

        let result = ctx.service.remove_admin(&admin.identities[0]).await;
        assert!(result.is_ok());

        let admin = ctx.repository.get(&admin.to_key()).unwrap();
        assert_eq!(admin.access_roles, vec![AccessRole::User]);
    }

    #[tokio::test]
    async fn fail_remove_self_admin() {
        let ctx: TestContext = setup();
        let mut admin = user_test_utils::mock_user();
        admin.identities = vec![ctx.service.call_context.caller()];
        admin.access_roles = vec![AccessRole::Admin];

        ctx.repository.insert(admin.to_key(), admin.clone());

        let result = ctx.service.remove_admin(&admin.identities[0]).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn register_user_happy_path() {
        let ctx: TestContext = setup();
        let input = RegisterUserInput {
            identities: vec![Principal::from_slice(&[2; 29])],
        };

        let result = ctx.service.register_user(input, vec![]).await;
        assert!(result.is_ok());

        let user = ctx.repository.get(&result.unwrap().to_key()).unwrap();
        assert_eq!(user.identities, vec![ctx.service.call_context.caller()]);
        assert_eq!(
            user.unconfirmed_identities,
            vec![Principal::from_slice(&[2; 29])]
        );
        assert_eq!(user.access_roles, vec![AccessRole::User]);
    }

    #[tokio::test]
    async fn confirm_user_identity() {
        let ctx: TestContext = setup();
        let mut user = user_test_utils::mock_user();
        user.identities = vec![Principal::anonymous()];
        user.unconfirmed_identities = vec![ctx.service.call_context.caller()];

        ctx.repository.insert(user.to_key(), user.clone());

        let input = ConfirmUserIdentityInput {
            user_id: Uuid::from_bytes(user.id).hyphenated().to_string(),
        };

        let result = ctx.service.confirm_user_identity(input).await;
        assert!(result.is_ok());

        let user = ctx.repository.get(&result.unwrap().to_key()).unwrap();
        assert_eq!(
            user.identities,
            vec![Principal::anonymous(), ctx.service.call_context.caller()]
        );
        assert!(user.unconfirmed_identities.is_empty());
    }

    #[tokio::test]
    async fn edit_user_happy_path() {
        let ctx: TestContext = setup();
        let mut user = user_test_utils::mock_user();
        user.identities = vec![Principal::anonymous()];
        user.unconfirmed_identities = vec![ctx.service.call_context.caller()];

        ctx.repository.insert(user.to_key(), user.clone());

        let input = EditUserInput {
            user_id: Uuid::from_bytes(user.id).hyphenated().to_string(),
            identities: Some(vec![ctx.service.call_context.caller()]),
        };

        let result = ctx.service.edit_user(input).await;
        assert!(result.is_ok());

        let user = ctx.repository.get(&result.unwrap().to_key()).unwrap();
        assert_eq!(user.identities, vec![ctx.service.call_context.caller()]);
        assert!(user.unconfirmed_identities.is_empty());
    }
}
