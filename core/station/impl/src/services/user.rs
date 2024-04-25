use crate::{
    core::{
        authorization::Authorization,
        generate_uuid_v4,
        utils::{paginated_items, retain_accessible_resources, PaginatedData, PaginatedItemsArgs},
        CallContext,
    },
    errors::UserError,
    mappers::{authorization::USER_PRIVILEGES, UserMapper},
    models::{
        resource::{Resource, ResourceId, UserResourceAction},
        AddUserOperationInput, EditUserOperationInput, User, UserCallerPrivileges, UserId,
        ADMIN_GROUP_ID,
    },
    repositories::{UserRepository, UserWhereClause},
};
use candid::Principal;
use ic_canister_core::api::ServiceResult;
use ic_canister_core::model::ModelValidator;
use ic_canister_core::repository::Repository;
use lazy_static::lazy_static;
use station_api::{ListUsersInput, UserPrivilege};
use std::sync::Arc;
use uuid::Uuid;

lazy_static! {
    pub static ref USER_SERVICE: Arc<UserService> =
        Arc::new(UserService::new(UserRepository::default()));
}

#[derive(Default, Debug)]
pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub const DEFAULT_USER_LIST_LIMIT: u16 = 100;
    pub const MAX_USER_LIST_LIMIT: u16 = 1000;

    pub fn new(user_repository: UserRepository) -> Self {
        Self { user_repository }
    }

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

        Ok(user)
    }

    pub async fn get_caller_privileges_for_user(
        &self,
        user_id: &UserId,
        ctx: &CallContext,
    ) -> ServiceResult<UserCallerPrivileges> {
        Ok(UserCallerPrivileges {
            id: user_id.to_owned(),
            can_edit: Authorization::is_allowed(
                ctx,
                &Resource::User(UserResourceAction::Update(ResourceId::Id(*user_id))),
            ),
        })
    }

    /// Returns the user associated with the given user identity.
    pub fn get_user_by_identity(&self, identity: &Principal) -> ServiceResult<User> {
        let user = self.user_repository.find_by_identity(identity).ok_or(
            UserError::NotFoundUserIdentity {
                identity: identity.to_text(),
            },
        )?;

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
    /// This method should only be called by a system call (self canister call or controller).
    pub async fn add_user(&self, input: AddUserOperationInput) -> ServiceResult<User> {
        for identity in input.identities.iter() {
            self.assert_identity_has_no_associated_user(identity, None)?;
        }

        if let Some(name) = &input.name {
            self.assert_name_has_no_associated_user(name, None)?;
        }

        let user_id = generate_uuid_v4().await;
        let user = UserMapper::from_create_input(*user_id.as_bytes(), input);

        user.validate()?;

        self.user_repository.insert(user.to_key(), user.to_owned());

        Ok(user)
    }

    /// Edits the user associated with the given user id and returns the updated user.
    ///
    /// This method should only be called by a system call (self canister call or controller).
    pub async fn edit_user(&self, input: EditUserOperationInput) -> ServiceResult<User> {
        let mut user = self.get_user(&input.user_id)?;

        if let Some(identities) = &input.identities {
            for identity in identities.iter() {
                self.assert_identity_has_no_associated_user(identity, Some(user.id))?;
            }
        }

        if let Some(name) = &input.name {
            self.assert_name_has_no_associated_user(name, Some(user.id))?;
        }

        user.update_with(input)?;
        user.validate()?;

        self.user_repository.insert(user.to_key(), user.to_owned());

        Ok(user)
    }

    /// Returns the list of users from the given pagination parameters.
    ///
    /// The default limit is 100 and the maximum limit is 1000.
    pub async fn list_users(
        &self,
        input: ListUsersInput,
        ctx: Option<&CallContext>,
    ) -> ServiceResult<PaginatedData<User>> {
        let mut users = self.user_repository.find_where(UserWhereClause {
            search_term: input.search_term,
            statuses: input
                .statuses
                .map(|statuses| statuses.into_iter().map(Into::into).collect()),
        });

        // filter out users that the caller does not have access to read
        if let Some(ctx) = ctx {
            retain_accessible_resources(ctx, &mut users, |user| {
                Resource::User(UserResourceAction::Read(ResourceId::Id(user.id)))
            });
        }

        let result = paginated_items(PaginatedItemsArgs {
            offset: input.paginate.to_owned().and_then(|p| p.offset),
            limit: input.paginate.and_then(|p| p.limit),
            default_limit: Some(Self::DEFAULT_USER_LIST_LIMIT),
            max_limit: Some(Self::MAX_USER_LIST_LIMIT),
            items: &users,
        })?;

        Ok(result)
    }

    /// Returns the user privileges from the given user.
    pub async fn get_caller_privileges(
        &self,
        ctx: &CallContext,
    ) -> ServiceResult<Vec<UserPrivilege>> {
        let mut privileges = Vec::new();

        for privilege in USER_PRIVILEGES.into_iter() {
            let is_allowed = Authorization::is_allowed(ctx, &privilege.to_owned().into());

            if is_allowed {
                privileges.push(privilege.to_owned());
            }
        }

        Ok(privileges)
    }

    /// Asserts that the given identity does not have an associated user.
    fn assert_identity_has_no_associated_user(
        &self,
        identity: &Principal,
        skip_user_id: Option<UserId>,
    ) -> ServiceResult<()> {
        let user = self.user_repository.find_by_identity(identity);

        if let Some(user) = user {
            if let Some(skip_user_id) = skip_user_id {
                if user.id == skip_user_id {
                    return Ok(());
                }
            }

            Err(UserError::IdentityAlreadyHasUser {
                user: Uuid::from_bytes(user.id).hyphenated().to_string(),
            })?
        }

        Ok(())
    }

    fn assert_name_has_no_associated_user(
        &self,
        name: &str,
        skip_user_id: Option<UserId>,
    ) -> ServiceResult<()> {
        let user = self.user_repository.find_by_name(name);

        if let Some(id) = user {
            if let Some(skip_user_id) = skip_user_id {
                if id == skip_user_id {
                    return Ok(());
                }
            }

            Err(UserError::NameAlreadyHasUser {
                user: Uuid::from_bytes(id).hyphenated().to_string(),
            })?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::{test_utils, validation::disable_mock_resource_validation},
        models::{
            access_policy::AuthScope,
            user_group_test_utils::mock_user_group,
            user_test_utils::{self, mock_user},
            EditAccessPolicyOperationInput, UserStatus,
        },
        repositories::{UserGroupRepository, USER_REPOSITORY},
        services::access_policy::ACCESS_POLICY_SERVICE,
    };
    use station_api::PaginationInput;

    struct TestContext {
        service: UserService,
        repository: UserRepository,
        call_context: CallContext,
    }

    fn setup() -> TestContext {
        test_utils::init_canister_system();

        let user_group_repository = UserGroupRepository::default();

        user_group_repository.insert(*ADMIN_GROUP_ID, mock_user_group());

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

        let result = ctx.service.get_user(&user.id);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), user);
    }

    #[test]
    fn get_user_by_identity() {
        let ctx: TestContext = setup();
        let mut user = user_test_utils::mock_user();
        user.identities = vec![ctx.call_context.caller()];

        ctx.repository.insert(user.to_key(), user.clone());

        let result = ctx.service.get_user_by_identity(&user.identities[0]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), user);
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
            groups: vec![*ADMIN_GROUP_ID],
            status: UserStatus::Active,
            name: None,
        };

        let result = ctx.service.add_user(input).await;
        assert!(result.is_ok());

        let user = ctx.repository.get(&result.unwrap().to_key()).unwrap();
        assert_eq!(user.identities, vec![Principal::from_slice(&[2; 29])]);
        assert_eq!(user.groups, vec![*ADMIN_GROUP_ID]);
    }

    #[tokio::test]
    async fn add_user_non_existent_group_should_fail() {
        let ctx: TestContext = setup();

        disable_mock_resource_validation();

        let input = AddUserOperationInput {
            identities: vec![Principal::from_slice(&[2; 29])],
            groups: vec![[0; 16]],
            status: UserStatus::Active,
            name: None,
        };

        let result = ctx.service.add_user(input).await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "USER_GROUP_DOES_NOT_EXIST: The user group 00000000-0000-0000-0000-000000000000 does not exist."
        );
    }

    #[tokio::test]
    async fn add_user_with_identity_of_existing_user_should_fail() {
        let ctx: TestContext = setup();
        let input = AddUserOperationInput {
            identities: vec![Principal::from_slice(&[2; 29])],
            groups: vec![*ADMIN_GROUP_ID],
            status: UserStatus::Active,
            name: Some("Jane Doe".to_string()),
        };

        let result = ctx.service.add_user(input).await;
        assert!(result.is_ok());

        let input = AddUserOperationInput {
            identities: vec![Principal::from_slice(&[2; 29])],
            groups: vec![*ADMIN_GROUP_ID],
            status: UserStatus::Active,
            name: Some("John Doe".to_string()),
        };

        let result = ctx.service.add_user(input).await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "IDENTITY_ALREADY_HAS_USER: The identity already has an associated user."
        );
    }

    #[tokio::test]
    async fn add_user_with_existing_name_should_fail() {
        let mut user = mock_user();
        user.name = Some("Jane Doe".to_string());

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        let input = AddUserOperationInput {
            identities: vec![Principal::from_slice(&[3; 29])],
            groups: vec![*ADMIN_GROUP_ID],
            status: UserStatus::Active,
            name: Some("Jane Doe".to_string()),
        };

        let result = USER_SERVICE.add_user(input).await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "NAME_ALREADY_HAS_USER: The name already has an associated user."
        );
    }

    #[tokio::test]
    async fn edit_user_with_existing_name_should_fail() {
        let mut user = mock_user();
        user.name = Some("Jane Doe".to_string());
        let mut another_user = mock_user();
        another_user.name = Some("John Doe".to_string());

        USER_REPOSITORY.insert(user.to_key(), user.clone());
        USER_REPOSITORY.insert(another_user.to_key(), another_user.clone());

        let input = EditUserOperationInput {
            user_id: user.id,
            name: another_user.name,
            identities: None,
            groups: None,
            status: None,
        };

        let result = USER_SERVICE.edit_user(input).await;

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err().to_string(),
            "NAME_ALREADY_HAS_USER: The name already has an associated user."
        );
    }

    #[tokio::test]
    async fn edit_user_happy_path() {
        let ctx: TestContext = setup();
        let mut user = user_test_utils::mock_user();
        user.identities = vec![Principal::from_slice(&[1; 29])];

        ctx.repository.insert(user.to_key(), user.clone());

        let input = EditUserOperationInput {
            user_id: user.id,
            identities: Some(vec![ctx.call_context.caller()]),
            groups: None,
            name: None,
            status: None,
        };

        let result = ctx.service.edit_user(input).await;
        assert!(result.is_ok());

        let user = ctx.repository.get(&result.unwrap().to_key()).unwrap();
        assert_eq!(user.identities, vec![ctx.call_context.caller()]);
    }

    #[tokio::test]
    async fn edit_user_should_fail_for_identity_of_existing_user() {
        let mut user = user_test_utils::mock_user();
        user.identities = vec![Principal::from_slice(&[2; 29])];
        USER_REPOSITORY.insert(user.to_key(), user.clone());

        let mut another_user = user_test_utils::mock_user();
        another_user.identities = vec![Principal::from_slice(&[3; 29])];
        USER_REPOSITORY.insert(another_user.to_key(), another_user.clone());

        let input = EditUserOperationInput {
            user_id: user.id,
            identities: Some(vec![Principal::from_slice(&[3; 29])]),
            groups: None,
            name: None,
            status: None,
        };

        let result = USER_SERVICE.edit_user(input).await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "IDENTITY_ALREADY_HAS_USER: The identity already has an associated user."
        );
    }

    #[tokio::test]
    async fn list_users_should_use_offset_and_limit() {
        let ctx: TestContext = setup();
        for i in 0..50 {
            let mut user = user_test_utils::mock_user();
            user.id = [i; 16];
            user.identities = vec![Principal::from_slice(&[i; 29])];
            ctx.repository.insert(user.to_key(), user.clone());
        }

        let input = ListUsersInput {
            search_term: None,
            statuses: None,
            paginate: Some(PaginationInput {
                offset: Some(10),
                limit: Some(30),
            }),
        };

        let result = ctx.service.list_users(input, None).await.unwrap();
        assert_eq!(result.items.len(), 30);
        assert_eq!(result.next_offset, Some(40));
    }

    #[tokio::test]
    async fn get_user_privileges_by_identity() {
        let mut user = user_test_utils::mock_user();
        user.groups = Vec::new();

        USER_REPOSITORY.insert(user.to_key(), user.clone());

        let ctx = CallContext::new(user.identities[0]);

        ACCESS_POLICY_SERVICE
            .edit_access_policy(EditAccessPolicyOperationInput {
                auth_scope: Some(AuthScope::Restricted),
                user_groups: None,
                users: Some(vec![user.id]),
                resource: Resource::User(UserResourceAction::List),
            })
            .await
            .unwrap();
        ACCESS_POLICY_SERVICE
            .edit_access_policy(EditAccessPolicyOperationInput {
                auth_scope: Some(AuthScope::Authenticated),
                user_groups: None,
                users: Some(Vec::new()),
                resource: Resource::User(UserResourceAction::Create),
            })
            .await
            .unwrap();

        let privileges = USER_SERVICE.get_caller_privileges(&ctx).await.unwrap();

        assert_eq!(privileges.len(), 2);
        assert!(privileges.contains(&UserPrivilege::ListUsers));
        assert!(privileges.contains(&UserPrivilege::AddUser));
    }
}
