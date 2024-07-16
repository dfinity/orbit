//! Implements `dfx review id XXXX` command.  This corresponds to Orbit station `get_request` API call.

use crate::{args::review::id::Args, StationAgent};
use anyhow::anyhow;
use candid::Principal;
use orbit_station_api::{ApiErrorDTO, GetRequestInput, GetRequestResponse};

/// The main entry point for the `dfx orbit review next` CLI.
pub async fn exec(args: Args) -> anyhow::Result<()> {
    let args = GetRequestInput::from(args);
    let mut station_agent = StationAgent::new()?;
    let ic_agent = station_agent.dfx.agent().await?;
    // The station canister ID to which we will make the API call.
    let orbit_canister_id = crate::local_config::default_station()?
        .ok_or_else(|| anyhow!("No default station specified"))?
        .station_id;
    let canister_id = Principal::from_text(&orbit_canister_id)?;
    let response_bytes = ic_agent
        .update(&canister_id, "get_request")
        .with_arg(candid::encode_one(args)?)
        .call_and_wait()
        .await?;
    let ans: Result<GetRequestResponse, ApiErrorDTO> = candid::decode_one(&response_bytes)?;
    println!("{ans:#?}");
    Ok(())
}
