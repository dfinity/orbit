//! Core utility features for the control plane.

mod constants;
pub use constants::*;

mod memory;
pub use memory::*;

mod call_context;
pub use call_context::*;

mod config;
pub use config::*;

#[cfg(not(test))]
pub use ic_canister_core::cdk as ic_cdk;
#[cfg(test)]
pub use ic_canister_core::cdk::mocks as ic_cdk;
