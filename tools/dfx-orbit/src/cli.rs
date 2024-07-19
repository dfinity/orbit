//! Implementation of the `dfx-orbit` commands.
pub mod canister;
pub mod me;
pub mod request;
pub mod review;
pub mod station;

use crate::{
    args::{
        canister::CanisterArgs, request::CreateRequestArgs, review::ReviewArgs, DfxOrbitArgs,
        DfxOrbitSubcommands,
    },
    dfx_extension_api::OrbitExtensionAgent,
    StationAgent,
};
use anyhow::bail;

/// A command line tool for interacting with Orbit on the Internet Computer.
pub async fn exec(args: DfxOrbitArgs) -> anyhow::Result<()> {
    let orbit_agent = OrbitExtensionAgent::new()?;

    // We don't need to instanciate a StationAgent to execute this command directly on the orbit agent
    if let DfxOrbitSubcommands::Station(station_args) = args.command {
        station::exec(orbit_agent, station_args)?;
        return Ok(());
    };

    let mut station_agent = StationAgent::new(orbit_agent).await?;

    match args.command {
        DfxOrbitSubcommands::Me => {
            let ans = station_agent.me().await?;
            println!("{}", serde_json::to_string_pretty(&ans)?);
            Ok(())
        }
        DfxOrbitSubcommands::Canister(canister_args) => {
            match canister_args {
                CanisterArgs::Claim(claim_args) => {
                    station_agent
                        .claim_canister(claim_args.canister, claim_args.exclusive)
                        .await?;
                }
                CanisterArgs::UploadHttpAssets(upload_http_assets_args) => {
                    canister::upload_http_assets::exec(upload_http_assets_args).await?;
                }
            }
            Ok(())
        }
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
        DfxOrbitSubcommands::Review(review_args) => {
            let result_json = match review_args {
                ReviewArgs::List(args) => {
                    serde_json::to_string_pretty(&station_agent.review_list(args.into()).await?)?
                }
                ReviewArgs::Next(args) => {
                    serde_json::to_string_pretty(&station_agent.review_next(args.into()).await?)?
                }
                ReviewArgs::Id(args) => {
                    if args.approve.is_some() || args.reject.is_some() {
                        bail!("Approving or rejecting is currently unimplemented");
                    }

                    serde_json::to_string_pretty(&station_agent.review_id(args.into()).await?)?
                }
            };
            println!("{}", result_json);
            Ok(())
        }
        _ => unreachable!(),
    }
}
