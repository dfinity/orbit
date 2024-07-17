use crate::{args::request::CreateRequestArgs, StationAgent};
use clap::Parser;
use orbit_station_api::{
    ChangeExternalCanisterResourceTargetDTO, CreateRequestInput, EditPermissionOperationInput,
    ExternalCanisterResourceActionDTO, RequestOperationInput, ResourceDTO,
};

/// Requests the privilige of proposing canister upgrades.
#[derive(Debug, Parser)]
pub struct RequestPermissionUpdateCanisterArgs {
    /// Canister name or ID.
    // TODO: If a canister is not specified, require --all.
    #[structopt(long)]
    pub canister: Option<String>,
}

impl CreateRequestArgs for RequestPermissionUpdateCanisterArgs {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    fn into_create_request_input(
        self,
        station_agent: &StationAgent,
    ) -> anyhow::Result<CreateRequestInput> {
        let canisters: anyhow::Result<ChangeExternalCanisterResourceTargetDTO> =
            if let Some(canister_name_or_id) = self.canister {
                station_agent
                    .canister_id(&canister_name_or_id)
                    .map(ChangeExternalCanisterResourceTargetDTO::Canister)
            } else {
                Ok(ChangeExternalCanisterResourceTargetDTO::Any)
            };

        let resource =
            ResourceDTO::ExternalCanister(ExternalCanisterResourceActionDTO::Change(canisters?));

        let operation = RequestOperationInput::EditPermission(EditPermissionOperationInput {
            resource,
            auth_scope: None,
            users: None,
            user_groups: None,
        });
        Ok(CreateRequestInput {
            operation,
            title: None,
            summary: None,
            execution_plan: None,
        })
    }
}
