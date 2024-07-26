use clap::{Parser, Subcommand};

/// Station management commands.
#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct AssetArgs {
    /// The name of the asset canister targeted by this action
    #[clap(long)]
    pub(crate) canister: String,

    #[command(subcommand)]
    pub(crate) action: AssetArgsAction,
}

#[derive(Debug, Clone, Subcommand)]
pub enum AssetArgsAction {
    /// Upload assets to an asset canister
    Upload(AssetUploadArgs),
}

#[derive(Debug, Clone, Parser)]
pub struct AssetUploadArgs {
    /// The source directories to upload (multiple values possible)
    #[clap(num_args = 1..)]
    pub(crate) files: Vec<String>,
}
