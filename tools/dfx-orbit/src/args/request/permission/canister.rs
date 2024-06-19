//! Makes `EditPermission` requests regarding `ExternalCanister` to Orbit.
use candid::Principal;
use clap::{Parser, Subcommand};

use crate::{args::request::CreateRequestArgs, orbit_station_agent::StationAgent};

/// Request canister changes.
#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum Args {
    /// Request changes to canister permissions.
    Change(ChangeCanister),
}

impl CreateRequestArgs for Args {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    fn into_create_request_input(
        self,
        station_agent: &StationAgent,
    ) -> anyhow::Result<orbit_station_api::CreateRequestInput> {
        match self {
            Args::Change(change_args) => change_args.into_create_request_input(station_agent),
        }
    }
}

/// Requests the privilige of proposing canister upgrades.
#[derive(Debug, Parser)]
pub struct ChangeCanister {
    /// Canister name or ID.
    #[structopt(long)]
    pub canister: Option<String>,
}

impl CreateRequestArgs for ChangeCanister {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    fn into_create_request_input(
        self,
        station_agent: &StationAgent,
    ) -> anyhow::Result<orbit_station_api::CreateRequestInput> {
        let canisters: anyhow::Result<orbit_station_api::ChangeExternalCanisterResourceTargetDTO> =
            if let Some(canister_name_or_id) = self.canister {
                station_agent
                    .canister_id(&canister_name_or_id)
                    .map(orbit_station_api::ChangeExternalCanisterResourceTargetDTO::Canister)
            } else {
                Ok(orbit_station_api::ChangeExternalCanisterResourceTargetDTO::Any)
            };

        let resource = orbit_station_api::ResourceDTO::ExternalCanister(
            orbit_station_api::ExternalCanisterResourceActionDTO::Change(canisters?),
        );

        let operation = orbit_station_api::RequestOperationInput::EditPermission(
            orbit_station_api::EditPermissionOperationInput {
                resource,
                auth_scope: None,
                users: None,
                user_groups: None,
            },
        );
        Ok(orbit_station_api::CreateRequestInput {
            operation,
            title: None,
            summary: None,
            execution_plan: None,
        })
    }
}

impl TryFrom<Args> for orbit_station_api::RequestOperationInput {
    type Error = anyhow::Error;
    fn try_from(args: Args) -> anyhow::Result<Self> {
        match args {
            Args::Change(change_args) => Ok(
                orbit_station_api::RequestOperationInput::EditPermission(change_args.try_into()?),
            ),
        }
    }
}

impl TryFrom<ChangeCanister> for orbit_station_api::EditPermissionOperationInput {
    type Error = anyhow::Error;

    fn try_from(args: ChangeCanister) -> anyhow::Result<Self> {
        if let Some(canister_name_or_id) = args.canister {
            // Grant permission for just this one canister.
            let canister_id = Principal::from_text(canister_name_or_id)?;
            let resource = orbit_station_api::ResourceDTO::ExternalCanister(
                orbit_station_api::ExternalCanisterResourceActionDTO::Change(
                    orbit_station_api::ChangeExternalCanisterResourceTargetDTO::Canister(
                        canister_id,
                    ),
                ),
            );
            Ok(orbit_station_api::EditPermissionOperationInput {
                resource,
                auth_scope: None,
                users: None,
                user_groups: None,
            })
        } else {
            unimplemented!("Need to implement granting access to all canisters.")
        }
    }
}
