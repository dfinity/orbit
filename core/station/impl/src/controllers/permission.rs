use crate::{
    core::{
        authorization::Authorization,
        middlewares::{authorize, call_context},
    },
    models::resource::{PermissionResourceAction, Resource},
    services::permission::{PermissionService, PERMISSION_SERVICE},
};
use ic_cdk_macros::query;
use lazy_static::lazy_static;
use orbit_essentials::api::ApiResult;
use orbit_essentials::with_middleware;
use station_api::{
    GetPermissionInput, GetPermissionResponse, ListPermissionsInput, ListPermissionsResponse,
    PermissionCallerPrivilegesDTO,
};
use std::sync::Arc;

// Canister entrypoints for the controller.
#[query(name = "get_permission")]
async fn get_permission(input: GetPermissionInput) -> ApiResult<GetPermissionResponse> {
    CONTROLLER.get_permission(input).await
}

#[query(name = "list_permissions")]
async fn list_permissions(input: ListPermissionsInput) -> ApiResult<ListPermissionsResponse> {
    CONTROLLER.list_permissions(input).await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: PermissionController =
        PermissionController::new(Arc::clone(&PERMISSION_SERVICE));
}

#[derive(Debug)]
pub struct PermissionController {
    permission_service: Arc<PermissionService>,
}

impl PermissionController {
    fn new(permission_service: Arc<PermissionService>) -> Self {
        Self { permission_service }
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::Permission(PermissionResourceAction::Read)]))]
    async fn get_permission(&self, input: GetPermissionInput) -> ApiResult<GetPermissionResponse> {
        let ctx = call_context();
        let permission = self
            .permission_service
            .get_permission(&Resource::from(input.resource));

        Ok(GetPermissionResponse {
            permission: permission.clone().into(),
            privileges: station_api::PermissionCallerPrivilegesDTO {
                can_edit: Authorization::is_allowed(
                    &ctx,
                    &Resource::Permission(PermissionResourceAction::Update),
                ),
                resource: permission.resource.into(),
            },
        })
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::Permission(PermissionResourceAction::Read)]))]
    async fn list_permissions(
        &self,
        input: ListPermissionsInput,
    ) -> ApiResult<ListPermissionsResponse> {
        let ctx = call_context();
        let result = self.permission_service.list_permissions(input).await?;
        let deps = self
            .permission_service
            .get_permissions_dependencies(&result.items)?;

        let can_edit = Authorization::is_allowed(
            &ctx,
            &Resource::Permission(PermissionResourceAction::Update),
        );
        let mut privileges = Vec::new();
        for policy in &result.items {
            privileges.push(PermissionCallerPrivilegesDTO {
                can_edit,
                resource: policy.resource.clone().into(),
            });
        }

        Ok(ListPermissionsResponse {
            permissions: result.items.into_iter().map(|p| p.into()).collect(),
            user_groups: deps.groups.into_iter().map(Into::into).collect(),
            users: deps.users.into_iter().map(Into::into).collect(),
            next_offset: result.next_offset,
            total: result.total,
            privileges,
        })
    }
}
