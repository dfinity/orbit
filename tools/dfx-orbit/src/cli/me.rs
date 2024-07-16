//! Implementation of the `dfx-orbit me` command.

use crate::{error::StationAgentResult, StationAgent};
use anyhow::Context;
use candid::encode_args;
use orbit_station_api::{ApiErrorDTO, MeResponse};

impl StationAgent {
    pub async fn me(&mut self) -> StationAgentResult<MeResponse> {
        let ans = self
            .update_orbit("me")
            .await?
            .with_arg(empty_args())
            .call_and_wait()
            .await
            .with_context(|| "Failed to make API call")?;
        let ans: Result<MeResponse, ApiErrorDTO> =
            candid::decode_one(&ans).with_context(|| "Failed to decode response")?;

        Ok(ans?)
    }
}

/// Encodes an empty tuple as Candid.  This is used for methods with no arguments.
pub fn empty_args() -> Vec<u8> {
    encode_args(()).expect("Failed to candid encode empty tuple")
}
