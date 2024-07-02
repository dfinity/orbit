//! CLI arguments for `dfx-orbit canister call`.

use clap::Parser;

use crate::{args::request::CreateRequestArgs, orbit_station_agent::StationAgent};

/// Requests that a call be made to a canister.
#[derive(Debug, Parser)]
pub struct Args {
    /// The canister name or ID.
    canister: String,
    /// The name of the method to call.
    method: String,
    /// The argument to pass to the canister.
    argument: Option<String>,
}

impl CreateRequestArgs for Args {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    fn into_create_request_input(
        self,
        _station_agent: &StationAgent,
    ) -> anyhow::Result<orbit_station_api::CreateRequestInput> {
        /*
        let Args {
            canister,method,argument
        } = self;
        let canister_id = station_agent.canister_id(&canister)?;
        let operation = {
            let module = std::fs::read(wasm)
                .expect("Could not read Wasm file")
                .to_vec();
            let arg = if let Some(file) = arg_file {
                Some(
                    std::fs::read(file)
                        .expect("Could not read argument file")
                        .to_vec(),
                )
            } else {
                arg.map(|arg| arg.as_bytes().to_vec())
            };
            let mode = mode.into();
            orbit_station_api::ChangeExternalCanisterOperationInput {
                canister_id,
                mode,
                module,
                arg,
            }
        };
        let operation = orbit_station_api::RequestOperationInput::ChangeExternalCanister(operation);
        Ok(orbit_station_api::CreateRequestInput {
            operation,
            title: None,
            summary: None,
            execution_plan: None,
        })
        */
        unimplemented!("Argument conversion not implemented yet")
    }
}
