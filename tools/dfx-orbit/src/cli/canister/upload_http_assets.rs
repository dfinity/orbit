//! Implements the `dfx-orbit canister upload-http-assets` CLI command.
use std::path::PathBuf;

use ic_utils::canister::CanisterBuilder;
use walkdir::WalkDir;

use crate::args::canister::UploadHttpAssets as Args;

/// The main entry point for the `dfx orbit` CLI.
pub async fn exec(args: Args) -> anyhow::Result<()> {
    let Args {
        canister,
        path,
        verbose,
    } = args;
    if verbose {
        println!("Uploading assets to canister: {}", canister);
        for asset in list_assets(&path){
            println!("Uploading asset: {}", asset.display());
        }
    }
    let mut station_agent = crate::orbit_station_agent::StationAgent::new()?;
    let canister_id = station_agent.canister_id(&canister)?;
    let _canister_agent = CanisterBuilder::new()
        .with_agent(station_agent.dfx.agent().await?)
        .with_canister_id(canister_id)
        .build()?;
    todo!("Still need to upload the assets")
}

/// Lists all the files at the given path.
///
/// - Links are followed.
/// - Only files are returned.
/// - The files are sorted by name.
/// - Any files that cannot be read are ignored.
/// - The path includes the prefix.
fn list_assets(path: &str) -> Vec<PathBuf> {
    WalkDir::new(path)
        .sort_by_file_name()
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.into_path())
        .collect()
}
