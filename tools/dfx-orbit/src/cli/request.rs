//! Implements `dfx request` commands.  These correspond to Orbit station `create_request` API calls.

use crate::args::request::{Args, CreateRequestArgs};
use anyhow::anyhow;
use candid::Principal;
use orbit_station_api::{ApiErrorDTO, CreateRequestResponse};

/// The main entry point for the `dfx orbit request` CLI.
pub async fn exec(args: Args) -> anyhow::Result<Result<CreateRequestResponse, ApiErrorDTO>> {
    // Converts the CLI arg type into the equivalent Orbit API type.
    let mut station_agent = crate::orbit_station_agent::StationAgent::new()?;
    let args = args.into_create_request_input(&station_agent)?;
    let ic_agent = station_agent.dfx.agent().await?;
    let orbit_canister_id = crate::local_config::default_station()?
        .ok_or_else(|| anyhow!("No default station specified"))?
        .canister_id;

    let canister_id = Principal::from_text(&orbit_canister_id)?;
    let bytes = ic_agent
        .update(&canister_id, "create_request")
        .with_arg(candid::encode_one(args)?)
        .call_and_wait()
        .await?;
    let ans: Result<CreateRequestResponse, ApiErrorDTO> = candid::decode_one(&bytes)?;
    println!("{ans:#?}");
    Ok(ans)
}
