//! Access to the DFX extension configuration.
use clap::Subcommand;

/// CLI subcommands for getting the local dfx configuration for this extension.
#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum Args {
    /// Gets the local configuration directory.
    Dir,
    /// Gets the local configuration file.
    File,
}
