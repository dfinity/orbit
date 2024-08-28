use crate::{
    core::middlewares::{authorize, call_context},
    mappers::HelperMapper,
    models::resource::{Resource, UserResourceAction},
    services::UserService,
};
use ic_cdk_macros::query;
use lazy_static::lazy_static;
use orbit_essentials::api::ApiResult;
use orbit_essentials::with_middleware;
use station_api::{
    GetUserInput, GetUserResponse, ListUsersInput, ListUsersResponse, MeResponse,
    UserCallerPrivilegesDTO,
};

// Canister entrypoints for the controller.
#[query(name = "get_user")]
async fn get_user(input: GetUserInput) -> ApiResult<GetUserResponse> {
    CONTROLLER.get_user(input).await
}

#[query(name = "list_users")]
async fn list_users(input: ListUsersInput) -> ApiResult<ListUsersResponse> {
    CONTROLLER.list_users(input).await
}

#[query(name = "me")]
async fn me() -> ApiResult<MeResponse> {
    CONTROLLER.me().await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: UserController = UserController::new(UserService::default());
}

#[derive(Debug)]
pub struct UserController {
    user_service: UserService,
}

impl UserController {
    fn new(user_service: UserService) -> Self {
        Self { user_service }
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::from(&input)]))]
    async fn get_user(&self, input: GetUserInput) -> ApiResult<GetUserResponse> {
        let ctx = call_context();
        let user = self
            .user_service
            .get_user(HelperMapper::to_uuid(input.user_id)?.as_bytes())?;
        let privileges = self
            .user_service
            .get_caller_privileges_for_user(&user.id, &ctx)
            .await?;

        Ok(GetUserResponse {
            user: user.into(),
            privileges: privileges.into(),
        })
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::User(UserResourceAction::List)]))]
    async fn list_users(&self, input: ListUsersInput) -> ApiResult<ListUsersResponse> {
        let ctx = call_context();
        let list = self.user_service.list_users(input, Some(&ctx)).await?;
        let mut privileges = Vec::new();

        for user in &list.items {
            let user_privileges = self
                .user_service
                .get_caller_privileges_for_user(&user.id, &ctx)
                .await?;

            privileges.push(UserCallerPrivilegesDTO::from(user_privileges));
        }

        Ok(ListUsersResponse {
            users: list.items.into_iter().map(Into::into).collect(),
            next_offset: list.next_offset,
            total: list.total,
            privileges,
        })
    }

    /// Returns the user that is calling this endpoint.
    #[with_middleware(guard = authorize(&call_context(), &[Resource::from(&call_context())]))]
    async fn me(&self) -> ApiResult<MeResponse> {
        let ctx = call_context();
        let user = self.user_service.get_user_by_identity(&ctx.caller())?;

        let privileges = self.user_service.get_caller_privileges(&ctx).await?;

        Ok(MeResponse {
            me: user.into(),
            privileges,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;

    use crate::{
        core::{set_mock_caller, test_utils, validation::disable_mock_resource_validation},
        models::{AddUserOperationInput, UserStatus},
        services::UserService,
    };

    struct TestContext {
        user_service: UserService,
    }

    fn setup() -> TestContext {
        test_utils::init_canister_system();

        TestContext {
            user_service: UserService::default(),
        }
    }

    #[tokio::test]
    async fn me_returns_successfully() {
        let ctx = setup();
        disable_mock_resource_validation();

        let identity = Principal::from_slice(&[1; 29]);
        ctx.user_service
            .add_user(AddUserOperationInput {
                groups: vec![],
                identities: vec![identity],
                name: "user-1".to_string(),
                status: UserStatus::Active,
            })
            .expect("Failed to add user");

        set_mock_caller(identity);
        CONTROLLER
            .me()
            .await
            .expect("Failed to call `me` successfully");
    }

    #[tokio::test]
    async fn me_returns_successfully_with_non_existent_group() {
        let ctx = setup();
        let identity = Principal::from_slice(&[1; 29]);

        ctx.user_service
            .add_user(AddUserOperationInput {
                groups: vec![[0; 16]],
                identities: vec![identity],
                name: "user-1".to_string(),
                status: UserStatus::Active,
            })
            .expect("Failed to add user");

        set_mock_caller(identity);
        let response = CONTROLLER
            .me()
            .await
            .expect("Failed to call `me` successfully");

        assert!(
            response.me.groups.is_empty(),
            "Non existent group should be ignored"
        );
    }
}
