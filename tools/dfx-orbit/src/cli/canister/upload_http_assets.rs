//! Implements the `dfx-orbit canister upload-http-assets` CLI command.
use crate::args::canister::UploadHttpAssets as Args;

/// The main entry point for the `dfx orbit` CLI.
pub async fn exec(args: Args) -> anyhow::Result<()> {
    let Args {
        canister: _canister,
        path: _path,
    } = args;
    let _station_agent = crate::orbit_station_agent::StationAgent::new()?;
    todo!()
}
