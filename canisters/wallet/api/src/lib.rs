//! Transport layer that defines the data transfer objects (DTOs) that are used to communicate
//! with the clients of the wallet canister.

mod wallet_details;
pub use wallet_details::*;

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

mod user_group;
pub use user_group::*;

mod upgrade;
pub use upgrade::*;

mod policy;
pub use policy::*;
