//! Various error types for failure scenarios.

/// Error types for handling wallets.
mod wallet;
pub use wallet::*;

/// Error types to use across different modules.
mod core;
pub use core::*;
