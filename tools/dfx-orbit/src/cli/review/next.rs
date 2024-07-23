//! Implements `dfx review next` command.  These correspond to Orbit station `get_next_approvable_request` API call.

use crate::{error::StationAgentResult, StationAgent};
use orbit_station_api::{GetNextApprovableRequestInput, GetNextApprovableRequestResponse};

impl StationAgent {
    pub async fn review_next(
        &mut self,
        args: GetNextApprovableRequestInput,
    ) -> StationAgentResult<GetNextApprovableRequestResponse> {
        self.update_orbit_typed("get_next_approvable_request", args)
            .await
    }
}
