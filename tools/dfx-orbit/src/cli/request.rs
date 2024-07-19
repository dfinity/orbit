//! Implements `dfx request` commands.  These correspond to Orbit station `create_request` API calls.

use crate::{error::StationAgentResult, StationAgent};
use orbit_station_api::{CreateRequestInput, CreateRequestResponse};

impl StationAgent {
    pub async fn request(
        &mut self,
        input: CreateRequestInput,
    ) -> StationAgentResult<CreateRequestResponse> {
        self.update_orbit_typed("create_request", input).await
    }
}
