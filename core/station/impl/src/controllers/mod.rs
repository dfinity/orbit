//! Canister controller entrypoints.
//!
//! These entrypoints are used to handle the necessary business logic for the wallet canister and expose
//! the functionality to the clients.

mod system;
pub use system::*;

mod capabilities;
pub use capabilities::*;

mod account;
pub use account::*;

mod address_book;
pub use address_book::*;

mod notification;
pub use notification::*;

mod transfer;
pub use transfer::*;

mod proposal;
pub use proposal::*;

mod user;
pub use user::*;

mod proposal_policy;
pub use proposal_policy::*;

mod access_policy;
pub use access_policy::*;

mod user_group;
pub use user_group::*;

mod http;
pub use http::*;

#[cfg(test)]
mod tests {
    use ic_canister_core::api::*;
    use station_api::*;

    #[test]
    fn check_candid_interface() {
        use candid_parser::utils::{service_equal, CandidSource};

        candid::export_service!();
        let new_interface = __export_service();

        service_equal(
            CandidSource::Text(&new_interface),
            CandidSource::Text(include_str!("../../../api/spec.did")),
        )
        .unwrap();
    }
}
