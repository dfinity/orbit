//! Storable models.
//!
//! This module contains the storable models for the Bank canister.

/// The wallet related models, which is used to store the wallet information.
mod wallet;
pub use wallet::*;

/// The blockchain related models, which is used to store represent a blockchain within the system.
mod blockchain;
pub use blockchain::*;

/// The blockchain standards related models, which is used to store represent a blockchain within the system.
mod blockchain_standard;
pub use blockchain_standard::*;

/// The account related models, which is used to represent a user account within the system.
mod account;
pub use account::*;

/// The wallet policy related models, which is used to represent wallet rules.
mod wallet_policy;
pub use wallet_policy::*;
