//! Implements `dfx review id XXXX` command.  This corresponds to Orbit station `get_request` API call.

use crate::StationAgent;
use candid::Principal;
use orbit_station_api::{ApiErrorDTO, GetRequestInput, GetRequestResponse};

impl StationAgent {
    pub async fn review_id(&mut self, args: GetRequestInput) -> anyhow::Result<()> {
        let ic_agent = self.dfx.agent().await?;

        // The station canister ID to which we will make the API call.
        let orbit_canister_id = &self.station.station_id;
        let canister_id = Principal::from_text(orbit_canister_id)?;

        let response_bytes = ic_agent
            .update(&canister_id, "get_request")
            .with_arg(candid::encode_one(args)?)
            .call_and_wait()
            .await?;
        let ans: Result<GetRequestResponse, ApiErrorDTO> = candid::decode_one(&response_bytes)?;

        println!("{ans:#?}");
        Ok(())
    }
}
