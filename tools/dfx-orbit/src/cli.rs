//! Implementation of the `dfx-orbit` commands.
pub mod canister;
pub mod me;
pub mod request;
pub mod review;
pub mod station;

use crate::{
    args::{DfxOrbitArgs, DfxOrbitSubcommands},
    StationAgent,
};
use anyhow::{anyhow, Context};

/// A command line tool for interacting with Orbit on the Internet Computer.
pub async fn exec(args: DfxOrbitArgs) -> anyhow::Result<()> {
    // We don't need to instanciate a StationAgent to execute this command
    if let DfxOrbitSubcommands::Station(station_args) = args.command {
        station::exec(station_args)?;
        return Ok(());
    };

    let mut station_agent = StationAgent::new()?;

    match args.command {
        DfxOrbitSubcommands::Me => {
            let ans = station_agent.me().await?;
            println!(
                "{}",
                serde_json::to_string_pretty(&ans)
                    .with_context(|| "Failed to serialize response as JSON")?
            );
            Ok(())
        }
        DfxOrbitSubcommands::Canister(canister_args) => canister::exec(canister_args).await,
        DfxOrbitSubcommands::Request(request_args) => match request::exec(request_args).await {
            Ok(Ok(_response)) => Ok(()),
            Ok(Err(e)) => Err(anyhow!("Error response from the station: {e:?}")),
            Err(e) => Err(e),
        },
        DfxOrbitSubcommands::Review(review_args) => station_agent.review(review_args).await,
        _ => unreachable!(),
    }
}
