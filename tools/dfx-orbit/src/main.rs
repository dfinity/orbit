//! # `dfx orbit` tool for managing digital assets.
//!
//! Note: This will initially be a standalone executable, but will be converted into a dfx extension once the dfx subcommand extension framework is well defined.
use dfx_orbit::{self as lib, DfxOrbitArgs};
use clap::Parser;

fn main() {
    let args = DfxOrbitArgs::parse();
    lib::main(args)
}
