//! Implements the dfx extension CLI commands for managing stations.
use crate::args::station::{StationArgs};

/// Implements CLI commands for managing Orbit stations.
pub fn main(args: StationArgs) {
    match args {
        StationArgs::Add(add_args) => {
            todo!("Implement the `add` command for managing stations with args: {add_args:?}.");
        }
        StationArgs::List(list_args) => {
            todo!("Implement the `list` command for managing stations with args: {list_args:?}.");
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
