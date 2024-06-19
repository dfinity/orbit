//! # `dfx orbit` tool for managing digital assets.
//!
//! Note: This will initially be a standalone executable, but will be converted into a dfx extension once the dfx subcommand extension framework is well defined.
use clap::Parser;
use dfx_orbit::{self as lib, args::DfxOrbitArgs};
use tokio::runtime::Runtime;

fn main() {
    let args = DfxOrbitArgs::parse();
    let runtime = Runtime::new().expect("Unable to create a runtime");
    runtime.block_on(async {
        lib::cli::exec(args).await.unwrap();
    });
}
