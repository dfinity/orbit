//! Implementation of the `dfx-orbit` commands.
pub mod canister;
pub mod me;
pub mod request;
pub mod review;
pub mod station;

use crate::{
    args::{request::CreateRequestArgs, DfxOrbitArgs, DfxOrbitSubcommands},
    StationAgent,
};

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
            println!("{}", serde_json::to_string_pretty(&ans)?);
            Ok(())
        }
        DfxOrbitSubcommands::Canister(canister_args) => canister::exec(canister_args).await,
        DfxOrbitSubcommands::Request(request_args) => {
            let response = station_agent
                .request(request_args.into_create_request_input(&station_agent)?)
                .await?;
            let request_id = &response.request.id;
            let request_url = station_agent.request_url(request_id);
            println!("Created request: {request_id}");
            println!("Request URL: {request_url}");
            println!("To view the request, run: dfx-orbit review id {request_id}");
            Ok(())
        }
        DfxOrbitSubcommands::Review(review_args) => station_agent.review(review_args).await,
        _ => unreachable!(),
    }
}
