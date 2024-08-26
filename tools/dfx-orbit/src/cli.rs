//! Implementation of the `dfx-orbit` commands.
pub(crate) mod asset;
pub(crate) mod station;

use station_api::{RequestApprovalStatusDTO, SubmitRequestApprovalInput};

use crate::{
    args::{review::ReviewArgs, DfxOrbitArgs, DfxOrbitSubcommands},
    dfx_extension_api::OrbitExtensionAgent,
    DfxOrbit,
};

/// A command line tool for interacting with Orbit on the Internet Computer.
pub async fn exec(args: DfxOrbitArgs) -> anyhow::Result<()> {
    let orbit_agent = OrbitExtensionAgent::new()?;

    // We don't need to instanciate a StationAgent to execute this command directly on the orbit agent
    if let DfxOrbitSubcommands::Station(station_args) = args.command {
        station::exec(orbit_agent, station_args)?;
        return Ok(());
    };

    let mut dfx_orbit = DfxOrbit::new(orbit_agent).await?;

    match args.command {
        DfxOrbitSubcommands::Me => {
            let ans = dfx_orbit.station.me().await?;
            println!("{}", serde_json::to_string_pretty(&ans)?);
            Ok(())
        }
        DfxOrbitSubcommands::Request(request_args) => {
            let request = dfx_orbit
                .station
                .request(request_args.into_create_request_input(&dfx_orbit)?)
                .await?;
            dfx_orbit.print_create_request_info(&request);

            Ok(())
        }
        DfxOrbitSubcommands::Review(review_args) => match review_args {
            ReviewArgs::List(args) => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(
                        &dfx_orbit.station.review_list(args.into()).await?
                    )?
                );

                Ok(())
            }
            ReviewArgs::Next(args) => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(
                        &dfx_orbit.station.review_next(args.into()).await?
                    )?
                );

                Ok(())
            }
            ReviewArgs::Id(args) => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(
                        &dfx_orbit.station.review_id(args.clone().into()).await?
                    )?
                );

                if let Ok(submit) = SubmitRequestApprovalInput::try_from(args) {
                    let action = match submit.decision {
                        RequestApprovalStatusDTO::Approved => "approve",
                        RequestApprovalStatusDTO::Rejected => "reject",
                    };
                    dfx_core::cli::ask_for_consent(&format!(
                        "Would you like to {action} this request?"
                    ))?;
                    dfx_orbit.station.submit(submit).await?;
                };

                Ok(())
            }
        },
        DfxOrbitSubcommands::Asset(asset_args) => {
            dfx_orbit.exec_asset(asset_args).await?;
            Ok(())
        }
        _ => unreachable!(),
    }
}
