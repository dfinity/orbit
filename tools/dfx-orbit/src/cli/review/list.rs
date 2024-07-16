//! Implements `dfx review list` command.  These correspond to Orbit station `list_requests` API call.

use crate::{error::StationAgentResult, StationAgent};
use orbit_station_api::{ApiErrorDTO, ListRequestsInput, ListRequestsResponse};

impl StationAgent {
    pub async fn review_list(
        &mut self,
        args: ListRequestsInput,
    ) -> StationAgentResult<ListRequestsResponse> {
        let response_bytes = self
            .update_orbit("list_requests")
            .await?
            .with_arg(candid::encode_one(args)?)
            .call_and_wait()
            .await?;
        let ans: Result<ListRequestsResponse, ApiErrorDTO> = candid::decode_one(&response_bytes)?;

        Ok(ans?)
    }
}
