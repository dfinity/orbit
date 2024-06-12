//! # `dfx orbit` tool for managing digital assets.
//!
//! Note: This will initially be a standalone executable, but will be converted into a dfx extension once the dfx subcommand extension framework is well defined.
use clap::Parser;
use dfx_orbit::{self as lib, args::DfxOrbitArgs};

fn main() {
    let args = DfxOrbitArgs::parse();
    lib::cli::main(args)
}
