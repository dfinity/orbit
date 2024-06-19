//! `dfx` does not have an extension API, yet.  So imagine one existed and create a polyfill.  This is it.
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
