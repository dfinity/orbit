//! Local dfx configuration of Orbit stations.
use crate::{dfx_extension_api::OrbitExtensionAgent, station_agent::StationConfig};
use anyhow::Context;
use serde::{Deserialize, Serialize};

/// Configuration that lives in e.g. ~/.config/dfx/orbit.json
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExtensionConfig {
    /// Default station name.
    pub default_station: Option<String>,
}

/// The directoy in the orbit dfx config directory where stations are stored.
pub const STATIONS_DIR: &str = "stations";

impl OrbitExtensionAgent {
    fn stations_dir(&self) -> anyhow::Result<cap_std::fs::Dir> {
        let config_dir = self.extension_config_dir()?;
        config_dir
            .create_dir_all(STATIONS_DIR)
            .with_context(|| "Failed to create stations directory")?;
        let stations_dir = config_dir
            .open_dir(STATIONS_DIR)
            .with_context(|| "Failed to open station directory")?;
        Ok(stations_dir)
    }

    /// The file in which the config for a particular station is stored.
    ///
    /// If the file does not exist, this will return an error.
    fn station_file(&self, name: &str) -> anyhow::Result<cap_std::fs::File> {
        self.open_station_file(name, false).with_context(|| {
            format!(
                "Failed to open station file for station '{name}':  Is the station name correct?"
            )
        })
    }

    /// Creates and returne file in which the config for a particular station is stored.
    ///
    /// If the file already exists, this will return an error.
    fn create_station_file(&self, name: &str) -> anyhow::Result<cap_std::fs::File> {
        self.open_station_file(name, true).with_context(|| {
            format!("Failed to create station file for station '{name}'.  Does it already exist?")
        })
    }

    /// The file in which the config for a particular station is stored.
    ///
    /// Optionally create the file if it does not exist.
    fn open_station_file(&self, name: &str, create_new: bool) -> anyhow::Result<cap_std::fs::File> {
        let basename = Self::station_file_name(name);
        let stations_dir = self.stations_dir()?;
        let mut open_options = cap_std::fs::OpenOptions::new();
        let open_options = open_options.read(true).write(true).create_new(create_new);
        let file = stations_dir.open_with(basename, open_options)?;
        Ok(file)
    }

    /// Lists all Orbit stations in the local dfx configuration.
    pub fn list_stations(&self) -> Vec<String> {
        // Get all entries in the station dir that are valid station configs.
        let stations_dir = self.stations_dir().expect("Failed to get stations dir");
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
            .filter(|station_name| self.station(station_name).is_ok())
            .collect()
    }

    /// Adds a new Orbit station to the local dfx configuration.
    ///
    /// If there is no default station, the new station is set as the default.
    // TODO: Check that the URL works & is the root URL.
    pub fn add_station<T>(&self, args: T) -> anyhow::Result<()>
    where
        T: Into<StationConfig>,
    {
        let station: StationConfig = args.into();
        let station_file = self.create_station_file(&station.name)?;
        station_file.set_len(0)?;
        serde_json::to_writer_pretty(station_file, &station).expect("Failed to write station file");

        if self.default_station_name()?.is_none() {
            self.set_default_station(Some(station.name.to_owned()))?;
        }

        Ok(())
    }

    /// Gets the local stored dfx configuration for a given station, or the default station if none is specified.
    pub fn station_or_default(&self, name: Option<&str>) -> anyhow::Result<StationConfig> {
        if let Some(name) = name {
            self.station(name)
        } else {
            let name = self
                .default_station()
                .with_context(|| "Station not specified and failed to get default.")?
                .with_context(|| "Station not specified and no default station set.")?
                .name;
            self.station(&name)
        }
    }

    /// Gets the local stored dfx configuration for a given station.
    pub fn station(&self, name: &str) -> anyhow::Result<StationConfig> {
        let station_file = self.station_file(name)?;
        let station: StationConfig = serde_json::from_reader(station_file)
            .with_context(|| "Failed to parse station file")?;
        Ok(station)
    }

    /// Removes an Orbit station from the local dfx configuration.
    pub fn remove_station(&self, name: &str) -> anyhow::Result<()> {
        let dir = self.stations_dir()?;
        let path = Self::station_file_name(name);
        dir.remove_file(path)
            .with_context(|| format!("Failed to remove dfx config file for station {}", name))?;

        if self.default_station_name()? == Some(name.to_string()) {
            self.set_default_station(None)?;
        }
        Ok(())
    }

    /// Renames an Orbit station in the local dfx configuration.
    ///
    /// If the station being renamed is the default station, the default is updated to reflect the new name.
    pub fn rename_station(&self, name: &str, new_name: &str) -> anyhow::Result<()> {
        let default_station_name = self.default_station_name()?;
        let mut station = self.station(name)?;
        station.name = new_name.to_string();
        self.add_station(station)?;
        self.remove_station(name)?;
        if default_station_name == Some(name.to_string()) {
            self.set_default_station(Some(new_name.to_string()))?;
        }
        Ok(())
    }

    /// Gets the common configuration for this dfx extension.
    ///
    /// If the config does not exist or is empty, default values are assumed.
    fn extension_config(&self) -> anyhow::Result<ExtensionConfig> {
        let common_config_file = self.extension_config_file()?;
        if common_config_file.metadata()?.len() == 0 {
            Ok(ExtensionConfig::default())
        } else {
            serde_json::from_reader(common_config_file)
                .with_context(|| "Failed to parse extension config file as JSON.")
        }
    }

    /// Gets the name of the default Orbit station from the local dfx configuration.
    pub fn default_station_name(&self) -> anyhow::Result<Option<String>> {
        Ok(self.extension_config()?.default_station)
    }

    /// Gets the default Orbit station from the local dfx configuration.
    pub fn default_station(&self) -> anyhow::Result<Option<StationConfig>> {
        if let Some(name) = &self.default_station_name()? {
            Ok(Some(self.station(name)?))
        } else {
            Ok(None)
        }
    }

    /// Sets the default Orbit station in the local dfx configuration.
    pub fn set_default_station(&self, name_maybe: Option<String>) -> anyhow::Result<()> {
        // Check if the station exists.
        if let Some(name) = &name_maybe {
            self.station(name)?;
        }
        // Set the default station.
        let mut extension_config = self.extension_config()?;
        extension_config.default_station = name_maybe;
        let common_config_file = self.extension_config_file()?;
        // TODO: Have a dedicated function for doing the update rather than updating the file directly.
        // Something like with_config_update(|config| { config.default_station = name_maybe; }) that provides the current config and writes the modified config back.
        common_config_file.set_len(0)?;
        serde_json::to_writer_pretty(common_config_file, &extension_config)
            .with_context(|| "Failed to write extension config file as JSON.")?;
        Ok(())
    }

    /// The name of the file in which the config for a given station is stored.
    fn station_file_name(name: &str) -> String {
        format!("{}.json", name)
    }
}
