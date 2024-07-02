//! Local dfx configuration of Orbit stations.
use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{args::station::Add, dfx_extension_api::DfxExtensionAgent};

/// Configuration that lives in e.g. ~/.config/dfx/orbit.json
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExtensionConfig {
    /// Default station name.
    pub default_station: Option<String>,
}

/// Configuration for a given station that lives in e.g. ~/.config/dfx/orbit/stations/<station_name>.json
#[derive(Debug, Serialize, Deserialize)]
pub struct StationConfig {
    /// Station name.
    pub name: String,
    /// Wallet canister ID.
    // TODO: This should be a principal.
    pub station_id: String,
    /// The dfx network name.
    pub network: String,
    /// The Orbit user interface URL.
    // TODO: This would be better as URL.  That requires serde to be implemented for URL.  Consider: https://docs.rs/url_serde/latest/url_serde/
    pub url: String,
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
/// The name of the file in which the config for a given station is stored.
pub fn station_file_name(name: &str) -> String {
    format!("{}.json", name)
}
/// The file in which the config for a particular station is stored.
///
/// If the file does not exist, this will return an error.
pub fn station_file(name: &str) -> anyhow::Result<cap_std::fs::File> {
    open_station_file(name, false).with_context(|| {
        format!("Failed to open station file for station '{name}':  Is the station name correct?")
    })
}

/// Creates and returne file in which the config for a particular station is stored.
///
/// If the file already exists, this will return an error.
pub fn create_station_file(name: &str) -> anyhow::Result<cap_std::fs::File> {
    open_station_file(name, true).with_context(|| {
        format!("Failed to create station file for station '{name}'.  Does it already exist?")
    })
}

/// The file in which the config for a particular station is stored.
///
/// Optionally create the file if it does not exist.
pub fn open_station_file(name: &str, create_new: bool) -> anyhow::Result<cap_std::fs::File> {
    let basename = station_file_name(name);
    let stations_dir = stations_dir()?;
    let mut open_options = cap_std::fs::OpenOptions::new();
    let open_options = open_options.read(true).write(true).create_new(create_new);
    let file = stations_dir.open_with(basename, open_options)?;
    Ok(file)
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

/// Adds a new Orbit station to the local dfx configuration.
///
/// If there is no default station, the new station is set as the default.
// TODO: Check that the URL works & is the root URL.
pub fn add_station(args: &Add) -> anyhow::Result<()> {
    let Add {
        name,
        station_id,
        network,
        url,
    } = args;
    let station = StationConfig {
        name: name.to_string(),
        station_id: station_id.to_string(),
        network: network.to_string(),
        url: url.to_string(),
    };
    let station_file = create_station_file(name)?;
    station_file.set_len(0)?;
    serde_json::to_writer_pretty(station_file, &station).expect("Failed to write station file");

    if default_station_name()?.is_none() {
        set_default_station(Some(name.to_string()))?;
    }

    Ok(())
}

/// Gets the local stored dfx configuration for a given station, or the default station if none is specified.
pub fn station_or_default(name: Option<&str>) -> anyhow::Result<StationConfig> {
    if let Some(name) = name {
        station(name)
    } else {
        let name = default_station()
            .with_context(|| "Station not specified and failed to get default.")?
            .with_context(|| "Station not specified and no default station set.")?
            .name;
        station(&name)
    }
}

/// Gets the local stored dfx configuration for a given station.
pub fn station(name: &str) -> anyhow::Result<StationConfig> {
    let station_file = station_file(name)?;
    let station: StationConfig =
        serde_json::from_reader(station_file).with_context(|| "Failed to parse station file")?;
    Ok(station)
}

/// Removes an Orbit station from the local dfx configuration.
pub fn remove_station(name: &str) -> anyhow::Result<()> {
    let dir = stations_dir()?;
    let path = station_file_name(name);
    dir.remove_file(path)
        .with_context(|| format!("Failed to remove dfx config file for station {}", name))?;

    if default_station_name()? == Some(name.to_string()) {
        set_default_station(None)?;
    }
    Ok(())
}

/// Renames an Orbit station in the local dfx configuration.
///
/// If the station being renamed is the default station, the default is updated to reflect the new name.
pub fn rename_station(name: &str, new_name: &str) -> anyhow::Result<()> {
    let dir = stations_dir()?;
    let old_path = station_file_name(name);
    let new_path = station_file_name(new_name);
    dir.rename(old_path, &dir, new_path).with_context(|| {
        format!(
            "Failed to rename dfx config file for station {} to {}",
            name, new_name
        )
    })?;

    if default_station_name()? == Some(name.to_string()) {
        set_default_station(Some(new_name.to_string()))?;
    }
    Ok(())
}

/// Gets the common configuration for this dfx extension.
///
/// If the config does not exist or is empty, default values are assumed.
pub fn extension_config() -> anyhow::Result<ExtensionConfig> {
    // TODO: Make orbit a const
    let dfx_extension_agent = DfxExtensionAgent::new("orbit");
    let common_config_file = dfx_extension_agent.extension_config_file()?;
    if common_config_file.metadata()?.len() == 0 {
        Ok(ExtensionConfig::default())
    } else {
        serde_json::from_reader(common_config_file)
            .with_context(|| "Failed to parse extension config file as JSON.")
    }
}

/// Gets the name of the default Orbit station from the local dfx configuration.
pub fn default_station_name() -> anyhow::Result<Option<String>> {
    Ok(extension_config()?.default_station)
}

/// Gets the default Orbit station from the local dfx configuration.
pub fn default_station() -> anyhow::Result<Option<StationConfig>> {
    if let Some(name) = default_station_name()? {
        Ok(Some(station(&name)?))
    } else {
        Ok(None)
    }
}

/// Sets the default Orbit station in the local dfx configuration.
pub fn set_default_station(name_maybe: Option<String>) -> anyhow::Result<()> {
    // Check if the station exists.
    if let Some(name) = &name_maybe {
        station(name)?;
    }
    // Set the default station.
    let mut extension_config = extension_config()?;
    extension_config.default_station = name_maybe;
    let dfx_extension_agent = DfxExtensionAgent::new("orbit");
    let common_config_file = dfx_extension_agent.extension_config_file()?;
    // TODO: Update atomically rather than rewriting.
    // TODO: Have a dedicated function for doing the update rather than updating the file directly.
    // Something like with_config_update(|config| { config.default_station = name_maybe; }) that provides the current config and writes the modified config back.
    common_config_file.set_len(0)?;
    serde_json::to_writer_pretty(common_config_file, &extension_config)
        .with_context(|| "Failed to write extension config file as JSON.")?;
    Ok(())
}
