//! Canister controller entrypoints.
//!
//! These entrypoints are used to handle the necessary business logic for the wallet canister and expose
//! the functionality to the clients.

mod wallet;
pub use wallet::*;

mod account;
pub use account::*;

mod notification;
pub use notification::*;

mod transfer;
pub use transfer::*;

mod proposal;
pub use proposal::*;

mod user;
pub use user::*;

mod user_group;
pub use user_group::*;

#[cfg(test)]
mod tests {
    use ic_canister_core::api::ApiResult;
    use wallet_api::*;

    #[test]
    fn check_candid_interface() {
        use candid::utils::{service_compatible, CandidSource};

        candid::export_service!();
        let new_interface = __export_service();

        service_compatible(
            CandidSource::Text(&new_interface),
            CandidSource::Text(include_str!("../../../api/spec.did")),
        )
        .unwrap();
    }
}
