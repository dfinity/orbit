use crate::StationAgent;
use clap::Parser;
use orbit_station_api::{
    ChangeExternalCanisterResourceTargetDTO, EditPermissionOperationInput,
    ExternalCanisterResourceActionDTO, RequestOperationInput, ResourceDTO,
};

/// Requests the permisson to propose canister upgrades.
#[derive(Debug, Clone, Parser)]
pub struct RequestPermissionUpgradeCanisterArgs {
    /// Canister name or ID. If none specified, this will request all
    // TODO: If a canister is not specified, require --all.
    pub canister: Option<String>,
}

impl RequestPermissionUpgradeCanisterArgs {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    pub(crate) fn into_create_request_input(
        self,
        station_agent: &StationAgent,
    ) -> anyhow::Result<RequestOperationInput> {
        let canisters: ChangeExternalCanisterResourceTargetDTO =
            if let Some(canister_name_or_id) = self.canister {
                station_agent
                    .canister_id(&canister_name_or_id)
                    .map(ChangeExternalCanisterResourceTargetDTO::Canister)?
            } else {
                ChangeExternalCanisterResourceTargetDTO::Any
            };

        let resource =
            ResourceDTO::ExternalCanister(ExternalCanisterResourceActionDTO::Change(canisters));

        Ok(RequestOperationInput::EditPermission(
            EditPermissionOperationInput {
                resource,
                auth_scope: None,
                users: None,
                user_groups: None,
            },
        ))
    }
}
