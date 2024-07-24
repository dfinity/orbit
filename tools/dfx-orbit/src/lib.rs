//! Library for interacting with Orbit on the Internet Computer.
//#![warn(missing_docs)]
//#![warn(clippy::missing_docs_in_private_items)]
//#![deny(clippy::panic)]
//#![deny(clippy::unwrap_used)]

pub mod args;
pub mod cli;
pub mod dfx_extension_api;
pub mod error;
pub mod local_config;
pub mod orbit_station_agent;

use dfx_core::DfxInterface;
use dfx_extension_api::OrbitExtensionAgent;
use local_config::StationConfig;

/// A dfx agent for communicating with a specific station.
pub struct StationAgent {
    /// The station to communicate with.
    pub station: StationConfig,
    /// The dfx agent.
    pub dfx: OrbitExtensionAgent,
    // The dfx interface
    pub interface: DfxInterface,
}
