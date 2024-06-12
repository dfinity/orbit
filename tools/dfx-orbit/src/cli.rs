//! Implementation of the `dfx-orbit` commands.
pub mod dfx_extension_cli;
pub mod station;

use crate::args::{DfxOrbitArgs, DfxOrbitSubcommands};

/// A command line tool for interacting with Orbit on the Internet Computer.
pub fn main(args: DfxOrbitArgs) {
    match args.command {
        DfxOrbitSubcommands::Station(station_args) => {
            station::main(station_args);
        }
        DfxOrbitSubcommands::DfxExtension(dfx_extension_args) => {
            dfx_extension_cli::main(dfx_extension_args);
        }
    }
}
