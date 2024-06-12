//! dfx-orbit station management commands.
pub mod config;

use clap::Subcommand;

/// Station management commands.
#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum Args {
    #[command(subcommand)]
    Config(config::Args),
}
