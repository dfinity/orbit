//! Canister services used to handle the necessary business logic for the Wallet canister.

mod account;
pub use account::*;

mod notification;
pub use notification::*;

mod transfer;
pub use transfer::*;

mod user;
pub use user::*;

mod proposal;
pub use proposal::*;

mod wallet;
pub use wallet::*;

mod upgrade;
pub use upgrade::*;
