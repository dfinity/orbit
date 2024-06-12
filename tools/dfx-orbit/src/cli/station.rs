//! Implements the dfx extension CLI commands for managing stations.
use crate::args::station::StationArgs;
use crate::local_config;

/// Implements CLI commands for managing Orbit stations.
pub fn main(args: StationArgs) {
    match args {
        StationArgs::Add(add_args) => {
            local_config::add_station(&add_args)
                .expect("Failed to add station to local dfx config");
        }
        StationArgs::List(_list_args) => {
            let stations = local_config::list_stations();
            let json = serde_json::to_string_pretty(&stations)
                .expect("Failed to serialize list of stations");
            println!("{json}");
        }
        StationArgs::Show(show_args) => {
            let station = local_config::station(&show_args.name)
                .expect("Failed to get station from local dfx config");
            let json = serde_json::to_string_pretty(&station).expect("Failed to serialize station");
            println!("{json}");
        }
        StationArgs::Remove(remove_args) => {
            todo!(
                "Implement the `remove` command for managing stations with args: {remove_args:?}."
            );
        }
        StationArgs::Rename(rename_args) => {
            todo!(
                "Implement the `rename` command for managing stations with args: {rename_args:?}."
            );
        }
    }
}
