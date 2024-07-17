//! Arguments for `dfx-orbit request permission permission`.

use crate::{args::request::CreateRequestArgs, StationAgent};
use clap::Parser;
use orbit_station_api::{
    CreateRequestInput, EditPermissionOperationInput, PermissionResourceActionDTO,
    RequestOperationInput, ResourceDTO,
};

/// Requests the privilige of proposing canister upgrades.
#[derive(Debug, Parser)]
pub struct RequestPermissionUpdatePermissionsArgs {
    /// A users that should be permitted to change permissions.  WARNING: Any user that is not listed will lose the ability to change permissions.
    #[structopt(long)]
    pub user: Vec<String>,
    /// A groups that should be permitted to change permissions.  WARNING: Any group that is not listed will lose the ability to change permissions.
    #[structopt(long)]
    pub group: Vec<String>,
}

impl CreateRequestArgs for RequestPermissionUpdatePermissionsArgs {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    fn into_create_request_input(
        self,
        _station_agent: &StationAgent,
    ) -> anyhow::Result<CreateRequestInput> {
        let RequestPermissionUpdatePermissionsArgs {
            user: users,
            group: user_groups,
        } = self;

        let operation = RequestOperationInput::EditPermission(EditPermissionOperationInput {
            resource: ResourceDTO::Permission(PermissionResourceActionDTO::Update),
            auth_scope: None,
            users: Some(users),
            user_groups: Some(user_groups),
        });
        Ok(CreateRequestInput {
            operation,
            title: None,
            summary: None,
            execution_plan: None,
        })
    }
}

/// Requests the privilige of proposing canister upgrades.
#[derive(Debug, Parser)]
pub struct RequestPermissionReadPermissionsArgs {
    /// A users that should be permitted to change permissions.  WARNING: Any user that is not listed will lose the ability to change permissions.
    #[structopt(long)]
    pub user: Vec<String>,
    /// A groups that should be permitted to change permissions.  WARNING: Any group that is not listed will lose the ability to change permissions.
    #[structopt(long)]
    pub group: Vec<String>,
}

impl CreateRequestArgs for RequestPermissionReadPermissionsArgs {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    fn into_create_request_input(
        self,
        _station_agent: &StationAgent,
    ) -> anyhow::Result<CreateRequestInput> {
        let RequestPermissionReadPermissionsArgs {
            user: users,
            group: user_groups,
        } = self;

        let operation = RequestOperationInput::EditPermission(EditPermissionOperationInput {
            resource: ResourceDTO::Permission(PermissionResourceActionDTO::Read),
            auth_scope: None,
            users: Some(users),
            user_groups: Some(user_groups),
        });
        Ok(CreateRequestInput {
            operation,
            title: None,
            summary: None,
            execution_plan: None,
        })
    }
}
