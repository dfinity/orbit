//! dfx-orbit station management commands.
pub mod config;

use clap::Subcommand;

/// Station management commands.
#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum Args {
    /// CLI subcommands for getting the local dfx configuration for this extension.
    #[command(subcommand)]
    Config(config::Args),
}
