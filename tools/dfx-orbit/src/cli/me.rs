//! Implementation of the `dfx-orbit me` command.

use crate::{error::StationAgentResult, StationAgent};
use orbit_station_api::MeResponse;

impl StationAgent {
    pub async fn me(&mut self) -> StationAgentResult<MeResponse> {
        self.update_orbit_typed("me", ()).await
    }
}
