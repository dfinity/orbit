//! Implements the `dfx-orbit canister *` CLI commands.
mod claim;
mod upload_http_assets;

use crate::args::canister::Args;

/// The main entry point for the `dfx orbit` CLI.
pub async fn exec(args: Args) -> anyhow::Result<()> {
    match args {
        Args::Claim(claim_args) => claim::exec(claim_args),
        Args::UploadHttpAssets(upload_http_assets_args) => {
            upload_http_assets::exec(upload_http_assets_args).await
        }
    }
}
