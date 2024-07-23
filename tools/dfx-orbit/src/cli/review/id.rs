//! Implements `dfx review id XXXX` command.  This corresponds to Orbit station `get_request` API call.

use crate::{error::StationAgentResult, StationAgent};
use orbit_station_api::{GetRequestInput, GetRequestResponse};

impl StationAgent {
    pub async fn review_id(
        &mut self,
        args: GetRequestInput,
    ) -> StationAgentResult<GetRequestResponse> {
        self.update_orbit_typed("get_request", args).await
    }
}
