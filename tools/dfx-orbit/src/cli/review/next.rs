//! Implements `dfx review next` command.  These correspond to Orbit station `get_next_approvable_request` API call.

use crate::{error::StationAgentResult, StationAgent};
use anyhow::Context;
use candid::Principal;
use orbit_station_api::{
    ApiErrorDTO, GetNextApprovableRequestInput, GetNextApprovableRequestResponse,
};

impl StationAgent {
    pub async fn review_next(
        &mut self,
        args: GetNextApprovableRequestInput,
    ) -> StationAgentResult<GetNextApprovableRequestResponse> {
        let ic_agent = self.dfx.agent().await?;

        // The station canister ID to which we will make the API call.
        let canister_id = Principal::from_text(&self.station.station_id)
            .with_context(|| "failed to parse principal")?;

        let response_bytes = ic_agent
            .update(&canister_id, "get_next_approvable_request")
            .with_arg(candid::encode_one(args)?)
            .call_and_wait()
            .await?;

        let ans: Result<GetNextApprovableRequestResponse, ApiErrorDTO> =
            candid::decode_one(&response_bytes)?;

        Ok(ans?)
    }
}
