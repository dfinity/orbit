//! Implements `dfx review list` command.  These correspond to Orbit station `list_requests` API call.

use orbit_station_api::{ApiErrorDTO, ListRequestsInput, ListRequestsResponse};

use crate::args::review::list::Args;

/// The main entry point for the `dfx orbit review next` CLI.
pub async fn exec(args: Args) -> anyhow::Result<()> {
    let args = ListRequestsInput::from(args);
    let mut station_agent = crate::orbit_station_agent::StationAgent::new()?;
    let response_bytes = station_agent
        .update_orbit("list_requests")
        .await?
        .with_arg(candid::encode_one(args)?)
        .call_and_wait()
        .await?;
    let ans: Result<ListRequestsResponse, ApiErrorDTO> = candid::decode_one(&response_bytes)?;
    println!("{}", serde_json::to_string_pretty(&ans)?);
    Ok(())
}
