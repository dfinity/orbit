//! Library for interacting with Orbit on the Internet Computer.
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![deny(clippy::panic)]
#![deny(clippy::unwrap_used)]
pub mod args;
pub mod cli;
pub mod dfx_extension_api;
pub mod local_config;
pub mod orbit_station_agent;

/// The name of the Orbit dfx extension.
pub const ORBIT_EXTENSION_NAME: &str = "orbit";
