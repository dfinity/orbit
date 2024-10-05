use crate::{
    asset::{RequestAssetArgs, VerifyAssetArgs},
    canister::{RequestCanisterArgs, VerifyCanisterArgs},
    dfx::OrbitExtensionAgent,
    me::MeArgs,
    permission::RequestPermissionArgs,
    review::args::ReviewArgs,
    station::StationArgs,
    util::init_logger,
    DfxOrbit,
};
use clap::{Parser, Subcommand};
use slog::trace;
use station_api::{CreateRequestInput, GetRequestInput, GetRequestResponse};

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

/// Request canister changes.
#[derive(Debug, Clone, Parser)]
#[clap(version, about, long_about = None)]
pub struct RequestArgs {
    /// Title of the request
    #[clap(long)]
    pub title: Option<String>,

    /// Summary of the request
    #[clap(long)]
    pub summary: Option<String>,

    #[clap(subcommand)]
    pub action: RequestArgsActions,
}

#[derive(Debug, Clone, Subcommand)]
#[clap(version, about, long_about = None)]
pub enum RequestArgsActions {
    /// Manage assets stored in an asset canister through Orbit
    Asset(RequestAssetArgs),
    /// Request canister operations through Orbit
    Canister(RequestCanisterArgs),
    /// Request permissions
    #[clap(subcommand)]
    Permission(RequestPermissionArgs),
}

impl RequestArgs {
    pub async fn into_request(self, dfx_orbit: &DfxOrbit) -> anyhow::Result<CreateRequestInput> {
        let operation = match self.action {
            RequestArgsActions::Canister(canister_args) => {
                canister_args.into_request(dfx_orbit).await?
            }
            RequestArgsActions::Asset(asset_args) => asset_args.into_request(dfx_orbit).await?,
            RequestArgsActions::Permission(permission_args) => {
                permission_args.into_request(dfx_orbit)?
            }
        };

        Ok(CreateRequestInput {
            operation,
            title: self.title,
            summary: self.summary,
            execution_plan: None,
        })
    }
}

#[derive(Debug, Clone, Parser)]
pub struct VerifyArgs {
    /// The ID of the request to verify
    pub request_id: String,

    /// Approve the request, if the validation succeeds
    #[clap(short = 'a', long)]
    pub and_approve: bool,
    /// Reject the request, if the validation fails
    #[clap(short = 'r', long)]
    pub or_reject: bool,

    /// The type of request to verify
    #[clap(subcommand)]
    pub action: VerifyArgsAction,
}

impl VerifyArgs {
    pub async fn verify(
        &self,
        dfx_orbit: &DfxOrbit,
        request: &GetRequestResponse,
    ) -> anyhow::Result<()> {
        // TODO: Don't allow non-pending requests to be verified, since they might no longer be
        // verifiable after the execution

        match &self.action {
            VerifyArgsAction::Asset(args) => args.verify(dfx_orbit, request).await?,
            VerifyArgsAction::Canister(args) => args.verify(dfx_orbit, request).await?,
        };

        Ok(())
    }

    pub(crate) async fn conditionally_execute_actions(
        &self,
        dfx_orbit: &DfxOrbit,
        verified: anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        match verified {
            Ok(()) => {
                if self.and_approve {
                    dfx_core::cli::ask_for_consent(
                        "Verification successful, approve the request?",
                    )?;
                    dfx_orbit
                        .station
                        .approve(self.request_id.clone(), None)
                        .await?;
                } else {
                    println!("Verification successful!");
                }
            }
            Err(err) => {
                if self.or_reject {
                    dfx_core::cli::ask_for_consent(&format!(
                        "Verification failed: {err}. Reject the request?"
                    ))?;
                    dfx_orbit
                        .station
                        .reject(self.request_id.clone(), None)
                        .await?;
                } else {
                    println!("Verification failed!");
                };

                return Err(err);
            }
        };

        Ok(())
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum VerifyArgsAction {
    /// Manage assets stored in an asset canister through Orbit
    Asset(VerifyAssetArgs),
    /// Request canister operations through Orbit
    Canister(VerifyCanisterArgs),
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
