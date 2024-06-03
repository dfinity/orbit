//! Core common features to be used by IC Canisters.

mod lock;
pub use lock::*;

/// Utilities to interact with random numbers and UUIDs safely within the canister.
mod random;
pub use random::*;

/// Utilities to interact with time safely within the canister.
mod time;
pub use time::*;

/// String utils.
mod string;
pub use string::*;

/// Number utils.
mod number;
pub use number::*;

/// Hash utils.
mod hash;
pub use hash::*;
