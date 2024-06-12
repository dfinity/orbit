//! Implements the dfx extension CLI commands for managing stations.
use crate::args::station::{ListResponse, StationArgs};
use crate::local_config;

/// Implements CLI commands for managing Orbit stations.
pub fn main(args: StationArgs) -> anyhow::Result<()> {
    match args {
        StationArgs::Add(add_args) => {
            local_config::add_station(&add_args)
                .expect("Failed to add station to local dfx config");
        }
        StationArgs::List(_list_args) => {
            let stations = local_config::list_stations();
            let ans = ListResponse { stations };
            // Note: The formatted ans is a sequence of complete lines, so an additional newline, as provided by println, is not needed.
            print!("{ans}");
        }
        StationArgs::Default => {
            let default_station = local_config::default_station_name()
                .expect("Failed to get default station from local dfx config");
            if let Some(station) = default_station {
                println!("{station}");
            }
        }
        StationArgs::Use(use_args) => {
            local_config::set_default_station(Some(use_args.name))
                .expect("Failed to set default station in local dfx config");
        }
        StationArgs::Show(show_args) => {
            let station = local_config::station(&show_args.name)
                .expect("Failed to get station from local dfx config");
            let json = serde_json::to_string_pretty(&station).expect("Failed to serialize station");
            println!("{json}");
        }
        StationArgs::Remove(remove_args) => {
            local_config::remove_station(&remove_args.name)
                .expect("Failed to remove station from local dfx config");
        }
        StationArgs::Rename(rename_args) => {
            local_config::rename_station(&rename_args.old, &rename_args.new)
                .expect("Failed to rename station in local dfx config");
        }
    }
    Ok(())
}
