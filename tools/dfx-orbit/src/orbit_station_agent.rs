//! A dfx and IC agent for communicating with an Orbit station.

use crate::{dfx_extension_api::OrbitExtensionAgent, error::StationAgentResult, StationAgent};
use candid::{CandidType, Principal};
use ic_agent::{agent::UpdateBuilder, Agent};
use orbit_station_api::ApiErrorDTO;

impl StationAgent {
    /// Creates a new agent for communicating with the default station.
    pub async fn new(mut agent: OrbitExtensionAgent) -> anyhow::Result<Self> {
        let station = agent
            .default_station()?
            .ok_or_else(|| anyhow::format_err!("No default station specified"))?;
        let interface = agent.dfx_interface().await?;

        Ok(Self {
            station,
            dfx: agent,
            interface,
        })
    }

    /// Gets the ID of a given canister name.  If the name is already an ID, it is returned as is.
    pub fn canister_id(&self, canister_name: &str) -> anyhow::Result<Principal> {
        let network = &self.station.network;
        self.dfx.canister_id(canister_name, network)
    }

    /// Builds a canister update call on the network used by the station.
    ///
    /// # Example
    /// ```
    /// let response_bytes = station_agent.update_canister(&canister_id, "method_name").await
    ///         .with_arg(candid::encode_one(args)?)
    ///         .call_and_wait()
    ///         .await?;
    /// ```
    pub async fn update_canister_id(
        &mut self,
        canister_id: &Principal,
        method_name: &str,
    ) -> anyhow::Result<UpdateBuilder> {
        Ok(self.agent().update(canister_id, method_name))
    }

    /// Builds a canister update call to a named canister on the network used by the station.
    ///
    /// # Example
    /// ```
    /// let response_bytes = station_agent.update_canister("mycanister", "method_name").await
    ///         .with_arg(candid::encode_one(args)?)
    ///         .call_and_wait()
    ///         .await?;
    /// ```
    pub fn update_canister(
        &mut self,
        canister: &str,
        method_name: &str,
    ) -> anyhow::Result<UpdateBuilder> {
        let canister_id = self.canister_id(canister)?;
        Ok(self.agent().update(&canister_id, method_name))
    }

    /// Makes an update call to the station.
    ///
    /// # Example
    /// ```
    /// let response_bytes = station_agent.update_canister("mycanister", "method_name").await
    ///         .with_arg(candid::encode_one(args)?)
    ///         .call_and_wait()
    ///         .await?;
    /// ```
    pub async fn update_orbit(&mut self, method_name: &str) -> anyhow::Result<UpdateBuilder> {
        let orbit_canister_id = Principal::from_text(&self.station.station_id)?;
        Ok(self.agent().update(&orbit_canister_id, method_name))
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
            self.station.url, request_id
        )
    }

    /// Gets the dfx agent.
    pub fn agent(&self) -> &Agent {
        self.interface.agent()
    }
}
