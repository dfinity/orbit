//! Core utility features for the wallet canister.

mod config;
pub use config::*;

mod constants;
pub use constants::*;

mod memory;
pub use memory::*;

mod call_context;
pub use call_context::*;

mod processor;
pub use processor::*;

pub mod middlewares;

#[cfg(not(test))]
pub use ic_canister_core::cdk as ic_cdk;
#[cfg(test)]
pub use ic_canister_core::cdk::mocks as ic_cdk;

#[cfg(not(test))]
pub use ic_canister_core::utils::generate_uuid_v4;
#[cfg(test)]
pub async fn generate_uuid_v4() -> uuid::Uuid {
    uuid::Uuid::new_v4()
}
