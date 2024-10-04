//! # `dfx orbit` tool for managing digital assets.
//!
//! Note: This will initially be a standalone executable, but will be converted into a dfx extension once the dfx subcommand extension framework is well defined.

use clap::Parser;
use dfx_orbit::{self as lib, args::DfxOrbitArgs};
use tokio::runtime::Builder;

fn main() {
    let args = DfxOrbitArgs::parse();
    //print!("Args: {}", args);
    let runtime = Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Unable to create a runtime");
    runtime.block_on(async {
        if let Err(err) = lib::cli::exec(args).await {
            println!("Failed to execute command: {}", err);
            std::process::exit(1);
        }
    });
}
