use crate::{
    core::{
        access_control::evaluate_caller_access,
        generate_uuid_v4,
        ic_cdk::api::print,
        utils::{paginated_items, PaginatedData, PaginatedItemsArgs},
        CallContext,
    },
    errors::{AccessControlError, UserError},
    mappers::{UserMapper, USER_PRIVILEGES},
    models::{
        access_control::{ResourceSpecifier, ResourceType, UserActionSpecifier},
        specifier::CommonSpecifier,
        AddUserOperationInput, EditUserOperationInput, User, UserId, ADMIN_GROUP_ID,
    },
    repositories::{UserRepository, UserWhereClause},
};
use candid::Principal;
use futures::{stream, StreamExt};
use ic_canister_core::api::ServiceResult;
use ic_canister_core::model::ModelValidator;
use ic_canister_core::repository::Repository;
use lazy_static::lazy_static;
use std::sync::Arc;
use uuid::Uuid;
use wallet_api::{ListUsersInput, UserPrivilege};

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
    /// This method should only be called by a system call (self canister call or controller).
    pub async fn edit_user(&self, input: EditUserOperationInput) -> ServiceResult<User> {
        let mut user = self.get_user(&input.user_id)?;

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
            users = stream::iter(users.iter())
                .filter_map(|user| async move {
                    match evaluate_caller_access(
                        ctx,
                        &ResourceSpecifier::Common(
                            ResourceType::User,
                            UserActionSpecifier::Read(CommonSpecifier::Id(vec![user
                                .id
                                .to_owned()])),
                        ),
                    )
                    .await
                    {
                        Ok(_) => Some(user.to_owned()),
                        Err(_) => None,
                    }
                })
                .collect()
                .await
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
    pub async fn get_user_privileges_by_identity(
        &self,
        user_identity: &Principal,
    ) -> ServiceResult<Vec<UserPrivilege>> {
        let mut privileges = Vec::new();
        for privilege in USER_PRIVILEGES.into_iter() {
            let evaluated_access = evaluate_caller_access(
                &CallContext::new(user_identity.to_owned()),
                &privilege.to_owned().into(),
            )
            .await;

            match evaluated_access {
                Ok(_) => privileges.push(privilege),
                Err(AccessControlError::Unauthorized { .. }) => {}
                Err(err) => {
                    // We do not fail the entire operation if there is an error
                    // to still return the valid privileges that were evaluated,
                    // this enables clients to still use the valid evaluated privileges.
                    print(format!("Error evaluating user access: {:?}", err));
                }
            }
        }

        Ok(privileges)
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
    use wallet_api::PaginationInput;

    use super::*;
    use crate::{
        core::test_utils,
        models::{
            access_control::{
                CommonActionSpecifier, ResourceSpecifier, ResourceType, UserSpecifier,
            },
            user_test_utils, AddAccessPolicyOperationInput, UserStatus,
        },
        services::POLICY_SERVICE,
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
    async fn edit_user_happy_path() {
        let ctx: TestContext = setup();
        let mut user = user_test_utils::mock_user();
        user.identities = vec![Principal::anonymous()];

        ctx.repository.insert(user.to_key(), user.clone());

        let input = EditUserOperationInput {
            user_id: user.id,
            identities: Some(vec![ctx.call_context.caller()]),
            groups: None,
            name: None,
        };

        let result = ctx.service.edit_user(input).await;
        assert!(result.is_ok());

        let user = ctx.repository.get(&result.unwrap().to_key()).unwrap();
        assert_eq!(user.identities, vec![ctx.call_context.caller()]);
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
        let ctx: TestContext = setup();
        let mut user = user_test_utils::mock_user();
        user.identities = vec![ctx.call_context.caller()];
        ctx.repository.insert(user.to_key(), user.clone());

        POLICY_SERVICE
            .add_access_policy(AddAccessPolicyOperationInput {
                user: UserSpecifier::Id(vec![user.id]),
                resource: ResourceSpecifier::Common(
                    ResourceType::User,
                    CommonActionSpecifier::List,
                ),
            })
            .await
            .unwrap();
        POLICY_SERVICE
            .add_access_policy(AddAccessPolicyOperationInput {
                user: UserSpecifier::Any,
                resource: ResourceSpecifier::Common(
                    ResourceType::User,
                    CommonActionSpecifier::Create,
                ),
            })
            .await
            .unwrap();

        let privileges = ctx
            .service
            .get_user_privileges_by_identity(&user.identities[0])
            .await
            .unwrap();

        assert_eq!(privileges.len(), 2);
        assert!(privileges.contains(&UserPrivilege::ListUsers));
        assert!(privileges.contains(&UserPrivilege::AddUser));
    }
}
