//! CLI arguments for `dfx-orbit canister call`.

use clap::Parser;
use orbit_station_api::{CallExternalCanisterOperationInput, CanisterMethodDTO};

use crate::{args::request::CreateRequestArgs, orbit_station_agent::StationAgent};

/// Requests that a call be made to a canister.
#[derive(Debug, Parser)]
pub struct Args {
    /// The canister name or ID.
    canister: String,
    /// The name of the method to call.
    method_name: String,
    // TODO:
    // /// The argument to pass to the canister.
    // argument: Option<String>,
    // TODO:
    // /// The format of the argument.
    // #[clap(short, long)]
    // r#type: Option<CandidFormat>,
    // TODO: Read argument from a file
    /// Specifies the amount of cycles to send on the call.
    #[clap(short, long)]
    with_cycles: Option<u64>,
}

impl CreateRequestArgs for Args {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    fn into_create_request_input(
        self,
        station_agent: &StationAgent,
    ) -> anyhow::Result<orbit_station_api::CreateRequestInput> {
        let Args {
            canister,
            method_name,
            with_cycles,
        } = self;
        let canister_id = station_agent.canister_id(&canister)?;
        let operation = orbit_station_api::RequestOperationInput::CallExternalCanister(
            CallExternalCanisterOperationInput {
                validation_method: None,
                execution_method: CanisterMethodDTO {
                    canister_id,
                    method_name,
                },
                arg: None,
                execution_method_cycles: with_cycles,
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
