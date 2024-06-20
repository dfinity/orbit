//! Implements the `dfx-orbit canister upload-http-assets` CLI command.
use crate::args::canister::UploadHttpAssets as Args;

/// The main entry point for the `dfx orbit` CLI.
pub async fn exec(args: Args) -> anyhow::Result<()> {
    let Args {
        canister,
        path: _path,
    } = args;
    let station_agent = crate::orbit_station_agent::StationAgent::new()?;
    let _canister_id = station_agent.canister_id(&canister)?;
    todo!()
}
