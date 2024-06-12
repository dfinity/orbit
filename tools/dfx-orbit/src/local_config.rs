//! Local dfx configuration of Orbit stations.
use serde::{Deserialize, Serialize};

use crate::dfx_extension_api::DfxExtensionAgent;

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

/// The directoy in the orbit dfx config directory where stations are stored.
pub const STATIONS_DIR: &str = "stations";
/// The directory in the orbit dfx config directory where stations are recorded.
pub fn stations_dir() -> anyhow::Result<cap_std::fs::Dir> {
    let dfx_extension_agent = DfxExtensionAgent::new("orbit");
    let config_dir = dfx_extension_agent
        .extension_config_dir()
        .expect("Failed to get extension config dir");
    config_dir.create_dir_all(STATIONS_DIR)?;
    let stations_dir = config_dir
        .open_dir(STATIONS_DIR)
        .expect("Failed to open stations dir");
    Ok(stations_dir)
}
/// The file in which the config for a particular station is stored.
pub fn station_file(name: &str) -> anyhow::Result<cap_std::fs::File> {
    let basename = format!("{}.json", name);
    let stations_dir = stations_dir()?;
    let station_file = stations_dir
        .open(basename)
        .expect("Failed to open station file");
    Ok(station_file)
}

/// Lists all Orbit stations in the local dfx configuration.
pub fn list_stations() -> Vec<String> {
    // Get all entries in the station dir that are valid station configs.
    let stations_dir = stations_dir().expect("Failed to get stations dir");
    stations_dir
        .entries()
        .expect("Failed to read stations dir")
        // Filter out directory entries that could not be read.  (Maybe we have no permissions to access the file or something like that?)
        .filter_map(|entry| entry.ok())
        // Filter out entries that are not files.
        .filter(|dir_entry| {
            dir_entry
                .file_type()
                .expect("Failed to get file type")
                .is_file()
        })
        // Filter out entries that don't have the .json suffix.  Return the filename without the suffix.  This is the station name.
        .filter_map(|dir_entry| {
            dir_entry
                .file_name()
                .to_string_lossy()
                .strip_suffix(".json")
                .map(|name| name.to_string())
        })
        // Filter out entries that are not valid station configs.
        .filter(|station_name| station(station_name).is_ok())
        .collect()
}

/// Gets the local stored dfx configuration for a given station.
pub fn station(name: &str) -> anyhow::Result<StationConfig> {
    let station_file = station_file(name)?;
    let station: StationConfig =
        serde_json::from_reader(station_file).expect("Failed to parse station file");
    Ok(station)
}
