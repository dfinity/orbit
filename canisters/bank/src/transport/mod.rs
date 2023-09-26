//! Transport layer that defines the data transfer objects (DTOs) that are used to communicate
//! with the clients of the cank canister.

/// Bank details data transfer objects (DTOs).
mod bank_details;
pub use bank_details::*;

/// Common data transfer objects (DTOs) (e.g. errors, pagination, etc.)
mod common;
pub use common::*;

/// Management data transfer objects (DTOs) (e.g. canister init, upgrade, etc.)
mod management;
pub use management::*;
