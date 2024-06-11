//! Local dfx configuration of Orbit stations.

/// Configuration that lives in e.g. ~/.config/dfx/orbit.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CommonConfig {
    /// Default station name.
    pub default_station: String,
}

/// Configuration for a given station that lives in e.g. ~/.config/dfx/orbit/stations/<station_name>.json
pub struct StationConfig {
    /// Station name.
    pub name: String,
    /// Wallet canister ID.
    pub canister_id: String,
}
