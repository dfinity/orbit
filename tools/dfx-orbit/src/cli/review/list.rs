//! Implements `dfx review list` command.  These correspond to Orbit station `list_requests` API call.

use crate::{error::StationAgentResult, StationAgent};
use orbit_station_api::{ListRequestsInput, ListRequestsResponse};

impl StationAgent {
    pub async fn review_list(
        &mut self,
        args: ListRequestsInput,
    ) -> StationAgentResult<ListRequestsResponse> {
        self.update_orbit_typed("list_requests", args).await
    }
}
