use crate::{
    core::{generate_uuid_v4, CallContext},
    errors::UserError,
    mappers::{HelperMapper, UserMapper},
    models::{AddUserOperationInput, EditUserOperationInput, User, UserId, ADMIN_GROUP_ID},
    repositories::UserRepository,
};
use candid::Principal;
use ic_canister_core::api::ServiceResult;
use ic_canister_core::model::ModelValidator;
use ic_canister_core::{repository::Repository, types::UUID};
use lazy_static::lazy_static;
use uuid::Uuid;
use wallet_api::ConfirmUserIdentityInput;

lazy_static! {
    pub static ref USER_SERVICE: UserService = UserService::default();
}

#[derive(Default, Debug)]
pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    /// Returns the user associated with the given user id.
    pub fn get_user(&self, user_id: &UserId, ctx: &CallContext) -> ServiceResult<User> {
        let user =
            self.user_repository
                .get(&User::key(*user_id))
                .ok_or(UserError::NotFoundUser {
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
        let user = self.user_repository.find_by_identity(identity).ok_or(
            UserError::NotFoundUserIdentity {
                identity: identity.to_text(),
            },
        )?;

        self.assert_user_access(&user, ctx)?;

        Ok(user)
    }

    /// Removes the admin role from the given identity if it has an associated user.
    pub async fn remove_admin(&self, identity: &Principal, ctx: &CallContext) -> ServiceResult<()> {
        if ctx.caller() == *identity {
            Err(UserError::CannotRemoveOwnAdminRole)?
        }

        let user = self.user_repository.find_by_identity(identity);
        if let Some(mut user) = user {
            user.groups.retain(|group| *group != *ADMIN_GROUP_ID);
            self.user_repository.insert(user.to_key(), user.to_owned());
        }

        Ok(())
    }

    /// Creates a new user with the given user details and returns the created user.
    ///
    /// This method can only be called by a system call (self canister call or controller).
    pub async fn add_user(
        &self,
        input: AddUserOperationInput,
        ctx: &CallContext,
    ) -> ServiceResult<User> {
        if !ctx.caller_is_controller_or_self() {
            Err(UserError::Unauthorized)?
        }

        for identity in input.identities.iter() {
            self.assert_identity_has_no_associated_user(identity)?;
        }

        let user_id = generate_uuid_v4().await;
        let user = UserMapper::from_create_input(*user_id.as_bytes(), input);

        user.validate()?;

        self.user_repository.insert(user.to_key(), user.to_owned());

        Ok(user)
    }

    /// Edits the user associated with the given user id and returns the updated user.
    ///
    /// This method can only be called by a system call (self canister call or controller).
    pub async fn edit_user(
        &self,
        input: EditUserOperationInput,
        ctx: &CallContext,
    ) -> ServiceResult<User> {
        if !ctx.caller_is_controller_or_self() {
            Err(UserError::Unauthorized)?
        }

        let mut user = self.get_user(&input.user_id, ctx)?;

        user.update_with(input)?;
        user.validate()?;

        self.user_repository.insert(user.to_key(), user.to_owned());

        Ok(user)
    }

    /// Confirms the identity associated with the given user id and returns the updated user.
    pub async fn confirm_user_identity(
        &self,
        input: ConfirmUserIdentityInput,
        ctx: &CallContext,
    ) -> ServiceResult<User> {
        let caller_identity = ctx.caller();
        self.assert_identity_has_no_associated_user(&caller_identity)?;

        let user_id = HelperMapper::to_uuid(input.user_id)?;
        let mut user = self.get_user(user_id.as_bytes(), ctx)?;

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
    fn assert_user_access(&self, user: &User, ctx: &CallContext) -> ServiceResult<()> {
        let is_user_owner = user.identities.contains(&ctx.caller())
            || user.unconfirmed_identities.contains(&ctx.caller());
        if !is_user_owner && !ctx.is_admin() {
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
    use crate::core::ic_cdk::api::id as self_canister_id;
    use crate::{
        core::test_utils,
        models::{user_test_utils, UserStatus},
    };

    struct TestContext {
        service: UserService,
        repository: UserRepository,
        call_context: CallContext,
    }

    fn setup() -> TestContext {
        test_utils::init_canister_config();

        TestContext {
            repository: UserRepository::default(),
            service: UserService::default(),
            call_context: CallContext::new(Principal::from_slice(&[9; 29])),
        }
    }

    #[test]
    fn get_user() {
        let ctx: TestContext = setup();
        let mut user = user_test_utils::mock_user();
        user.identities = vec![ctx.call_context.caller()];

        ctx.repository.insert(user.to_key(), user.clone());

        let result = ctx.service.get_user(&user.id, &ctx.call_context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), user);
    }

    #[test]
    fn get_user_by_identity() {
        let ctx: TestContext = setup();
        let mut user = user_test_utils::mock_user();
        user.identities = vec![ctx.call_context.caller()];

        ctx.repository.insert(user.to_key(), user.clone());

        let result = ctx
            .service
            .get_user_by_identity(&user.identities[0], &ctx.call_context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), user);
    }

    #[test]
    fn not_allowed_get_user_by_identity() {
        let ctx: TestContext = setup();
        let mut user = user_test_utils::mock_user();
        user.identities = vec![Principal::from_slice(&[255; 29])];

        ctx.repository.insert(user.to_key(), user.clone());

        let result = ctx
            .service
            .get_user_by_identity(&user.identities[0], &ctx.call_context);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn can_remove_admin() {
        let ctx: TestContext = setup();
        let mut caller = user_test_utils::mock_user();
        caller.identities = vec![ctx.call_context.caller()];
        caller.groups = vec![*ADMIN_GROUP_ID];
        let mut admin = user_test_utils::mock_user();
        admin.identities = vec![Principal::from_slice(&[255; 29])];
        admin.groups = vec![*ADMIN_GROUP_ID, [1; 16]];

        ctx.repository.insert(caller.to_key(), caller.clone());
        ctx.repository.insert(admin.to_key(), admin.clone());

        let result = ctx
            .service
            .remove_admin(&admin.identities[0], &ctx.call_context)
            .await;
        assert!(result.is_ok());

        let admin = ctx.repository.get(&admin.to_key()).unwrap();
        assert_eq!(admin.groups, vec![[1; 16]]);
    }

    #[tokio::test]
    async fn fail_remove_self_admin() {
        let ctx: TestContext = setup();
        let mut admin = user_test_utils::mock_user();
        admin.identities = vec![ctx.call_context.caller()];
        admin.groups = vec![*ADMIN_GROUP_ID];

        ctx.repository.insert(admin.to_key(), admin.clone());

        let result = ctx
            .service
            .remove_admin(&admin.identities[0], &ctx.call_context)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn add_user_happy_path() {
        let ctx: TestContext = setup();
        let input = AddUserOperationInput {
            identities: vec![Principal::from_slice(&[2; 29])],
            unconfirmed_identities: vec![],
            groups: vec![*ADMIN_GROUP_ID],
            status: UserStatus::Active,
            name: None,
        };
        let call_context = CallContext::new(self_canister_id());

        let result = ctx.service.add_user(input, &call_context).await;
        assert!(result.is_ok());

        let user = ctx.repository.get(&result.unwrap().to_key()).unwrap();
        assert_eq!(user.identities, vec![Principal::from_slice(&[2; 29])]);
        assert_eq!(user.unconfirmed_identities, vec![]);
        assert_eq!(user.groups, vec![*ADMIN_GROUP_ID]);
    }

    #[tokio::test]
    async fn confirm_user_identity() {
        let ctx: TestContext = setup();
        let mut user = user_test_utils::mock_user();
        user.identities = vec![Principal::anonymous()];
        user.unconfirmed_identities = vec![ctx.call_context.caller()];

        ctx.repository.insert(user.to_key(), user.clone());

        let input = ConfirmUserIdentityInput {
            user_id: Uuid::from_bytes(user.id).hyphenated().to_string(),
        };

        let result = ctx
            .service
            .confirm_user_identity(input, &ctx.call_context)
            .await;
        assert!(result.is_ok());

        let user = ctx.repository.get(&result.unwrap().to_key()).unwrap();
        assert_eq!(
            user.identities,
            vec![Principal::anonymous(), ctx.call_context.caller()]
        );
        assert!(user.unconfirmed_identities.is_empty());
    }

    #[tokio::test]
    async fn edit_user_happy_path() {
        let ctx: TestContext = setup();
        let mut user = user_test_utils::mock_user();
        user.identities = vec![Principal::anonymous()];
        user.unconfirmed_identities = vec![ctx.call_context.caller()];

        ctx.repository.insert(user.to_key(), user.clone());

        let input = EditUserOperationInput {
            user_id: user.id,
            identities: Some(vec![ctx.call_context.caller()]),
            unconfirmed_identities: None,
            groups: None,
            name: None,
        };

        let call_context = CallContext::new(self_canister_id());
        let result = ctx.service.edit_user(input, &call_context).await;
        assert!(result.is_ok());

        let user = ctx.repository.get(&result.unwrap().to_key()).unwrap();
        assert_eq!(user.identities, vec![ctx.call_context.caller()]);
        assert!(user.unconfirmed_identities.is_empty());
    }
}
