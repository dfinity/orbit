//! Arguments for `dfx-orbit request permission permission`.

use clap::Parser;
use orbit_station_api::{
    EditPermissionOperationInput, PermissionResourceActionDTO, RequestOperationInput, ResourceDTO,
};

/// Requests the privilige of proposing canister upgrades.
#[derive(Debug, Clone, Parser)]
pub struct RequestPermissionUpdatePermissionsArgs {
    /// A users that should be permitted to change permissions.  WARNING: Any user that is not listed will lose the ability to change permissions.
    #[structopt(long)]
    pub user: Vec<String>,
    /// A groups that should be permitted to change permissions.  WARNING: Any group that is not listed will lose the ability to change permissions.
    #[structopt(long)]
    pub group: Vec<String>,
}

impl From<RequestPermissionUpdatePermissionsArgs> for RequestOperationInput {
    fn from(value: RequestPermissionUpdatePermissionsArgs) -> Self {
        RequestOperationInput::EditPermission(EditPermissionOperationInput {
            resource: ResourceDTO::Permission(PermissionResourceActionDTO::Update),
            auth_scope: None,
            users: Some(value.user),
            user_groups: Some(value.group),
        })
    }
}

/// Requests the privilige of proposing canister upgrades.
#[derive(Debug, Clone, Parser)]
pub struct RequestPermissionReadPermissionsArgs {
    /// A users that should be permitted to change permissions.  WARNING: Any user that is not listed will lose the ability to change permissions.
    #[structopt(long)]
    pub user: Vec<String>,
    /// A groups that should be permitted to change permissions.  WARNING: Any group that is not listed will lose the ability to change permissions.
    #[structopt(long)]
    pub group: Vec<String>,
}

impl From<RequestPermissionReadPermissionsArgs> for RequestOperationInput {
    fn from(value: RequestPermissionReadPermissionsArgs) -> Self {
        RequestOperationInput::EditPermission(EditPermissionOperationInput {
            resource: ResourceDTO::Permission(PermissionResourceActionDTO::Read),
            auth_scope: None,
            users: Some(value.user),
            user_groups: Some(value.group),
        })
    }
}
