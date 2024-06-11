//! Library for interacting with Orbit on the Internet Computer.
pub mod args;
pub mod local_config;

pub use args::DfxOrbitArgs;

pub fn main(args: DfxOrbitArgs) {
    println!("Hello args: {args:?}");
}
