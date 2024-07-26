//! Implementation of the `dfx-orbit` commands.
mod asset;
mod canister;
mod me;
mod request;
mod review;
mod station;
mod submit;

pub use crate::cli::asset::AssetUploadRequest;
use crate::{
    args::{
        asset::AssetArgsAction, canister::CanisterArgs, review::ReviewArgs, DfxOrbitArgs,
        DfxOrbitSubcommands,
    },
    dfx_extension_api::OrbitExtensionAgent,
    StationAgent,
};

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
        DfxOrbitSubcommands::Review(review_args) => match review_args {
            ReviewArgs::List(args) => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&station_agent.review_list(args.into()).await?)?
                );

                Ok(())
            }
            ReviewArgs::Next(args) => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&station_agent.review_next(args.into()).await?)?
                );

                Ok(())
            }
            ReviewArgs::Id(args) => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(
                        &station_agent.review_id(args.clone().into()).await?
                    )?
                );

                // TODO: Reaffirm user consent before progressing with submitting
                if let Ok(submit) = args.try_into() {
                    station_agent.submit(submit).await?;
                };

                Ok(())
            }
        },
        DfxOrbitSubcommands::Asset(asset_args) => {
            match asset_args.action {
                AssetArgsAction::Upload(upload_args) => {
                    station_agent
                        .upload_assets(asset_args.canister, upload_args.files)
                        .await?;
                }
            }

            Ok(())
            //
        }
        _ => unreachable!(),
    }
}
