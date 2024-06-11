//! Library for interacting with Orbit on the Internet Computer.
use clap::Parser;

/// Manages Orbit on the Internet Computer.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct DfxOrbitArgs {
}

pub fn main(args: DfxOrbitArgs) {
    println!("Hello args: {args:?}");
}