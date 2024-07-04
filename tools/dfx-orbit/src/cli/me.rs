//! Implementation of the `dfx-orbit me` command.

use anyhow::Context;
use candid::encode_args;
use orbit_station_api::{ApiErrorDTO, MeResponse};

/// A command line tool for interacting with Orbit on the Internet Computer.
pub async fn exec() -> anyhow::Result<()> {
    let mut station_agent = crate::orbit_station_agent::StationAgent::new()?;
    let ans = station_agent
        .update_orbit("me")
        .await?
        .with_arg(empty_args())
        .call_and_wait()
        .await
        .with_context(|| "Failed to make API call")?;
    let ans: Result<MeResponse, ApiErrorDTO> =
        candid::decode_one(&ans).with_context(|| "Failed to decode response")?;
    println!(
        "{}",
        serde_json::to_string_pretty(&ans)
            .with_context(|| "Failed to serialize response as JSON")?
    );
    Ok(())
}

/// Encodes an empty tuple as Candid.  This is used for methods with no arguments.
pub fn empty_args() -> Vec<u8> {
    encode_args(()).expect("Failed to candid encode empty tuple")
}
