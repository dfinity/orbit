//! A dfx and IC agent for communicating with an Orbit station.

use crate::{error::StationAgentResult, local_config::StationConfig};
use candid::{CandidType, Principal};
use ic_agent::{agent::UpdateBuilder, Agent};
use orbit_station_api::ApiErrorDTO;

/// A dfx agent for communicating with a specific station.
pub struct StationAgent {
    /// The station to communicate with.
    pub config: StationConfig,
    pub agent: Agent,
}

impl StationAgent {
    pub fn new(agent: Agent, config: StationConfig) -> Self {
        Self { config, agent }
    }

    pub async fn update_canister_id(
        &mut self,
        canister_id: &Principal,
        method_name: &str,
    ) -> anyhow::Result<UpdateBuilder> {
        Ok(self.agent.update(canister_id, method_name))
    }

    // pub fn update_canister(
    //     &mut self,
    //     canister: &str,
    //     method_name: &str,
    // ) -> anyhow::Result<UpdateBuilder> {
    //     let canister_id = self.canister_id(canister)?;
    //     Ok(self.agent().update(&canister_id, method_name))
    // }

    async fn update_orbit(&mut self, method_name: &str) -> anyhow::Result<UpdateBuilder> {
        let orbit_canister_id = Principal::from_text(&self.config.station_id)?;
        Ok(self.agent.update(&orbit_canister_id, method_name))
    }

    /// Makes an update call to the station.
    ///
    /// This version integrates candid encoding / decoding
    pub async fn update_orbit_typed<Req, Res>(
        &mut self,
        method_name: &str,
        request: Req,
    ) -> StationAgentResult<Res>
    where
        Req: CandidType,
        Res: CandidType + for<'a> candid::Deserialize<'a>,
    {
        let encoded_request = candid::encode_one(request)?;

        let response_bytes = self
            .update_orbit(method_name)
            .await?
            .with_arg(encoded_request)
            .call_and_wait()
            .await?;
        let ans: Result<Res, ApiErrorDTO> = candid::decode_one(&response_bytes)?;

        Ok(ans?)
    }

    /// The URL for a request in the Orbit UI.
    pub fn request_url(&self, request_id: &str) -> String {
        format!(
            "{}/en/settings/requests?reqid={}",
            self.config.url, request_id
        )
    }
}
