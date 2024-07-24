use clap::{Parser, Subcommand};

/// Station management commands.
#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct AssetsArgs {
    /// The name of the asset canister targeted by this action
    #[clap(long, default_missing_value = "None")]
    canister: Option<String>,

    #[command(subcommand)]
    action: AssetsActionArgs,
}

#[derive(Debug, Clone, Subcommand)]
pub enum AssetsActionArgs {
    /// Upload assets to an asset canister
    Upload(AssetsUploadArgs),
}

#[derive(Debug, Clone, Parser)]
pub struct AssetsUploadArgs {
    /// The source directories to upload (multiple values possible)
    #[clap(long, num_args = 1..)]
    source: Vec<String>,
}
