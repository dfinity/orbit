//! Implementation of the `dfx-orbit` commands.
pub(crate) mod asset;
pub(crate) mod review;
pub(crate) mod station;

use crate::{
    args::{DfxOrbitArgs, DfxOrbitSubcommands},
    dfx_extension_api::OrbitExtensionAgent,
    DfxOrbit,
};
use slog::trace;

/// A command line tool for interacting with Orbit on the Internet Computer.
pub async fn exec(args: DfxOrbitArgs) -> anyhow::Result<()> {
    let logger = init_logger(args.verbose, args.quiet);
    trace!(logger, "Calling tool with arguments:\n{:#?}", args);

    let orbit_agent = OrbitExtensionAgent::new()?;

    // We don't need to instanciate a StationAgent to execute this command directly on the orbit agent
    if let DfxOrbitSubcommands::Station(station_args) = args.command {
        station::exec(orbit_agent, station_args)?;
        return Ok(());
    };

    let config = match args.station {
        Some(station_name) => orbit_agent.station(&station_name)?,
        None => orbit_agent
            .default_station()?
            .ok_or_else(|| anyhow::format_err!("No default station specified"))?,
    };

    let mut dfx_orbit = DfxOrbit::new(orbit_agent, config, logger).await?;

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
        DfxOrbitSubcommands::Review(review_args) => dfx_orbit.exec_review(review_args).await,
        DfxOrbitSubcommands::Asset(asset_args) => {
            dfx_orbit.exec_asset(asset_args).await?;
            Ok(())
        }
        _ => unreachable!(),
    }
}

/// Initalize the logger
///
/// Default log level is WARN, can be turned up to TRCE by adding -v flags
/// and down to CRIT by adding -q flags
fn init_logger(verbose: u8, quiet: u8) -> slog::Logger {
    use slog::Drain;

    let verbose = verbose.clamp(0, 3);
    let quiet = quiet.clamp(0, 2);
    let level = 3 + verbose - quiet;
    let level = slog::Level::from_usize(level as usize).unwrap();

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator)
        .build()
        .filter_level(level)
        .fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(drain, slog::o!())
}
