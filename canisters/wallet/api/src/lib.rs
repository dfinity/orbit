//! Transport layer that defines the data transfer objects (DTOs) that are used to communicate
//! with the clients of the wallet canister.

mod capabilities;
pub use capabilities::*;

mod address_book;
pub use address_book::*;

mod common;
pub use common::*;

mod system;
pub use system::*;

mod metadata;
pub use metadata::*;

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

mod change_canister;
pub use change_canister::*;

mod proposal_policy;
pub use proposal_policy::*;

mod access_policy;
pub use access_policy::*;

mod resource;
pub use resource::*;
