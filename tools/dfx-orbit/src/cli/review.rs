//! Implements `dfx review` commands.  These correspond to Orbit station `get_request`, approve and related API calls.

use crate::{error::StationAgentResult, StationAgent};
use orbit_station_api::{
    GetNextApprovableRequestInput, GetNextApprovableRequestResponse, GetRequestInput,
    GetRequestResponse, ListRequestsInput, ListRequestsResponse,
};

impl StationAgent {
    pub async fn review_id(
        &mut self,
        args: GetRequestInput,
    ) -> StationAgentResult<GetRequestResponse> {
        self.update_orbit_typed("get_request", args).await
    }

    pub async fn review_list(
        &mut self,
        args: ListRequestsInput,
    ) -> StationAgentResult<ListRequestsResponse> {
        self.update_orbit_typed("list_requests", args).await
    }

    pub async fn review_next(
        &mut self,
        args: GetNextApprovableRequestInput,
    ) -> StationAgentResult<GetNextApprovableRequestResponse> {
        self.update_orbit_typed("get_next_approvable_request", args)
            .await
    }
}
