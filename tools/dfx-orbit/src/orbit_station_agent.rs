//! A dfx and IC agent for communicating with an Orbit station.

use anyhow::anyhow;
use candid::Principal;
use ic_agent::agent::UpdateBuilder;

use crate::{
    dfx_extension_api::DfxExtensionAgent,
    local_config::{self, StationConfig},
};

/// A dfx agent for communicating with a specific station.
pub struct StationAgent {
    /// The station to communicate with.
    pub station: StationConfig,
    /// The dfx agent.
    pub dfx: DfxExtensionAgent,
}

impl StationAgent {
    /// Creates a new agent for communicating with the default station.
    pub fn new() -> anyhow::Result<Self> {
        let dfx = DfxExtensionAgent::new(crate::ORBIT_EXTENSION_NAME);
        let station = local_config::default_station()?
            .ok_or_else(|| anyhow::format_err!("No default station specified"))?;
        Ok(Self { station, dfx })
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
    /// let response_bytes = station_agent.update_canister(&canister_id, "method_name")
    ///         .with_arg(candid::encode_one(args)?)
    ///         .call_and_wait()
    ///         .await?;
    /// ```
    pub async fn update_canister_id(
        &mut self,
        canister_id: &Principal,
        method_name: &str,
    ) -> anyhow::Result<UpdateBuilder> {
        Ok(self.dfx.agent().await?.update(canister_id, method_name))
    }

    /// Builds a canister update call to a named canister on the network used by the station.
    ///
    /// # Example
    /// ```
    /// let response_bytes = station_agent.update_canister("mycanister", "method_name")
    ///         .with_arg(candid::encode_one(args)?)
    ///         .call_and_wait()
    ///         .await?;
    /// ```
    pub async fn update_canister(
        &mut self,
        canister: &str,
        method_name: &str,
    ) -> anyhow::Result<UpdateBuilder> {
        let canister_id = self.canister_id(canister)?;
        Ok(self.dfx.agent().await?.update(&canister_id, method_name))
    }

    /// Makes an update call to the station.
    pub async fn update_orbit(&mut self, method_name: &str) -> anyhow::Result<UpdateBuilder> {
        let orbit_canister_id = crate::local_config::default_station()?
            .ok_or_else(|| anyhow!("No default station specified"))?
            .station_id;
        let orbit_canister_id = Principal::from_text(&orbit_canister_id)?;
        Ok(self
            .dfx
            .agent()
            .await?
            .update(&orbit_canister_id, method_name))
    }

    /// The URL for a request in the Orbit UI.
    pub fn request_url(&self, request_id: &str) -> String {
        format!(
            "{}/en/settings/requests?reqid={}",
            self.station.url, request_id
        )
    }
}
