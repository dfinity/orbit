//! Transport layer that defines the data transfer objects (DTOs) that are used to communicate
//! with the clients of the control panel.

/// Account DTOs.
mod account;
pub use account::*;

/// Bank DTOs.
mod bank;
pub use bank::*;

/// Manage account DTOs.
mod manage_account;
pub use manage_account::*;

/// Manage bank DTOs.
mod manage_bank;
pub use manage_bank::*;

/// Account Identity DTOs.
mod account_identity;
pub use account_identity::*;

/// Canister hooks DTOs.
mod canister;
pub use canister::*;
