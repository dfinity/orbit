//! Implement submission of request approvals / rejectsions

use crate::{error::StationAgentResult, StationAgent};
use orbit_station_api::{SubmitRequestApprovalInput, SubmitRequestApprovalResponse};

impl StationAgent {
    pub async fn submit(
        &mut self,
        args: SubmitRequestApprovalInput,
    ) -> StationAgentResult<SubmitRequestApprovalResponse> {
        self.update_orbit_typed("submit_request_approval", args)
            .await
    }
}
