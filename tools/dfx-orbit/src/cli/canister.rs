//! Implements the `dfx-orbit canister *` CLI commands.

use crate::args::canister::CanisterArgs;
mod claim;
mod upload_http_assets;

/// The main entry point for the `dfx orbit` CLI.
pub async fn exec(args: CanisterArgs) -> anyhow::Result<()> {
    match args {
        CanisterArgs::Claim(claim_args) => claim::exec(claim_args),
        CanisterArgs::UploadHttpAssets(upload_http_assets_args) => {
            upload_http_assets::exec(upload_http_assets_args).await
        }
    }
}
