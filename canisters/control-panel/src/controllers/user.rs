//! User services.
use crate::{
    core::{ic_cdk::api::print, CallContext},
    mappers::HelperMapper,
    services::UserService,
    transport::{
        AssociateIdentityWithUserInput, AssociateIdentityWithUserResponse, DeleteUserResponse,
        GetUserResponse, ManageUserInput, ManageUserResponse, RegisterUserInput,
        RegisterUserResponse, UserDTO,
    },
};
use async_trait::async_trait;
use ic_canister_core::api::{ApiResult, WithLogs};
use ic_cdk_macros::{query, update};
use lazy_static::lazy_static;

// Canister entrypoints for the controller.

#[query(name = "get_user")]
async fn get_user() -> ApiResult<GetUserResponse> {
    CONTROLLER.get_user().await
}

#[update(name = "register_user")]
async fn register_user(input: RegisterUserInput) -> ApiResult<RegisterUserResponse> {
    CONTROLLER.register_user(input).await
}

#[update(name = "manage_user")]
async fn manage_user(input: ManageUserInput) -> ApiResult<ManageUserResponse> {
    CONTROLLER.manage_user(input).await
}

#[update(name = "associate_identity_with_user")]
async fn associate_identity_with_user(
    input: AssociateIdentityWithUserInput,
) -> ApiResult<AssociateIdentityWithUserResponse> {
    CONTROLLER.associate_identity_with_user(input).await
}

#[update(name = "delete_user")]
async fn delete_user() -> ApiResult<DeleteUserResponse> {
    CONTROLLER.delete_user().await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: Box<dyn UserController> = {
        let u = UserControllerImpl::new(UserService::new());
        let u = WithLogs(u);
        Box::new(u)
    };
}
#[async_trait]
pub trait UserController: Sync + Send {
    /// Returns the user associated with the caller identity if any.
    async fn get_user(&self) -> ApiResult<GetUserResponse>;
    /// Registers a new user with the given information.
    async fn register_user(&self, input: RegisterUserInput) -> ApiResult<RegisterUserResponse>;
    /// Updates the user with the given information.
    async fn manage_user(&self, input: ManageUserInput) -> ApiResult<ManageUserResponse>;
    /// Deletes the user associated with the caller identity if any.
    async fn delete_user(&self) -> ApiResult<DeleteUserResponse>;
    /// Associates the caller identity with the user if it was set as a unverified identity.
    async fn associate_identity_with_user(
        &self,
        input: AssociateIdentityWithUserInput,
    ) -> ApiResult<AssociateIdentityWithUserResponse>;
}

#[derive(Debug)]
pub struct UserControllerImpl {
    user_service: UserService,
}

impl UserControllerImpl {
    pub fn new(user_service: UserService) -> Self {
        Self { user_service }
    }
}

#[async_trait]
impl UserController for UserControllerImpl {
    async fn get_user(&self) -> ApiResult<GetUserResponse> {
        let ctx: CallContext = CallContext::get();
        let user = self
            .user_service
            .get_user_by_identity(&ctx.caller(), &ctx)?;

        Ok(GetUserResponse {
            user: UserDTO::from(user),
        })
    }

    async fn register_user(&self, input: RegisterUserInput) -> ApiResult<RegisterUserResponse> {
        let ctx: CallContext = CallContext::get();
        let user = self.user_service.register_user(input, &ctx).await?;

        Ok(RegisterUserResponse {
            user: UserDTO::from(user),
        })
    }

    async fn manage_user(&self, input: ManageUserInput) -> ApiResult<ManageUserResponse> {
        let ctx: CallContext = CallContext::get();
        let user = self.user_service.manage_user(input, &ctx).await?;

        Ok(ManageUserResponse {
            user: UserDTO::from(user),
        })
    }

    async fn delete_user(&self) -> ApiResult<DeleteUserResponse> {
        let ctx: CallContext = CallContext::get();
        let user = self
            .user_service
            .get_user_by_identity(&ctx.caller(), &ctx)?;

        let deleted_user = UserService::default().remove_user(&user.id, &ctx).await?;

        Ok(DeleteUserResponse {
            user: UserDTO::from(deleted_user),
        })
    }

    async fn associate_identity_with_user(
        &self,
        input: AssociateIdentityWithUserInput,
    ) -> ApiResult<AssociateIdentityWithUserResponse> {
        let ctx: CallContext = CallContext::get();
        let user = self
            .user_service
            .associate_identity_with_user(*HelperMapper::to_uuid(input.user_id)?.as_bytes(), &ctx)
            .await?;

        Ok(AssociateIdentityWithUserResponse {
            user: UserDTO::from(user),
        })
    }
}

#[async_trait]
impl<T: UserController> UserController for WithLogs<T> {
    async fn get_user(&self) -> ApiResult<GetUserResponse> {
        let out = self.0.get_user().await;

        print(format!("get_user: {:?}", out));

        out
    }

    async fn register_user(&self, input: RegisterUserInput) -> ApiResult<RegisterUserResponse> {
        let out = self.0.register_user(input).await;

        print(format!("register_user: {:?}", out));

        out
    }

    async fn manage_user(&self, input: ManageUserInput) -> ApiResult<ManageUserResponse> {
        let out = self.0.manage_user(input).await;

        print(format!("manage_user: {:?}", out));

        out
    }

    async fn delete_user(&self) -> ApiResult<DeleteUserResponse> {
        let out = self.0.delete_user().await;

        print(format!("delete_user: {:?}", out));

        out
    }

    async fn associate_identity_with_user(
        &self,
        input: AssociateIdentityWithUserInput,
    ) -> ApiResult<AssociateIdentityWithUserResponse> {
        let out = self.0.associate_identity_with_user(input).await;

        print(format!("associate_identity_with_user: {:?}", out));

        out
    }
}
