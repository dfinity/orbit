//! Various error types for failure scenarios.

/// Error types for handling wallets.
mod wallet;
pub use wallet::*;

/// Error types to use across different mappers.
mod mapper;
pub use mapper::*;

/// Error types to use across different factories.
mod factory;
pub use factory::*;

/// Error types to use across different blockchain apis.
mod blockchain_api;
pub use blockchain_api::*;
