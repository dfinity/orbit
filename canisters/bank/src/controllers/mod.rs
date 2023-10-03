//! Canister controller entrypoints.

/// Management entrypoints, enabling the owners to manage the canister and clients to fetch details about the canister.
mod management;
pub use management::*;

/// Wallet entrypoints, enabling the clients to manage their wallets.
mod wallet;
pub use wallet::*;

/// Transfer entrypoints, enabling the clients to transfer assets.
mod transfer;
pub use transfer::*;

/// Operation entrypoints, enabling the clients to take action upon bank operations.
///
/// This includes approving, rejecting, and cancelling operations.
///
/// Operations are mostly related to transfers, but can also be used for other actions.
mod operation;
pub use operation::*;

/// Account entrypoints, enabling the clients to manage their accounts.
/// 
/// This includes creating, updating, removing and confirming accounts.
mod account;
pub use account::*;
