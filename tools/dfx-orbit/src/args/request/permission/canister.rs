//! Makes `EditPermission` requests regarding `ExternalCanister` to Orbit.
use candid::Principal;
use clap::{Parser, Subcommand};

/// Request canister changes.
#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum Args {
    /// Request changes to canister permissions.
    Change(ChangeCanister),
}

/// Requests the privilige of proposing canister upgrades.
#[derive(Debug, Parser)]
pub struct ChangeCanister {
    /// Canister name or ID.
    #[structopt(long)]
    pub canister: Option<String>,
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
