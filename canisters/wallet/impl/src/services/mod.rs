//! Canister services used to handle the necessary business logic for the Wallet canister.

mod account;
pub use account::*;

mod address_book;
pub use address_book::*;

mod notification;
pub use notification::*;

mod transfer;
pub use transfer::*;

mod user;
pub use user::*;

mod user_group;
pub use user_group::*;

mod proposal;
pub use proposal::*;

mod wallet;
pub use wallet::*;

mod proposal_policy;
pub use proposal_policy::*;

mod change_canister;
pub use change_canister::*;

pub mod access_policy;
