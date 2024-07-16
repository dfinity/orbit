//! Library for interacting with Orbit on the Internet Computer.
//#![warn(missing_docs)]
//#![warn(clippy::missing_docs_in_private_items)]
//#![deny(clippy::panic)]
//#![deny(clippy::unwrap_used)]

pub mod args;
pub mod cli;
pub mod dfx_extension_api;
pub mod local_config;
pub mod orbit_station_agent;

use dfx_extension_api::DfxExtensionAgent;
use local_config::StationConfig;

/// The name of the Orbit dfx extension.
pub const ORBIT_EXTENSION_NAME: &str = "orbit";

/// A dfx agent for communicating with a specific station.
pub struct StationAgent {
    /// The station to communicate with.
    pub station: StationConfig,
    /// The dfx agent.
    pub dfx: DfxExtensionAgent,
}
