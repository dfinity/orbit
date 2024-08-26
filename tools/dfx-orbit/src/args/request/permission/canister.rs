use crate::DfxOrbit;
use clap::Parser;
use station_api::{
    EditPermissionOperationInput, ExternalCanisterIdDTO, ExternalCanisterResourceActionDTO,
    RequestOperationInput, ResourceDTO,
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
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        let canisters: ExternalCanisterIdDTO = if let Some(canister_name_or_id) = self.canister {
            dfx_orbit
                .canister_id(&canister_name_or_id)
                .map(ExternalCanisterIdDTO::Canister)?
        } else {
            ExternalCanisterIdDTO::Any
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
