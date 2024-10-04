//! Implements the dfx extension CLI commands for managing stations.
use crate::{
    args::station::{ListResponse, StationArgs},
    dfx::OrbitExtensionAgent,
};
use anyhow::Context;

/// Implements CLI commands for managing Orbit stations.
pub fn exec(orbit_agent: OrbitExtensionAgent, args: StationArgs) -> anyhow::Result<()> {
    match args {
        StationArgs::Add(add_args) => {
            orbit_agent
                .add_station(add_args)
                .with_context(|| "Failed to add station to local dfx config")?;
        }
        StationArgs::List(_list_args) => {
            let stations = orbit_agent.list_stations()?;
            let ans = ListResponse { stations };
            // Note: The formatted ans is a sequence of complete lines, so an additional newline, as provided by println, is not needed.
            print!("{ans}");
        }
        StationArgs::Default => {
            let default_station = orbit_agent
                .default_station_name()
                .with_context(|| "Failed to get default station from local dfx config")?;
            if let Some(station) = default_station {
                println!("{station}");
            }
        }
        StationArgs::Use(use_args) => {
            orbit_agent
                .set_default_station(Some(use_args.name))
                .with_context(|| "Failed to set default station in local dfx config")?;
        }
        StationArgs::Show(show_args) => {
            let station = orbit_agent
                .station_or_default(&show_args.name)
                .with_context(|| "Failed to get station from local dfx config")?;
            if show_args.json {
                let json = serde_json::to_string_pretty(&station)
                    .with_context(|| "Failed to serialize station")?;
                println!("{json}");
            } else {
                println!("{}", station);
            }
        }
        StationArgs::Remove(remove_args) => {
            orbit_agent
                .remove_station(&remove_args.name)
                .with_context(|| "Failed to remove station from local dfx config")?;
        }
        StationArgs::Edit(rename_args) => {
            orbit_agent
                .edit_station(
                    &rename_args.station,
                    rename_args.rename,
                    rename_args.station_id,
                    rename_args.network,
                    rename_args.url,
                )
                .with_context(|| "Failed to rename station in local dfx config")?;
        }
    }
    Ok(())
}
