pub mod asset;

use asset::VerifyAssetArgs;
use clap::{Parser, Subcommand};

/// Station management commands.
#[derive(Debug, Clone, Parser)]
#[clap(version, about, long_about = None)]
pub struct VerifyArgs {
    /// The ID of the request to verify
    pub(crate) request_id: String,

    // TODO: Auto approve / reject
    /// The type of request to verify
    #[clap(subcommand)]
    pub(crate) action: VerifyArgsAction,
}

#[derive(Debug, Clone, Subcommand)]
pub enum VerifyArgsAction {
    /// Verify orbit actions with regards to assets
    Asset(VerifyAssetArgs),
}
