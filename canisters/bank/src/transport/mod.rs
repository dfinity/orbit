//! Transport layer that defines the data transfer objects (DTOs) that are used to communicate
//! with the clients of the cank canister.

mod bank_details;
pub use bank_details::*;

mod common;
pub use common::*;

mod management;
pub use management::*;

mod notification;
pub use notification::*;

mod account;
pub use account::*;

mod transfer;
pub use transfer::*;

mod proposal;
pub use proposal::*;

mod user;
pub use user::*;
