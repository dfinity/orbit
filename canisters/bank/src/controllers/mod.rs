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
