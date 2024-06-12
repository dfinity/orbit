//! Local dfx configuration of Orbit stations.
use serde::{Deserialize, Serialize};

/// Configuration that lives in e.g. ~/.config/dfx/orbit.json
#[derive(Debug, Serialize, Deserialize)]
pub struct CommonConfig {
    /// Default station name.
    pub default_station: String,
}

/// Configuration for a given station that lives in e.g. ~/.config/dfx/orbit/stations/<station_name>.json
#[derive(Debug, Serialize, Deserialize)]
pub struct StationConfig {
    /// Station name.
    pub name: String,
    /// Wallet canister ID.
    pub canister_id: String,
}

/// Lists all Orbit stations in the local dfx configuration.
pub fn list_stations() -> Vec<StationConfig> {
    todo!()
}
