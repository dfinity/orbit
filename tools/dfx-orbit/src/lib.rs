//! Library for interacting with Orbit on the Internet Computer.
use clap::Parser;

pub mod wallet;
use wallet::OrbitWalletArgs;

/// Manages Orbit on the Internet Computer.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct DfxOrbitArgs {
    /// Manage Orbit wallets.
    #[command(subcommand)]
    field: OrbitWalletArgs,
}

pub fn main(args: DfxOrbitArgs) {
    println!("Hello args: {args:?}");
}
