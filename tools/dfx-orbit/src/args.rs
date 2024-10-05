//! Command line interface for `dfx-orbit`.
pub mod request;
pub mod verify;

use crate::{
    dfx::OrbitExtensionAgent, me::MeArgs, review::args::ReviewArgs, station::StationArgs,
    util::init_logger, DfxOrbit,
};
use clap::{Parser, Subcommand};
use request::RequestArgs;
use slog::trace;
use station_api::GetRequestInput;
use verify::VerifyArgs;

/// Manages Orbit on the Internet Computer.
#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct DfxOrbitArgs {
    /// Increase verbosity level
    #[clap(short, long, action = clap::ArgAction::Count, conflicts_with = "quiet")]
    pub(crate) verbose: u8,

    /// Reduce verbosity level
    #[clap(short, long, action = clap::ArgAction::Count, conflicts_with = "verbose")]
    pub(crate) quiet: u8,

    /// Name of the station to execute the command on. (Uses default station if unspecified)
    #[clap(short, long)]
    pub(crate) station: Option<String>,

    // TODO: Allow to specify --network, to overwrite the network specified by the station
    /// The user identity to run this command as
    #[clap(short, long)]
    pub(crate) identity: Option<String>,

    /// Manage Orbit stations.
    #[clap(subcommand)]
    pub(crate) command: DfxOrbitSubcommands,
}

/// CLI commands for managing Orbit on the Internet Computer.
#[derive(Debug, Subcommand)]
#[clap(version, about, long_about = None)]
pub enum DfxOrbitSubcommands {
    /// Manage Orbit stations.
    #[clap(subcommand)]
    Station(StationArgs),
    /// Make requests to Orbit
    Request(RequestArgs),
    /// Verify requests
    Verify(VerifyArgs),
    /// View and decide on requests.
    Review(ReviewArgs),
    /// Gets the caller's profile on an Orbit station.
    Me(MeArgs),
}

/// A command line tool for interacting with Orbit on the Internet Computer.
pub async fn exec(args: DfxOrbitArgs) -> anyhow::Result<()> {
    let logger = init_logger(args.verbose, args.quiet)?;
    trace!(logger, "Calling tool with arguments:\n{:#?}", args);

    let orbit_agent = OrbitExtensionAgent::new()?;

    // We don't need to instanciate a StationAgent to execute this command directly on the orbit agent
    if let DfxOrbitSubcommands::Station(station_args) = args.command {
        crate::station::exec(orbit_agent, station_args)?;
        return Ok(());
    };

    let config = match args.station {
        Some(station_name) => orbit_agent.station(&station_name)?,
        None => orbit_agent
            .default_station()?
            .ok_or_else(|| anyhow::format_err!("No default station specified"))?,
    };

    let dfx_orbit = DfxOrbit::new(orbit_agent, config, args.identity, logger).await?;

    match args.command {
        // Nicer display, json optional
        DfxOrbitSubcommands::Me(args) => {
            let ans = dfx_orbit.station.me().await?;
            if args.json {
                println!("{}", serde_json::to_string_pretty(&ans)?);
            } else {
                println!("{}", dfx_orbit.display_me(ans)?);
            }
            Ok(())
        }
        DfxOrbitSubcommands::Request(request_args) => {
            let request = dfx_orbit
                .station
                .request(request_args.into_request(&dfx_orbit).await?)
                .await?;
            dfx_orbit.print_create_request_info(&request);

            Ok(())
        }
        DfxOrbitSubcommands::Verify(verify_args) => {
            let request = dfx_orbit
                .station
                .review_id(GetRequestInput {
                    request_id: verify_args.request_id.clone(),
                })
                .await?;

            println!(
                "{}",
                dfx_orbit.display_get_request_response(request.clone())?
            );

            let verified = verify_args.verify(&dfx_orbit, &request).await;
            verify_args
                .conditionally_execute_actions(&dfx_orbit, verified)
                .await?;

            Ok(())
        }
        DfxOrbitSubcommands::Review(review_args) => dfx_orbit.exec_review(review_args).await,
        DfxOrbitSubcommands::Station(_) => unreachable!(),
    }
}
