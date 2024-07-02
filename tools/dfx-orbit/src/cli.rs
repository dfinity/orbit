//! Implementation of the `dfx-orbit` commands.
pub mod canister;
pub mod dfx_extension_cli;
pub mod request;
pub mod station;

use crate::args::{DfxOrbitArgs, DfxOrbitSubcommands};
use anyhow::anyhow;

/// A command line tool for interacting with Orbit on the Internet Computer.
pub async fn exec(args: DfxOrbitArgs) -> anyhow::Result<()> {
    match args.command {
        DfxOrbitSubcommands::Me => {
            let station_principal = &crate::local_config::default_station()?
                .ok_or_else(|| anyhow!("No default station specified"))?
                .station_id;
            let ans = crate::dfx_extension_api::call_dfx_cli(vec![
                "canister",
                "call",
                station_principal,
                "me",
            ])?;
            print!("{ans}");
            Ok(())
        }
        DfxOrbitSubcommands::Station(station_args) => station::exec(station_args),
        DfxOrbitSubcommands::DfxExtension(dfx_extension_args) => {
            dfx_extension_cli::exec(dfx_extension_args)
        }
        DfxOrbitSubcommands::Canister(canister_args) => canister::exec(canister_args).await,
        DfxOrbitSubcommands::Request(request_args) => match request::exec(request_args).await {
            Ok(Ok(_response)) => Ok(()),
            Ok(Err(e)) => Err(anyhow!("Error response from the station: {e:?}")),
            Err(e) => Err(e),
        },
    }
}
