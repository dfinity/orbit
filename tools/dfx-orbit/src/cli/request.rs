//! Implements `dfx request` commands.  These correspond to Orbit station `create_request` API calls.

use crate::args::request::{Args, CreateRequestArgs};
use orbit_station_api::{ApiErrorDTO, CreateRequestResponse};

/// The main entry point for the `dfx orbit request` CLI.
pub async fn exec(args: Args) -> anyhow::Result<Result<CreateRequestResponse, ApiErrorDTO>> {
    // Converts the CLI arg type into the equivalent Orbit API type.
    let mut station_agent = crate::orbit_station_agent::StationAgent::new()?;
    let args = args.into_create_request_input(&station_agent)?;
    let response_bytes = station_agent
        .update_orbit("create_request")
        .await?
        .with_arg(candid::encode_one(args)?)
        .call_and_wait()
        .await?;
    let ans: Result<CreateRequestResponse, ApiErrorDTO> = candid::decode_one(&response_bytes)?;
    if let Ok(response) = &ans {
        let request_id = &response.request.id;
        let request_url = station_agent.request_url(request_id);
        println!("Created request: {request_id}");
        println!("Request URL: {request_url}");
        println!("To view the request, run: dfx-orbit review id {request_id}");
    } else {
        println!("{ans:#?}");
    }
    Ok(ans)
}
