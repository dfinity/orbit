//! # Wallet benchmarks
//!
//! The wallet benchmarks provide a comprehensive set of benchmarks for the wallet canister.

pub mod repositories;

#[ic_cdk_macros::init]
pub async fn init() {
    // Initialize the random number generator with a fixed seed to ensure deterministic
    // results across runs of the benchmarks.
    ic_canister_core::utils::initialize_rng_from_seed([0u8; 32]);
}
