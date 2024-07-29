//! Library for interacting with Orbit on the Internet Computer.
//#![warn(missing_docs)]
//#![warn(clippy::missing_docs_in_private_items)]
//#![deny(clippy::panic)]
//#![deny(clippy::unwrap_used)]

pub mod args;
pub mod cli;
pub mod dfx_extension_api;
pub mod local_config;
pub mod station_agent;

use candid::Principal;
use dfx_core::DfxInterface;
use dfx_extension_api::OrbitExtensionAgent;
pub use station_agent::StationAgent;

pub struct DfxOrbit {
    // The station agent that handles communication with the station
    pub station: StationAgent,
    /// The dfx agent.
    pub dfx: OrbitExtensionAgent,
    // The dfx interface
    pub interface: DfxInterface,
}

impl DfxOrbit {
    /// Creates a new agent for communicating with the default station.
    pub async fn new(mut agent: OrbitExtensionAgent) -> anyhow::Result<Self> {
        let config = agent
            .default_station()?
            .ok_or_else(|| anyhow::format_err!("No default station specified"))?;
        let interface = agent.dfx_interface().await?;

        Ok(Self {
            station: StationAgent::new(interface.agent().clone(), config),
            dfx: agent,
            interface,
        })
    }

    // TODO: Remove and put this into the upper struct
    /// Gets the ID of a given canister name.  If the name is already an ID, it is returned as is.
    pub fn canister_id(&self, canister_name: &str) -> anyhow::Result<Principal> {
        let network = &self.station.config.network;
        self.dfx.canister_id(canister_name, network)
    }
}
