//! Implementation of the `dfx-orbit` commands.

use crate::args::{DfxOrbitArgs, DfxOrbitSubcommands, dfx_extension_api};



pub fn main(args: DfxOrbitArgs) {
    println!("Hello args: {args:?}");

    match args.command {
        DfxOrbitSubcommands::Station(station_args) => {
            println!("Hello station args: {station_args:?}");
        }
        DfxOrbitSubcommands::DfxExtension(dfx_extension_args) => {
            println!("Hello dfx extension args: {dfx_extension_args:?}");
            match dfx_extension_args {
                dfx_extension_api::Args::Config(config_args) => {
                    match config_args {
                        dfx_extension_api::config::Args::Dir(_dir_args) => {
                            let extension_agent = crate::dfx_extension_api::DfxExtensionAgent::new("orbit");
                            let ans = extension_agent.extension_config_dir();
                            println!("{ans:?}");
                        }
                    }
                }
            }
        }
    }
}
