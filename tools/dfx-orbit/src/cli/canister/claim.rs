//! Command to put a canister under Orbit control.
use crate::{dfx_extension_api, error::StationAgentResult, StationAgent};

impl StationAgent {
    pub async fn claim_canister(
        &mut self,
        canister: String,
        exclusive: bool,
    ) -> StationAgentResult<()> {
        let station_id = &self.station.station_id;
        let network = &self.station.network;

        let claim_type = if exclusive {
            "--set-controller"
        } else {
            "--add-controller"
        };

        // TODO: Implement this without calling the `dfx` executable.
        let result = dfx_extension_api::call_dfx_cli(vec![
            "canister",
            "update-settings",
            "--network",
            &network,
            claim_type,
            station_id,
            &canister,
        ])?;
        println!("{}", result);

        Ok(())
    }
}
