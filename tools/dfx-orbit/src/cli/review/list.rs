//! Implements `dfx review list` command.  These correspond to Orbit station `list_requests` API call.

use crate::{error::StationAgentResult, StationAgent};
use anyhow::Context;
use orbit_station_api::{ApiErrorDTO, ListRequestsInput, ListRequestsResponse};

impl StationAgent {
    pub async fn review_list(
        &mut self,
        args: ListRequestsInput,
    ) -> StationAgentResult<ListRequestsResponse> {
        let response_bytes = self
            .update_orbit("list_requests")
            .await?
            .with_arg(candid::encode_one(args).with_context(|| "failed to encode candid")?)
            .call_and_wait()
            .await
            .with_context(|| "failed to call station")?;
        let ans: Result<ListRequestsResponse, ApiErrorDTO> =
            candid::decode_one(&response_bytes).with_context(|| "failed to decode candid")?;

        Ok(ans?)
    }
}
