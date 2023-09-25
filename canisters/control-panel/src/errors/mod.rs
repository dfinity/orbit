//! Various error types for failure scenarios.

/// Error types for the account identity repository.
mod account_identity_repository_error;
pub use account_identity_repository_error::*;

/// Error types for the account bank repository.
mod account_bank_repository_error;
pub use account_bank_repository_error::*;

/// Error types for the account repository.
mod account_repository_error;
pub use account_repository_error::*;

/// Error types for the account management.
mod account_management_error;
pub use account_management_error::*;
