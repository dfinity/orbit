//! Implements `dfx request` commands.  These correspond to Orbit station `create_request` API calls.

use crate::{error::StationAgentResult, StationAgent};
use orbit_station_api::{ApiErrorDTO, CreateRequestInput, CreateRequestResponse};

impl StationAgent {
    pub async fn request(
        &mut self,
        input: CreateRequestInput,
    ) -> StationAgentResult<CreateRequestResponse> {
        // Makes an update call to the station.
        let response_bytes = self
            .update_orbit("create_request")
            .await?
            .with_arg(candid::encode_one(input)?)
            .call_and_wait()
            .await?;

        // Decodes the response from the station.
        let ans: Result<CreateRequestResponse, ApiErrorDTO> = candid::decode_one(&response_bytes)?;
        Ok(ans?)
    }
}
