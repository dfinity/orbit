//! Implementation of the `dfx-orbit` commands.
pub mod dfx_extension_cli;

use crate::args::{DfxOrbitArgs, DfxOrbitSubcommands};

/// A command line tool for interacting with Orbit on the Internet Computer.
pub fn main(args: DfxOrbitArgs) {
    println!("Hello args: {args:?}");

    match args.command {
        DfxOrbitSubcommands::Station(station_args) => {
            println!("Hello station args: {station_args:?}");
        }
        DfxOrbitSubcommands::DfxExtension(dfx_extension_args) => {
            println!("Hello dfx extension args: {dfx_extension_args:?}");
            dfx_extension_cli::main(dfx_extension_args);
        }
    }
}
