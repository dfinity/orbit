//! Implementation of the `dfx-orbit` commands.
pub mod canister;
pub mod me;
pub mod request;
pub mod review;
pub mod station;

use crate::args::{DfxOrbitArgs, DfxOrbitSubcommands};
use anyhow::anyhow;

/// A command line tool for interacting with Orbit on the Internet Computer.
pub async fn exec(args: DfxOrbitArgs) -> anyhow::Result<()> {
    match args.command {
        DfxOrbitSubcommands::Me => me::exec().await,
        DfxOrbitSubcommands::Station(station_args) => station::exec(station_args),
        DfxOrbitSubcommands::Canister(canister_args) => canister::exec(canister_args).await,
        DfxOrbitSubcommands::Request(request_args) => match request::exec(request_args).await {
            Ok(Ok(_response)) => Ok(()),
            Ok(Err(e)) => Err(anyhow!("Error response from the station: {e:?}")),
            Err(e) => Err(e),
        },
        DfxOrbitSubcommands::Review(review_args) => review::exec(review_args).await,
    }
}
