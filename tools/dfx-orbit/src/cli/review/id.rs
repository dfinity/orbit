//! Implements `dfx review id XXXX` command.  This corresponds to Orbit station `get_request` API call.

use crate::{error::StationAgentResult, StationAgent};
use anyhow::Context;
use candid::Principal;
use orbit_station_api::{ApiErrorDTO, GetRequestInput, GetRequestResponse};

impl StationAgent {
    pub async fn review_id(
        &mut self,
        args: GetRequestInput,
    ) -> StationAgentResult<GetRequestResponse> {
        let ic_agent = self.dfx.agent().await?;

        // The station canister ID to which we will make the API call.
        let canister_id = Principal::from_text(&self.station.station_id)
            .with_context(|| "failed to parse principal")?;

        let response_bytes: Vec<u8> = ic_agent
            .update(&canister_id, "get_request")
            .with_arg(candid::encode_one(args)?)
            .call_and_wait()
            .await?;

        let ans = candid::decode_one::<Result<GetRequestResponse, ApiErrorDTO>>(&response_bytes)?;

        println!("{ans:#?}");
        Ok(ans?)
    }
}
