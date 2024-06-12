//! Access to the DFX extension configuration.
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum Args {
    Dir(Dir),
    File,
}

#[derive(Debug, Parser)]
pub struct Dir {}
